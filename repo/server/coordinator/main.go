// #region 🧲Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// GraphQL server for the monorepo management API.

// #endregion 🧲Header

// #region 🔑Package
// Package declaration for the repo server binary. MUST be package main.
package main

// #endregion 🔑Package

// #region 🔌Adapters
// Standard library and third-party imports MUST be grouped by origin.
import (
	"bufio"
	"context"
	"crypto/hmac"
	"crypto/sha256"
	"database/sql"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"log"
	"math/rand"
	"net/http"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strconv"
	"strings"
	"sync"
	"time"

	repopkg "github.com/usalu/semio/repo/go"
	_ "modernc.org/sqlite"
)

// #endregion 🔌Adapters

// #region ⏱️Config
// Server configuration loading from environment variables. MUST provide sensible defaults.

// ⚙️Config holds all server configuration values.
type Config struct {
	Address          string
	DatabasePath     string
	RepoRoot         string
	Token            string
	GitHubSecret     string
	DiscordWebhook   string
	RequestBodyLimit int64
}

// 🖥️loadConfig reads server configuration from environment variables with fallback defaults.
func loadConfig() Config {
	cwd, _ := os.Getwd()
	return Config{
		Address:          envOrDefault("SEMIO_SERVER_ADDR", "127.0.0.1:8787"),
		DatabasePath:     envOrDefault("SEMIO_SERVER_DB", "semio-server.db"),
		RepoRoot:         envOrDefault("SEMIO_SERVER_REPO_ROOT", cwd),
		Token:            envOrDefault("SEMIO_SERVER_TOKEN", ""),
		GitHubSecret:     envOrDefault("SEMIO_SERVER_GITHUB_SECRET", ""),
		DiscordWebhook:   envOrDefault("SEMIO_SERVER_DISCORD_WEBHOOK", ""),
		RequestBodyLimit: envOrDefaultInt64("SEMIO_SERVER_BODY_LIMIT", 10*1024*1024),
	}
}

// 📦envOrDefault returns the environment variable value or the fallback if empty.
func envOrDefault(key, fallback string) string {
	if value := strings.TrimSpace(os.Getenv(key)); value != "" {
		return value
	}
	return fallback
}

// 🔬envOrDefaultInt64 returns the parsed int64 environment variable or the fallback.
func envOrDefaultInt64(key string, fallback int64) int64 {
	if value := strings.TrimSpace(os.Getenv(key)); value != "" {
		if parsed, err := strconv.ParseInt(value, 10, 64); err == nil {
			return parsed
		}
	}
	return fallback
}

// #endregion ⏱️Config

// #region 🎺Models
// Data model types for tickets, scopes, warnings, breachs, events, and API request/response payloads. MUST mirror the server SQLite schema.

// 🎫Ticket represents a tracked work item with lifecycle status.
type Ticket struct {
	ID        string     `json:"id"`
	Status    string     `json:"status"`
	Title     string     `json:"title"`
	Emoji     string     `json:"emoji"`
	Prompt    string     `json:"prompt"`
	Summary   string     `json:"summary"`
	LLM       string     `json:"llm"`
	Client    string     `json:"client"`
	Author    string     `json:"author"`
	GitHub    string     `json:"github_issue"`
	CreatedAt time.Time  `json:"created_at"`
	ClosedAt  *time.Time `json:"closed_at"`
}

// 📖Scope represents a code region (file, section, or definition) with line range.
type Scope struct {
	ID          string    `json:"id"`
	Kind        string    `json:"kind"`
	FilePath    string    `json:"file_path"`
	SectionPath string    `json:"section_path"`
	Definition  string    `json:"definition_name"`
	StartLine   int       `json:"start_line"`
	EndLine     int       `json:"end_line"`
	UpdatedAt   time.Time `json:"updated_at"`
}

// 🔭Warning represents a detected issue such as a scope conflict between tickets.
type Warning struct {
	ID             string     `json:"id"`
	Kind           string     `json:"kind"`
	Severity       string     `json:"severity"`
	Message        string     `json:"message"`
	TicketID       string     `json:"ticket_id"`
	ScopeID        string     `json:"scope_id"`
	CreatedAt      time.Time  `json:"created_at"`
	Acknowledged   *time.Time `json:"acknowledged_at"`
	AcknowledgedBy string     `json:"ack_by"`
}

// 📜Breach represents a policy breach detected in source code.
type Breach struct {
	ID         string     `json:"id"`
	Kind       string     `json:"kind"`
	Priority   string     `json:"priority"`
	ScopeID    string     `json:"scope_id"`
	FilePath   string     `json:"file_path"`
	Line       *int       `json:"line"`
	Column     *int       `json:"column"`
	Summary    string     `json:"summary"`
	Excerpt    string     `json:"excerpt"`
	Autofix    bool       `json:"autofixable"`
	DetectedAt time.Time  `json:"detected_at"`
	TicketID   string     `json:"ticket_id"`
	ResolvedAt *time.Time `json:"resolved_at"`
}

// 📡Event represents a system event persisted to the event log.
type Event struct {
	ID        string    `json:"id"`
	Type      string    `json:"type"`
	Source    string    `json:"source"`
	Payload   string    `json:"payload_json"`
	CreatedAt time.Time `json:"created_at"`
}

// 🔢LineRange represents a contiguous range of line numbers.
type LineRange struct {
	Start int
	End   int
}

// 🔷DiffHunk represents a single hunk with old and new line ranges from a unified diff.
type DiffHunk struct {
	OldRange LineRange
	NewRange LineRange
}

// 📍DiffFile represents a single file entry in a unified diff with its hunks.
type DiffFile struct {
	Path    string
	Hunks   []DiffHunk
	Deleted bool
	Created bool
}

// 🔬DiffResult aggregates all parsed diff files from a patch.
type DiffResult struct {
	Files []DiffFile
}

// 📸FileSnapshot holds the full content of a file for snapshot-based indexing.
type FileSnapshot struct {
	Path    string `json:"path"`
	Content string `json:"content"`
}

// 📦TicketOpenRequest is the JSON payload for opening a new ticket.
type TicketOpenRequest struct {
	TicketID    string `json:"ticket_id"`
	Title       string `json:"title"`
	Prompt      string `json:"prompt"`
	LLM         string `json:"llm"`
	Client      string `json:"client"`
	Author      string `json:"author"`
	GitHubIssue string `json:"github_issue"`
}

// 📨TicketCloseRequest is the JSON payload for closing a ticket.
type TicketCloseRequest struct {
	TicketID string   `json:"ticket_id"`
	Summary  string   `json:"summary"`
	Files    []string `json:"files"`
}

// 🔓TicketReopenRequest is the JSON payload for reopening a closed ticket.
type TicketReopenRequest struct {
	TicketID string `json:"ticket_id"`
	Prompt   string `json:"prompt"`
	LLM      string `json:"llm"`
	Title    string `json:"title"`
}

// 📋DiffIngestRequest is the JSON payload for ingesting a diff patch.
type DiffIngestRequest struct {
	TicketID  string         `json:"ticket_id"`
	RepoID    string         `json:"repo_id"`
	Patch     string         `json:"patch"`
	Snapshots []FileSnapshot `json:"snapshots"`
}

// 📩DiffIngestResponse holds the results of a diff ingestion operation.
type DiffIngestResponse struct {
	ChangedFiles  []string  `json:"changed_files"`
	ClaimedScopes []string  `json:"claimed_scopes"`
	Warnings      []Warning `json:"warnings"`
	Breachs       []Breach  `json:"breachs"`
	Blockers      []string  `json:"blockers"`
}

// 📄IndexFileRequest is the JSON payload for indexing a single file.
type IndexFileRequest struct {
	FilePath string `json:"file_path"`
	Content  string `json:"content"`
}

// #endregion 🎺Models

// #region 🔷Database
// SQLite database layer for persistent storage of tickets, scopes, claims, warnings, breachs, and events. MUST use WAL journal mode.

// 🔌Database wraps a sql.DB connection to the SQLite store.
type Database struct {
	db *sql.DB
}

// 🗄️openDatabase opens an SQLite database and runs schema migrations.
// ⏹️MUST enable WAL journal mode and foreign keys.
func openDatabase(path string) (*Database, error) {
	db, err := sql.Open("sqlite", path)
	if err != nil {
		return nil, err
	}
	if _, err := db.Exec("PRAGMA journal_mode=WAL"); err != nil {
		return nil, err
	}
	if _, err := db.Exec("PRAGMA foreign_keys=ON"); err != nil {
		return nil, err
	}
	store := &Database{db: db}
	if err := store.migrate(); err != nil {
		return nil, err
	}
	return store, nil
}

// 🆕migrate creates database tables if they do not already exist.
func (d *Database) migrate() error {
	statements := []string{
		"CREATE TABLE IF NOT EXISTS repos (id TEXT PRIMARY KEY, name TEXT, path TEXT, created_at DATETIME)",
		"CREATE TABLE IF NOT EXISTS tickets (id TEXT PRIMARY KEY, status TEXT, title TEXT, emoji TEXT, prompt TEXT, summary TEXT, llm TEXT, ui TEXT, author TEXT, github_issue TEXT, created_at DATETIME, closed_at DATETIME)",
		"CREATE TABLE IF NOT EXISTS scopes (id TEXT PRIMARY KEY, kind TEXT, file_path TEXT, section_path TEXT, definition_name TEXT, start_line INT, end_line INT, updated_at DATETIME)",
		"CREATE TABLE IF NOT EXISTS ticket_claims (ticket_id TEXT, scope_id TEXT, claim_type TEXT, first_seen_at DATETIME, last_seen_at DATETIME, PRIMARY KEY (ticket_id, scope_id))",
		"CREATE TABLE IF NOT EXISTS breachs (id TEXT PRIMARY KEY, kind TEXT, priority TEXT, scope_id TEXT, file_path TEXT, line INT, column INT, summary TEXT, excerpt TEXT, autofixable BOOL, detected_at DATETIME, ticket_id TEXT, resolved_at DATETIME)",
		"CREATE TABLE IF NOT EXISTS warnings (id TEXT PRIMARY KEY, kind TEXT, severity TEXT, message TEXT, ticket_id TEXT, scope_id TEXT, created_at DATETIME, acknowledged_at DATETIME, ack_by TEXT)",
		"CREATE TABLE IF NOT EXISTS events (id TEXT PRIMARY KEY, type TEXT, source TEXT, payload_json TEXT, created_at DATETIME)",
		"CREATE TABLE IF NOT EXISTS contributor_work (github TEXT, kind TEXT, item_id TEXT, PRIMARY KEY (github, kind, item_id))",
	}
	for _, stmt := range statements {
		if _, err := d.db.Exec(stmt); err != nil {
			return err
		}
	}
	return nil
}

// 📪Close closes the underlying SQL database connection.
// 📪MUST release all database resources.
func (d *Database) Close() error {
	return d.db.Close()
}

// 💿insertEvent persists a new event record.
func (d *Database) insertEvent(ctx context.Context, event Event) error {
	_, err := d.db.ExecContext(ctx, "INSERT INTO events (id, type, source, payload_json, created_at) VALUES (?, ?, ?, ?, ?)", event.ID, event.Type, event.Source, event.Payload, event.CreatedAt.UTC())
	return err
}

// 🎫upsertTicket inserts or updates a ticket record.
func (d *Database) upsertTicket(ctx context.Context, ticket Ticket) error {
	_, err := d.db.ExecContext(ctx, "INSERT INTO tickets (id, status, title, emoji, prompt, summary, llm, ui, author, github_issue, created_at, closed_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) ON CONFLICT(id) DO UPDATE SET status=excluded.status, title=excluded.title, emoji=excluded.emoji, prompt=excluded.prompt, summary=excluded.summary, llm=excluded.llm, ui=excluded.ui, author=excluded.author, github_issue=excluded.github_issue, closed_at=excluded.closed_at", ticket.ID, ticket.Status, ticket.Title, ticket.Emoji, ticket.Prompt, ticket.Summary, ticket.LLM, ticket.Client, ticket.Author, ticket.GitHub, ticket.CreatedAt.UTC(), ticket.ClosedAt)
	return err
}

// 🧹listTickets queries tickets optionally filtered by status.
func (d *Database) listTickets(ctx context.Context, status string) ([]Ticket, error) {
	query := "SELECT id, status, title, emoji, prompt, summary, llm, ui, author, github_issue, created_at, closed_at FROM tickets"
	args := []interface{}{}
	if status != "" {
		query += " WHERE status = ?"
		args = append(args, status)
	}
	rows, err := d.db.QueryContext(ctx, query, args...)
	if err != nil {
		return nil, err
	}
	defer rows.Close()
	var tickets []Ticket
	for rows.Next() {
		var ticket Ticket
		var closedAt sql.NullTime
		if err := rows.Scan(&ticket.ID, &ticket.Status, &ticket.Title, &ticket.Emoji, &ticket.Prompt, &ticket.Summary, &ticket.LLM, &ticket.Client, &ticket.Author, &ticket.GitHub, &ticket.CreatedAt, &closedAt); err != nil {
			return nil, err
		}
		if closedAt.Valid {
			ticket.ClosedAt = &closedAt.Time
		}
		tickets = append(tickets, ticket)
	}
	return tickets, nil
}

// 🔷getTicket retrieves a single ticket by ID.
func (d *Database) getTicket(ctx context.Context, ticketID string) (*Ticket, error) {
	row := d.db.QueryRowContext(ctx, "SELECT id, status, title, emoji, prompt, summary, llm, ui, author, github_issue, created_at, closed_at FROM tickets WHERE id = ?", ticketID)
	var ticket Ticket
	var closedAt sql.NullTime
	if err := row.Scan(&ticket.ID, &ticket.Status, &ticket.Title, &ticket.Emoji, &ticket.Prompt, &ticket.Summary, &ticket.LLM, &ticket.Client, &ticket.Author, &ticket.GitHub, &ticket.CreatedAt, &closedAt); err != nil {
		return nil, err
	}
	if closedAt.Valid {
		ticket.ClosedAt = &closedAt.Time
	}
	return &ticket, nil
}

// 🗑️replaceScopes deletes existing scopes for the file and inserts the new ones.
func (d *Database) replaceScopes(ctx context.Context, filePath string, scopes []Scope) error {
	if _, err := d.db.ExecContext(ctx, "DELETE FROM scopes WHERE file_path = ?", filePath); err != nil {
		return err
	}
	for _, scope := range scopes {
		if _, err := d.db.ExecContext(ctx, "INSERT INTO scopes (id, kind, file_path, section_path, definition_name, start_line, end_line, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)", scope.ID, scope.Kind, scope.FilePath, scope.SectionPath, scope.Definition, scope.StartLine, scope.EndLine, scope.UpdatedAt.UTC()); err != nil {
			return err
		}
	}
	return nil
}

// 🔭listScopesByFile retrieves all scopes for a given file path.
func (d *Database) listScopesByFile(ctx context.Context, filePath string) ([]Scope, error) {
	rows, err := d.db.QueryContext(ctx, "SELECT id, kind, file_path, section_path, definition_name, start_line, end_line, updated_at FROM scopes WHERE file_path = ?", filePath)
	if err != nil {
		return nil, err
	}
	defer rows.Close()
	var scopes []Scope
	for rows.Next() {
		var scope Scope
		if err := rows.Scan(&scope.ID, &scope.Kind, &scope.FilePath, &scope.SectionPath, &scope.Definition, &scope.StartLine, &scope.EndLine, &scope.UpdatedAt); err != nil {
			return nil, err
		}
		scopes = append(scopes, scope)
	}
	return scopes, nil
}

// 🔁upsertClaim inserts or updates a ticket-scope claim record.
func (d *Database) upsertClaim(ctx context.Context, ticketID string, scopeID string, claimType string, now time.Time) error {
	_, err := d.db.ExecContext(ctx, "INSERT INTO ticket_claims (ticket_id, scope_id, claim_type, first_seen_at, last_seen_at) VALUES (?, ?, ?, ?, ?) ON CONFLICT(ticket_id, scope_id) DO UPDATE SET claim_type=excluded.claim_type, last_seen_at=excluded.last_seen_at", ticketID, scopeID, claimType, now.UTC(), now.UTC())
	return err
}

// 📋listClaimsByTicket retrieves all scopes claimed by a ticket.
func (d *Database) listClaimsByTicket(ctx context.Context, ticketID string) ([]Scope, error) {
	rows, err := d.db.QueryContext(ctx, "SELECT scopes.id, scopes.kind, scopes.file_path, scopes.section_path, scopes.definition_name, scopes.start_line, scopes.end_line, scopes.updated_at FROM scopes JOIN ticket_claims ON scopes.id = ticket_claims.scope_id WHERE ticket_claims.ticket_id = ?", ticketID)
	if err != nil {
		return nil, err
	}
	defer rows.Close()
	var scopes []Scope
	for rows.Next() {
		var scope Scope
		if err := rows.Scan(&scope.ID, &scope.Kind, &scope.FilePath, &scope.SectionPath, &scope.Definition, &scope.StartLine, &scope.EndLine, &scope.UpdatedAt); err != nil {
			return nil, err
		}
		scopes = append(scopes, scope)
	}
	return scopes, nil
}

// ➖replaceWarnings removes conflict warnings and inserts the new set.
func (d *Database) replaceWarnings(ctx context.Context, warnings []Warning) error {
	if _, err := d.db.ExecContext(ctx, "DELETE FROM warnings WHERE kind = ?", "conflict"); err != nil {
		return err
	}
	for _, warning := range warnings {
		if _, err := d.db.ExecContext(ctx, "INSERT INTO warnings (id, kind, severity, message, ticket_id, scope_id, created_at, acknowledged_at, ack_by) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)", warning.ID, warning.Kind, warning.Severity, warning.Message, warning.TicketID, warning.ScopeID, warning.CreatedAt.UTC(), warning.Acknowledged, warning.AcknowledgedBy); err != nil {
			return err
		}
	}
	return nil
}

// ⚠️listWarnings retrieves warnings optionally filtered by ticket ID.
func (d *Database) listWarnings(ctx context.Context, ticketID string) ([]Warning, error) {
	query := "SELECT id, kind, severity, message, ticket_id, scope_id, created_at, acknowledged_at, ack_by FROM warnings"
	args := []interface{}{}
	if ticketID != "" {
		query += " WHERE ticket_id = ?"
		args = append(args, ticketID)
	}
	rows, err := d.db.QueryContext(ctx, query, args...)
	if err != nil {
		return nil, err
	}
	defer rows.Close()
	var warnings []Warning
	for rows.Next() {
		var warning Warning
		var acknowledged sql.NullTime
		if err := rows.Scan(&warning.ID, &warning.Kind, &warning.Severity, &warning.Message, &warning.TicketID, &warning.ScopeID, &warning.CreatedAt, &acknowledged, &warning.AcknowledgedBy); err != nil {
			return nil, err
		}
		if acknowledged.Valid {
			warning.Acknowledged = &acknowledged.Time
		}
		warnings = append(warnings, warning)
	}
	return warnings, nil
}

// 🔶listBreachs retrieves breachs optionally filtered by ticket ID.
func (d *Database) listBreachs(ctx context.Context, ticketID string) ([]Breach, error) {
	query := "SELECT id, kind, priority, scope_id, file_path, line, column, summary, excerpt, autofixable, detected_at, ticket_id, resolved_at FROM breachs"
	args := []interface{}{}
	if ticketID != "" {
		query += " WHERE ticket_id = ?"
		args = append(args, ticketID)
	}
	rows, err := d.db.QueryContext(ctx, query, args...)
	if err != nil {
		return nil, err
	}
	defer rows.Close()
	var breachs []Breach
	for rows.Next() {
		var breach Breach
		var line sql.NullInt64
		var column sql.NullInt64
		var resolved sql.NullTime
		if err := rows.Scan(&breach.ID, &breach.Kind, &breach.Priority, &breach.ScopeID, &breach.FilePath, &line, &column, &breach.Summary, &breach.Excerpt, &breach.Autofix, &breach.DetectedAt, &breach.TicketID, &resolved); err != nil {
			return nil, err
		}
		if line.Valid {
			value := int(line.Int64)
			breach.Line = &value
		}
		if column.Valid {
			value := int(column.Int64)
			breach.Column = &value
		}
		if resolved.Valid {
			breach.ResolvedAt = &resolved.Time
		}
		breachs = append(breachs, breach)
	}
	return breachs, nil
}

// 📬listConflicts finds scopes claimed by more than one open ticket.
func (d *Database) listConflicts(ctx context.Context) ([]struct {
	ScopeID string
	Tickets []string
}, error) {
	rows, err := d.db.QueryContext(ctx, "SELECT scope_id, GROUP_CONCAT(ticket_id) FROM ticket_claims JOIN tickets ON ticket_claims.ticket_id = tickets.id WHERE tickets.status = 'open' GROUP BY scope_id HAVING COUNT(ticket_id) > 1")
	if err != nil {
		return nil, err
	}
	defer rows.Close()
	var results []struct {
		ScopeID string
		Tickets []string
	}
	for rows.Next() {
		var scopeID string
		var ticketIDs string
		if err := rows.Scan(&scopeID, &ticketIDs); err != nil {
			return nil, err
		}
		results = append(results, struct {
			ScopeID string
			Tickets []string
		}{ScopeID: scopeID, Tickets: strings.Split(ticketIDs, ",")})
	}
	return results, nil
}

// 🤝addContributorWork holds the data fields for a addContributorWork record.
func (d *Database) addContributorWork(ctx context.Context, github string, kind string, itemID string) error {
	_, err := d.db.ExecContext(ctx, "INSERT OR REPLACE INTO contributor_work (github, kind, item_id) VALUES (?, ?, ?)", github, kind, itemID)
	return err
}

// 🚚removeContributorWork holds the data fields for a removeContributorWork record.
func (d *Database) removeContributorWork(ctx context.Context, github string, kindsAndIDs []struct{ Kind, ID string }) error {
	for _, kv := range kindsAndIDs {
		_, _ = d.db.ExecContext(ctx, "DELETE FROM contributor_work WHERE github = ? AND kind = ? AND item_id = ?", github, kv.Kind, kv.ID)
	}
	return nil
}

// 🔖listContributorsOnItem holds the data fields for a listContributorsOnItem record.
func (d *Database) listContributorsOnItem(ctx context.Context, kind string, itemID string) ([]string, error) {
	rows, err := d.db.QueryContext(ctx, "SELECT github FROM contributor_work WHERE kind = ? AND item_id = ?", kind, itemID)
	if err != nil {
		return nil, err
	}
	defer rows.Close()
	var out []string
	for rows.Next() {
		var g string
		if err := rows.Scan(&g); err != nil {
			return nil, err
		}
		out = append(out, g)
	}
	return out, nil
}

// 💾removeContributorWorkForCheckpoint holds the data fields for a removeContributorWorkForCheckpoint record.
func (d *Database) removeContributorWorkForCheckpoint(ctx context.Context, github string, files []string) error {
	for _, f := range files {
		_, _ = d.db.ExecContext(ctx, "DELETE FROM contributor_work WHERE github = ? AND kind = 'file' AND item_id = ?", github, f)
	}
	return nil
}

// #endregion 🔷Database

// #region ✨EventBus
// Asynchronous in-process event bus for decoupled event publishing and subscription. MUST persist events to the database before dispatching.

// 🎯EventHandler is a callback invoked when an event of a subscribed type is published.
type EventHandler func(context.Context, Event)

// 📡EventBus is a buffered channel-based event dispatcher with persistent storage.
type EventBus struct {
	ch       chan Event
	handlers map[string][]EventHandler
	db       *Database
	ctx      context.Context
	cancel   context.CancelFunc
	wg       sync.WaitGroup
}

// 🗄️NewEventBus creates a new event bus backed by the given database.
// 🆕MUST initialize the channel buffer to 256 and create a cancellable context.
func NewEventBus(db *Database) *EventBus {
	ctx, cancel := context.WithCancel(context.Background())
	return &EventBus{
		ch:       make(chan Event, 256),
		handlers: map[string][]EventHandler{},
		db:       db,
		ctx:      ctx,
		cancel:   cancel,
	}
}

// 🏷️Subscribe registers a handler for the given event type.
// ➕MUST append the handler to the handlers map.
func (b *EventBus) Subscribe(eventType string, handler EventHandler) {
	b.handlers[eventType] = append(b.handlers[eventType], handler)
}

// 📬Publish persists an event and dispatches it to subscribers.
// 💾MUST store the event in the database before sending to the channel.
func (b *EventBus) Publish(ctx context.Context, eventType string, source string, payload interface{}) error {
	payloadBytes, err := json.Marshal(payload)
	if err != nil {
		return err
	}
	event := Event{
		ID:        newID(),
		Type:      eventType,
		Source:    source,
		Payload:   string(payloadBytes),
		CreatedAt: time.Now().UTC(),
	}
	if err := b.db.insertEvent(ctx, event); err != nil {
		return err
	}
	select {
	case b.ch <- event:
		return nil
	case <-b.ctx.Done():
		return errors.New("event bus closed")
	}
}

// ▶️Start launches the event dispatch goroutine.
// ▶️MUST consume events from the channel and invoke registered handlers.
func (b *EventBus) Start() {
	b.wg.Add(1)
	go func() {
		defer b.wg.Done()
		for {
			select {
			case event := <-b.ch:
				if handlers := b.handlers[event.Type]; len(handlers) > 0 {
					for _, handler := range handlers {
						handler(b.ctx, event)
					}
				}
			case <-b.ctx.Done():
				return
			}
		}
	}()
}

// ⏹️Stop cancels the event bus context and waits for the dispatch goroutine to finish.
// ⏹️MUST block until the goroutine exits.
func (b *EventBus) Stop() {
	b.cancel()
	b.wg.Wait()
}

// #endregion ✨EventBus

// #region 🐍DiffParsing
// Unified diff parser that extracts file paths and hunk line ranges from patch text. MUST handle standard git diff output format.

// 🧩hunkHeader is a regex pattern matching unified diff hunk headers.
var hunkHeader = regexp.MustCompile(`@@ -([0-9]+)(?:,([0-9]+))? \+([0-9]+)(?:,([0-9]+))? @@`)

// 🧲parseUnifiedDiff extracts file paths and hunk ranges from a unified diff patch.
func parseUnifiedDiff(patch string) DiffResult {
	scanner := bufio.NewScanner(strings.NewReader(patch))
	var files []DiffFile
	var current *DiffFile
	for scanner.Scan() {
		line := scanner.Text()
		if strings.HasPrefix(line, "diff --git ") {
			parts := strings.Split(line, " ")
			if len(parts) >= 4 {
				path := strings.TrimPrefix(parts[3], "b/")
				files = append(files, DiffFile{Path: path})
				current = &files[len(files)-1]
			}
			continue
		}
		if strings.HasPrefix(line, "--- ") && current != nil {
			if strings.Contains(line, "/dev/null") {
			}
			continue
		}
		if strings.HasPrefix(line, "+++ ") && current != nil {
			if strings.Contains(line, "/dev/null") {
				current.Deleted = true
			}
			continue
		}
		if strings.HasPrefix(line, "@@ ") && current != nil {
			match := hunkHeader.FindStringSubmatch(line)
			if len(match) >= 5 {
				oldStart := parseHunkInt(match[1])
				oldCount := parseHunkIntWithDefault(match[2], 1)
				newStart := parseHunkInt(match[3])
				newCount := parseHunkIntWithDefault(match[4], 1)
				current.Hunks = append(current.Hunks, DiffHunk{
					OldRange: LineRange{Start: oldStart, End: oldStart + oldCount - 1},
					NewRange: LineRange{Start: newStart, End: newStart + newCount - 1},
				})
			}
		}
	}
	return DiffResult{Files: files}
}

// 🔬parseHunkInt parses a hunk header integer value.
func parseHunkInt(value string) int {
	parsed, _ := strconv.Atoi(value)
	return parsed
}

// 🔷parseHunkIntWithDefault parses a hunk header integer or returns the fallback.
func parseHunkIntWithDefault(value string, fallback int) int {
	if value == "" {
		return fallback
	}
	parsed, err := strconv.Atoi(value)
	if err != nil {
		return fallback
	}
	return parsed
}

// #endregion 🐍DiffParsing

// #region 📦Indexing
// Source code indexer that delegates to the shared repo/go parsing package. MUST support region-marker-based sections and language-specific definition patterns.

// 🔭IndexCache holds in-memory caches of indexed scopes partitioned by file path.
type IndexCache struct {
	Sections    map[string][]Scope
	Definitions map[string][]Scope
	Files       map[string]Scope
}

// 🆕newIndexCache creates an empty IndexCache with initialized maps.
func newIndexCache() IndexCache {
	return IndexCache{
		Sections:    map[string][]Scope{},
		Definitions: map[string][]Scope{},
		Files:       map[string]Scope{},
	}
}

// 🏗️buildScopesForFile delegates to the shared repopkg.BuildScopesForFile and converts ScopeEntry to Scope.
func buildScopesForFile(path string, content string) []Scope {
	now := time.Now().UTC()
	entries := repopkg.BuildScopesForFile(path, content)
	scopes := make([]Scope, len(entries))
	for i, e := range entries {
		scopes[i] = Scope{
			ID:          e.ID,
			Kind:        e.Kind,
			FilePath:    e.FilePath,
			SectionPath: e.SectionPath,
			Definition:  e.Definition,
			StartLine:   e.StartLine,
			EndLine:     e.EndLine,
			UpdatedAt:   now,
		}
	}
	return scopes
}

// #endregion 📦Indexing
// #region 🗡️Claims
// Scope claim mapping logic that associates diff hunks with overlapping scopes. MUST detect multi-ticket conflicts.

// 🔭mapClaims maps diff hunks to overlapping scopes and returns claimed IDs.
func mapClaims(scopes []Scope, diff DiffResult) ([]string, map[string][]Scope) {
	claimed := map[string][]Scope{}
	var claimedIDs []string
	for _, file := range diff.Files {
		if file.Path == "" {
			continue
		}
		fileScopes := filterScopesByFile(scopes, file.Path)
		for _, hunk := range file.Hunks {
			if hunk.NewRange.End == 0 {
				continue
			}
			for _, scope := range fileScopes {
				if scope.StartLine == 0 && scope.EndLine == 0 {
					continue
				}
				if rangesOverlap(hunk.NewRange, LineRange{Start: scope.StartLine, End: scope.EndLine}) {
					if scope.Kind == "definition" || scope.Kind == "section" {
						claimed[scope.ID] = append(claimed[scope.ID], scope)
					}
				}
			}
		}
	}
	sort.Strings(claimedIDs)
	return claimedIDs, claimed
}

// 🧹filterScopesByFile returns scopes matching the given file path.
func filterScopesByFile(scopes []Scope, filePath string) []Scope {
	var filtered []Scope
	for _, scope := range scopes {
		if scope.FilePath == filePath {
			filtered = append(filtered, scope)
		}
	}
	return filtered
}

// 🧪rangesOverlap tests whether two line ranges overlap.
func rangesOverlap(a LineRange, b LineRange) bool {
	if a.Start == 0 || b.Start == 0 {
		return false
	}
	return a.Start <= b.End && b.Start <= a.End
}

// 🔤appendIfMissing appends a string to a slice only if it is not already present.
func appendIfMissing(list []string, value string) []string {
	for _, item := range list {
		if item == value {
			return list
		}
	}
	return append(list, value)
}

// #endregion 🗡️Claims

// #region 🎊Warnings
// Conflict warning generation from multi-ticket scope overlaps. MUST produce error-severity warnings for blocking conflicts.

// 💿buildConflictWarnings creates warning records from detected scope conflicts.
func buildConflictWarnings(conflicts []struct {
	ScopeID string
	Tickets []string
}) []Warning {
	now := time.Now().UTC()
	var warnings []Warning
	for _, conflict := range conflicts {
		message := fmt.Sprintf("conflict on %s across tickets %s", conflict.ScopeID, strings.Join(conflict.Tickets, ", "))
		warnings = append(warnings, Warning{
			ID:        newID(),
			Kind:      "conflict",
			Severity:  "error",
			Message:   message,
			ScopeID:   conflict.ScopeID,
			CreatedAt: now,
		})
	}
	return warnings
}

// #endregion 🎊Warnings

// #region 🗻Server
// HTTP server with ticket lifecycle, diff ingestion, indexing, and webhook endpoints. MUST enforce authentication on mutating routes.

// 🗄️Server is the main HTTP server holding configuration, database, event bus, and caches.
type Server struct {
	config      Config
	db          *Database
	bus         *EventBus
	logger      *log.Logger
	cache       IndexCache
	cacheLock   sync.RWMutex
	githubCache map[string]GitHubComment
	ghLock      sync.Mutex
}

// ⚙️NewServer creates a new Server with the given config, database, and event bus.
// 💾MUST initialize the index cache and GitHub comment cache.
func NewServer(config Config, db *Database, bus *EventBus) *Server {
	return &Server{
		config:      config,
		db:          db,
		bus:         bus,
		logger:      log.New(os.Stdout, "", log.LstdFlags),
		cache:       newIndexCache(),
		githubCache: map[string]GitHubComment{},
	}
}

// 📨newRequestContext creates a request-scoped context with a 15-second timeout.
func (s *Server) newRequestContext(r *http.Request) (context.Context, context.CancelFunc) {
	return context.WithTimeout(r.Context(), 15*time.Second)
}

// 🖥️requireAuth checks the bearer token against the configured server token.
func (s *Server) requireAuth(r *http.Request) bool {
	if s.config.Token == "" {
		return true
	}
	auth := r.Header.Get("Authorization")
	if auth == "" {
		return false
	}
	parts := strings.SplitN(auth, " ", 2)
	if len(parts) != 2 || parts[0] != "Bearer" {
		return false
	}
	return parts[1] == s.config.Token
}

// 📋decodeJSON reads and decodes a JSON request body with size limits.
func (s *Server) decodeJSON(r *http.Request, payload interface{}) error {
	decoder := json.NewDecoder(io.LimitReader(r.Body, s.config.RequestBodyLimit))
	decoder.DisallowUnknownFields()
	return decoder.Decode(payload)
}

// 📩writeJSON writes a JSON response with the given status code.
func (s *Server) writeJSON(w http.ResponseWriter, status int, payload interface{}) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(status)
	_ = json.NewEncoder(w).Encode(payload)
}

// ❌respondError writes a JSON error response.
func (s *Server) respondError(w http.ResponseWriter, status int, message string) {
	s.writeJSON(w, status, map[string]string{"error": message})
}

// 📦handleEvents accepts CLI event payloads and persists/publishes them.
func (s *Server) handleEvents(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	if !s.requireAuth(r) {
		s.respondError(w, http.StatusUnauthorized, "unauthorized")
		return
	}
	var ev repopkg.Event
	if err := s.decodeJSON(r, &ev); err != nil {
		s.respondError(w, http.StatusBadRequest, err.Error())
		return
	}
	if ev.Kind == "" {
		s.respondError(w, http.StatusBadRequest, "kind required")
		return
	}
	ctx, cancel := s.newRequestContext(r)
	defer cancel()
	var payload interface{}
	if err := json.Unmarshal(ev.Payload, &payload); err != nil {
		payload = map[string]string{"raw": string(ev.Payload)}
	}
	if err := s.bus.Publish(ctx, string(ev.Kind), ev.Source, payload); err != nil {
		s.respondError(w, http.StatusInternalServerError, err.Error())
		return
	}
	s.writeJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}

// ✔️handleHealth responds with 200 OK for liveness checks.
func (s *Server) handleHealth(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	w.WriteHeader(http.StatusOK)
	_, _ = w.Write([]byte("ok"))
}

// 🎫handleTicketOpen creates a new ticket from the request payload.
func (s *Server) handleTicketOpen(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	if !s.requireAuth(r) {
		s.respondError(w, http.StatusUnauthorized, "unauthorized")
		return
	}
	var payload TicketOpenRequest
	if err := s.decodeJSON(r, &payload); err != nil {
		s.respondError(w, http.StatusBadRequest, err.Error())
		return
	}
	if payload.TicketID == "" || payload.Title == "" {
		s.respondError(w, http.StatusBadRequest, "ticket_id and title required")
		return
	}
	ctx, cancel := s.newRequestContext(r)
	defer cancel()
	now := time.Now().UTC()
	ticket := Ticket{
		ID:        payload.TicketID,
		Status:    "open",
		Title:     payload.Title,
		Prompt:    payload.Prompt,
		LLM:       payload.LLM,
		Client:    payload.Client,
		Author:    payload.Author,
		GitHub:    payload.GitHubIssue,
		CreatedAt: now,
	}
	if err := s.db.upsertTicket(ctx, ticket); err != nil {
		s.respondError(w, http.StatusInternalServerError, err.Error())
		return
	}
	_ = s.bus.Publish(ctx, "TicketOpened", "repo-cli", ticket)
	s.writeJSON(w, http.StatusOK, ticket)
}

// 📪handleTicketClose closes an existing ticket with a summary.
func (s *Server) handleTicketClose(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	if !s.requireAuth(r) {
		s.respondError(w, http.StatusUnauthorized, "unauthorized")
		return
	}
	var payload TicketCloseRequest
	if err := s.decodeJSON(r, &payload); err != nil {
		s.respondError(w, http.StatusBadRequest, err.Error())
		return
	}
	if payload.TicketID == "" || payload.Summary == "" {
		s.respondError(w, http.StatusBadRequest, "ticket_id and summary required")
		return
	}
	ctx, cancel := s.newRequestContext(r)
	defer cancel()
	ticket, err := s.db.getTicket(ctx, payload.TicketID)
	if err != nil {
		s.respondError(w, http.StatusNotFound, err.Error())
		return
	}
	now := time.Now().UTC()
	ticket.Status = "closed"
	ticket.Summary = payload.Summary
	ticket.ClosedAt = &now
	if err := s.db.upsertTicket(ctx, *ticket); err != nil {
		s.respondError(w, http.StatusInternalServerError, err.Error())
		return
	}
	_ = s.bus.Publish(ctx, "TicketClosed", "repo-cli", ticket)
	s.writeJSON(w, http.StatusOK, ticket)
}

// 🔓handleTicketReopen reopens a closed ticket with a new prompt.
func (s *Server) handleTicketReopen(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	if !s.requireAuth(r) {
		s.respondError(w, http.StatusUnauthorized, "unauthorized")
		return
	}
	var payload TicketReopenRequest
	if err := s.decodeJSON(r, &payload); err != nil {
		s.respondError(w, http.StatusBadRequest, err.Error())
		return
	}
	if payload.TicketID == "" || payload.Prompt == "" {
		s.respondError(w, http.StatusBadRequest, "ticket_id and prompt required")
		return
	}
	ctx, cancel := s.newRequestContext(r)
	defer cancel()
	ticket, err := s.db.getTicket(ctx, payload.TicketID)
	if err != nil {
		s.respondError(w, http.StatusNotFound, err.Error())
		return
	}
	ticket.Status = "open"
	ticket.Prompt = payload.Prompt
	ticket.LLM = payload.LLM
	if payload.Title != "" {
		ticket.Title = payload.Title
	}
	ticket.ClosedAt = nil
	if err := s.db.upsertTicket(ctx, *ticket); err != nil {
		s.respondError(w, http.StatusInternalServerError, err.Error())
		return
	}
	_ = s.bus.Publish(ctx, "TicketReopened", "repo-cli", ticket)
	s.writeJSON(w, http.StatusOK, ticket)
}

// 🧹handleTicketsQuery lists tickets optionally filtered by status.
func (s *Server) handleTicketsQuery(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	if !s.requireAuth(r) {
		s.respondError(w, http.StatusUnauthorized, "unauthorized")
		return
	}
	status := r.URL.Query().Get("status")
	ctx, cancel := s.newRequestContext(r)
	defer cancel()
	tickets, err := s.db.listTickets(ctx, status)
	if err != nil {
		s.respondError(w, http.StatusInternalServerError, err.Error())
		return
	}
	s.writeJSON(w, http.StatusOK, tickets)
}

// 🧲handleTicketDetail returns a single ticket by its path-extracted ID.
func (s *Server) handleTicketDetail(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	if !s.requireAuth(r) {
		s.respondError(w, http.StatusUnauthorized, "unauthorized")
		return
	}
	path := strings.TrimPrefix(r.URL.Path, "/tickets/")
	if path == "" {
		s.respondError(w, http.StatusNotFound, "ticket not found")
		return
	}
	if strings.HasSuffix(path, "/claims") {
		s.handleTicketClaims(w, r)
		return
	}
	ctx, cancel := s.newRequestContext(r)
	defer cancel()
	ticket, err := s.db.getTicket(ctx, path)
	if err != nil {
		s.respondError(w, http.StatusNotFound, err.Error())
		return
	}
	s.writeJSON(w, http.StatusOK, ticket)
}

// 🔭handleTicketClaims returns scope claims for a ticket.
func (s *Server) handleTicketClaims(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	path := strings.TrimSuffix(strings.TrimPrefix(r.URL.Path, "/tickets/"), "/claims")
	if path == "" {
		s.respondError(w, http.StatusNotFound, "ticket not found")
		return
	}
	ctx, cancel := s.newRequestContext(r)
	defer cancel()
	claims, err := s.db.listClaimsByTicket(ctx, path)
	if err != nil {
		s.respondError(w, http.StatusInternalServerError, err.Error())
		return
	}
	s.writeJSON(w, http.StatusOK, claims)
}

// ♻️handleDiffIngest ingests a diff patch, indexes changed files, maps claims, and returns results.
func (s *Server) handleDiffIngest(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	if !s.requireAuth(r) {
		s.respondError(w, http.StatusUnauthorized, "unauthorized")
		return
	}
	var payload DiffIngestRequest
	if err := s.decodeJSON(r, &payload); err != nil {
		s.respondError(w, http.StatusBadRequest, err.Error())
		return
	}
	if payload.TicketID == "" || payload.Patch == "" {
		s.respondError(w, http.StatusBadRequest, "ticket_id and patch required")
		return
	}
	ctx, cancel := s.newRequestContext(r)
	defer cancel()
	result, warnings, breachs, err := s.processDiff(ctx, payload.TicketID, payload.Patch, payload.Snapshots)
	if err != nil {
		s.respondError(w, http.StatusInternalServerError, err.Error())
		return
	}
	response := DiffIngestResponse{
		ChangedFiles:  result.ChangedFiles,
		ClaimedScopes: result.ClaimedScopes,
		Warnings:      warnings,
		Breachs:       breachs,
		Blockers:      result.Blockers,
	}
	s.writeJSON(w, http.StatusOK, response)
}

// 🎯handleReindex walks the repo and re-indexes all files.
func (s *Server) handleReindex(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	if !s.requireAuth(r) {
		s.respondError(w, http.StatusUnauthorized, "unauthorized")
		return
	}
	ctx, cancel := s.newRequestContext(r)
	defer cancel()
	files, err := s.walkRepoFiles()
	if err != nil {
		s.respondError(w, http.StatusInternalServerError, err.Error())
		return
	}
	for _, file := range files {
		content, err := os.ReadFile(filepath.Join(s.config.RepoRoot, file))
		if err != nil {
			continue
		}
		s.updateIndexForFile(ctx, file, string(content))
	}
	s.writeJSON(w, http.StatusOK, map[string]int{"files": len(files)})
}

// 📄handleIndexFile indexes a single file from the request payload.
func (s *Server) handleIndexFile(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	if !s.requireAuth(r) {
		s.respondError(w, http.StatusUnauthorized, "unauthorized")
		return
	}
	var payload IndexFileRequest
	if err := s.decodeJSON(r, &payload); err != nil {
		s.respondError(w, http.StatusBadRequest, err.Error())
		return
	}
	if payload.FilePath == "" {
		s.respondError(w, http.StatusBadRequest, "file_path required")
		return
	}
	ctx, cancel := s.newRequestContext(r)
	defer cancel()
	s.updateIndexForFile(ctx, payload.FilePath, payload.Content)
	s.writeJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}

// ⚠️handleWarnings returns warnings optionally filtered by ticket ID.
func (s *Server) handleWarnings(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	if !s.requireAuth(r) {
		s.respondError(w, http.StatusUnauthorized, "unauthorized")
		return
	}
	ctx, cancel := s.newRequestContext(r)
	defer cancel()
	warnings, err := s.db.listWarnings(ctx, r.URL.Query().Get("ticket_id"))
	if err != nil {
		s.respondError(w, http.StatusInternalServerError, err.Error())
		return
	}
	s.writeJSON(w, http.StatusOK, warnings)
}

// 🔷handleBreachs returns breachs optionally filtered by ticket ID.
func (s *Server) handleBreachs(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	if !s.requireAuth(r) {
		s.respondError(w, http.StatusUnauthorized, "unauthorized")
		return
	}
	ctx, cancel := s.newRequestContext(r)
	defer cancel()
	breachs, err := s.db.listBreachs(ctx, r.URL.Query().Get("ticket_id"))
	if err != nil {
		s.respondError(w, http.StatusInternalServerError, err.Error())
		return
	}
	s.writeJSON(w, http.StatusOK, breachs)
}

// 🔍handleScopes returns scopes for a given file query parameter.
func (s *Server) handleScopes(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	if !s.requireAuth(r) {
		s.respondError(w, http.StatusUnauthorized, "unauthorized")
		return
	}
	filePath := r.URL.Query().Get("file")
	if filePath == "" {
		s.respondError(w, http.StatusBadRequest, "file query required")
		return
	}
	ctx, cancel := s.newRequestContext(r)
	defer cancel()
	scopes, err := s.db.listScopesByFile(ctx, filePath)
	if err != nil {
		s.respondError(w, http.StatusInternalServerError, err.Error())
		return
	}
	s.writeJSON(w, http.StatusOK, scopes)
}

// #endregion 🗻Server

// #region 📌Processing
// Diff processing pipeline that indexes changed files, maps claims, detects conflicts, and produces warnings. MUST be transactional per request.

// 🔷ProcessResult holds the outcome of a diff processing operation.
type ProcessResult struct {
	ChangedFiles  []string
	ClaimedScopes []string
	Blockers      []string
}

// ♻️processDiff parses the patch, indexes changed files, maps claims, and detects conflicts.
// ⚠️MUST return warnings and breachs alongside the processing result.
func (s *Server) processDiff(ctx context.Context, ticketID string, patch string, snapshots []FileSnapshot) (ProcessResult, []Warning, []Breach, error) {
	diff := parseUnifiedDiff(patch)
	changedFiles := uniqueFiles(diff.Files)
	if err := s.bus.Publish(ctx, "DiffIngested", "repo-cli", map[string]interface{}{"ticket_id": ticketID, "files": changedFiles}); err != nil {
		return ProcessResult{}, nil, nil, err
	}
	contentByFile := snapshotMap(snapshots)
	for _, file := range changedFiles {
		content, ok := contentByFile[file]
		if !ok {
			data, err := os.ReadFile(filepath.Join(s.config.RepoRoot, file))
			if err != nil {
				continue
			}
			content = string(data)
		}
		s.updateIndexForFile(ctx, file, content)
	}
	s.cacheLock.RLock()
	var scopes []Scope
	for _, file := range changedFiles {
		scopes = append(scopes, s.cache.Sections[file]...)
		scopes = append(scopes, s.cache.Definitions[file]...)
	}
	s.cacheLock.RUnlock()
	now := time.Now().UTC()
	claimedIDs, _ := mapClaims(scopes, diff)
	for _, scopeID := range claimedIDs {
		if err := s.db.upsertClaim(ctx, ticketID, scopeID, "touched", now); err != nil {
			return ProcessResult{}, nil, nil, err
		}
	}
	conflicts, err := s.db.listConflicts(ctx)
	if err != nil {
		return ProcessResult{}, nil, nil, err
	}
	warnings := buildConflictWarnings(conflicts)
	if err := s.db.replaceWarnings(ctx, warnings); err != nil {
		return ProcessResult{}, nil, nil, err
	}
	blockers := []string{}
	for _, warning := range warnings {
		if warning.Severity == "error" {
			blockers = append(blockers, warning.Message)
		}
	}
	result := ProcessResult{
		ChangedFiles:  changedFiles,
		ClaimedScopes: claimedIDs,
		Blockers:      blockers,
	}
	return result, warnings, []Breach{}, nil
}

// 🧲uniqueFiles extracts deduplicated file paths from a diff result.
func uniqueFiles(files []DiffFile) []string {
	var list []string
	for _, file := range files {
		if file.Path != "" {
			list = appendIfMissing(list, file.Path)
		}
	}
	return list
}

// 📸snapshotMap converts a slice of file snapshots into a path-to-content map.
func snapshotMap(snapshots []FileSnapshot) map[string]string {
	mapping := map[string]string{}
	for _, snapshot := range snapshots {
		mapping[snapshot.Path] = snapshot.Content
	}
	return mapping
}

// 🗄️updateIndexForFile builds scopes from file content and updates both the database and cache.
func (s *Server) updateIndexForFile(ctx context.Context, filePath string, content string) {
	scopes := buildScopesForFile(filePath, content)
	var fileScope Scope
	var sections []Scope
	var definitions []Scope
	for _, scope := range scopes {
		if scope.Kind == "file" {
			fileScope = scope
		}
		if scope.Kind == "section" {
			sections = append(sections, scope)
		}
		if scope.Kind == "definition" {
			definitions = append(definitions, scope)
		}
	}
	_ = s.db.replaceScopes(ctx, filePath, scopes)
	s.cacheLock.Lock()
	s.cache.Files[filePath] = fileScope
	s.cache.Sections[filePath] = sections
	s.cache.Definitions[filePath] = definitions
	s.cacheLock.Unlock()
	_ = s.bus.Publish(ctx, "IndexUpdated", "server", map[string]interface{}{"file": filePath})
}

// 📄walkRepoFiles walks the repo root and returns all non-hidden file paths.
func (s *Server) walkRepoFiles() ([]string, error) {
	var files []string
	err := filepath.Walk(s.config.RepoRoot, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if info.IsDir() {
			if strings.HasPrefix(info.Name(), ".") && info.Name() != "." {
				return filepath.SkipDir
			}
			return nil
		}
		rel, err := filepath.Rel(s.config.RepoRoot, path)
		if err != nil {
			return err
		}
		files = append(files, filepath.ToSlash(rel))
		return nil
	})
	if err != nil {
		return nil, err
	}
	return files, nil
}

// #endregion 📌Processing

// #region 🖼️Webhooks
// GitHub webhook handlers for issue comment caching and issue event processing. MUST verify HMAC signatures when a secret is configured.

// 💬GitHubComment stores a cached GitHub issue comment for correlating close/reopen events.
type GitHubComment struct {
	Body   string
	Actor  string
	Repo   string
	Issue  int
	Second time.Time
}

// 🐙handleGitHubWebhook processes incoming GitHub webhook events.
func (s *Server) handleGitHubWebhook(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	if !s.requireAuth(r) {
		s.respondError(w, http.StatusUnauthorized, "unauthorized")
		return
	}
	body, err := io.ReadAll(r.Body)
	if err != nil {
		s.respondError(w, http.StatusBadRequest, err.Error())
		return
	}
	if s.config.GitHubSecret != "" {
		signature := r.Header.Get("X-Hub-Signature-256")
		if !verifyGitHubSignature(body, signature, s.config.GitHubSecret) {
			s.respondError(w, http.StatusUnauthorized, "invalid signature")
			return
		}
	}
	eventType := r.Header.Get("X-GitHub-Event")
	ctx, cancel := s.newRequestContext(r)
	defer cancel()
	_ = s.bus.Publish(ctx, "GitHubIssueEventReceived", "github", map[string]interface{}{"type": eventType})
	if eventType == "issue_comment" {
		var payload map[string]interface{}
		_ = json.Unmarshal(body, &payload)
		s.cacheGitHubComment(payload)
	}
	if eventType == "issues" {
		var payload map[string]interface{}
		_ = json.Unmarshal(body, &payload)
		s.handleGitHubIssueEvent(ctx, payload)
	}
	if eventType == "push" {
		var payload map[string]interface{}
		_ = json.Unmarshal(body, &payload)
		s.handleGitHubPushEvent(ctx, payload)
	}
	w.WriteHeader(http.StatusOK)
}

// 📦verifyGitHubSignature validates the HMAC-SHA256 signature of a webhook payload.
func verifyGitHubSignature(body []byte, signature string, secret string) bool {
	parts := strings.SplitN(signature, "=", 2)
	if len(parts) != 2 {
		return false
	}
	mac := hmac.New(sha256.New, []byte(secret))
	mac.Write(body)
	computed := hex.EncodeToString(mac.Sum(nil))
	return hmac.Equal([]byte(computed), []byte(parts[1]))
}

// 📡cacheGitHubComment stores a GitHub comment for correlating subsequent events.
func (s *Server) cacheGitHubComment(payload map[string]interface{}) {
	issue, repo, actor, body := extractIssueComment(payload)
	if issue == 0 || repo == "" || actor == "" || body == "" {
		return
	}
	key := fmt.Sprintf("%s#%d#%s", repo, issue, actor)
	s.ghLock.Lock()
	s.githubCache[key] = GitHubComment{
		Body:   body,
		Actor:  actor,
		Repo:   repo,
		Issue:  issue,
		Second: time.Now().UTC(),
	}
	s.ghLock.Unlock()
}

// 🔓handleGitHubIssueEvent processes GitHub issue close/reopen events.
func (s *Server) handleGitHubIssueEvent(ctx context.Context, payload map[string]interface{}) {
	action, _ := payload["action"].(string)
	issueNumber := extractIssueNumber(payload)
	repo := extractRepoFullName(payload)
	actor := extractActorLogin(payload)
	if issueNumber == 0 || repo == "" || actor == "" {
		return
	}
	comment := s.findCachedComment(repo, issueNumber, actor)
	if action == "closed" && comment.Body != "" {
		_ = s.bus.Publish(ctx, "TicketClosed", "github", map[string]interface{}{"issue": issueNumber, "comment": comment.Body})
	}
	if action == "reopened" && comment.Body != "" {
		_ = s.bus.Publish(ctx, "TicketReopened", "github", map[string]interface{}{"issue": issueNumber, "comment": comment.Body})
	}
}

// 💾findCachedComment retrieves a recently cached GitHub comment for the given issue.
func (s *Server) findCachedComment(repo string, issue int, actor string) GitHubComment {
	key := fmt.Sprintf("%s#%d#%s", repo, issue, actor)
	s.ghLock.Lock()
	defer s.ghLock.Unlock()
	comment, ok := s.githubCache[key]
	if !ok {
		return GitHubComment{}
	}
	if time.Since(comment.Second) > 90*time.Second {
		delete(s.githubCache, key)
		return GitHubComment{}
	}
	return comment
}

// 🧲extractIssueComment extracts issue number, repo, actor, and body from a webhook payload.
func extractIssueComment(payload map[string]interface{}) (int, string, string, string) {
	issueNumber := extractIssueNumber(payload)
	repo := extractRepoFullName(payload)
	actor := extractActorLogin(payload)
	body := ""
	if comment, ok := payload["comment"].(map[string]interface{}); ok {
		body, _ = comment["body"].(string)
	}
	return issueNumber, repo, actor, body
}

// 🔢extractIssueNumber extracts the issue number from a GitHub webhook payload.
func extractIssueNumber(payload map[string]interface{}) int {
	if issue, ok := payload["issue"].(map[string]interface{}); ok {
		if number, ok := issue["number"].(float64); ok {
			return int(number)
		}
	}
	return 0
}

// 🔷extractRepoFullName extracts the repository full name from a GitHub webhook payload.
func extractRepoFullName(payload map[string]interface{}) string {
	if repo, ok := payload["repository"].(map[string]interface{}); ok {
		if name, ok := repo["full_name"].(string); ok {
			return name
		}
	}
	return ""
}

// 💿handleGitHubPushEvent holds the data fields for a handleGitHubPushEvent record.
func (s *Server) handleGitHubPushEvent(ctx context.Context, payload map[string]interface{}) {
	actor := extractActorLogin(payload)
	if actor == "" {
		if pusher, ok := payload["pusher"].(map[string]interface{}); ok {
			if name, ok := pusher["name"].(string); ok {
				actor = name
			}
		}
	}
	var files []string
	if checkpoints, ok := payload["commits"].([]interface{}); ok {
		for _, c := range checkpoints {
			if cm, ok := c.(map[string]interface{}); ok {
				if added, ok := cm["added"].([]interface{}); ok {
					for _, a := range added {
						if p, ok := a.(string); ok {
							files = append(files, p)
						}
					}
				}
				if modified, ok := cm["modified"].([]interface{}); ok {
					for _, m := range modified {
						if p, ok := m.(string); ok {
							files = append(files, p)
						}
					}
				}
			}
		}
	}
	if actor != "" && len(files) > 0 {
		_ = s.db.removeContributorWorkForCheckpoint(ctx, actor, files)
	}
}

// 📤extractActorLogin extracts the sender login from a GitHub webhook payload.
func extractActorLogin(payload map[string]interface{}) string {
	if sender, ok := payload["sender"].(map[string]interface{}); ok {
		if login, ok := sender["login"].(string); ok {
			return login
		}
	}
	return ""
}

// #endregion 🖼️Webhooks

// 🔷#region 📔Discord
// ⚙️Discord notification integration for ticket lifecycle events. MUST silently skip when no webhook URL is configured.
func (s *Server) notifyDiscord(title string, body string) {
	if s.config.DiscordWebhook == "" {
		return
	}
	payload := map[string]string{"content": fmt.Sprintf("%s\n%s", title, body)}
	data, _ := json.Marshal(payload)
	request, err := http.NewRequest(http.MethodPost, s.config.DiscordWebhook, strings.NewReader(string(data)))
	if err != nil {
		return
	}
	request.Header.Set("Content-Type", "application/json")
	client := &http.Client{Timeout: 5 * time.Second}
	_, _ = client.Do(request)
}

// 🔔registerNotifications subscribes to ticket lifecycle events and sends Discord notifications.
func (s *Server) registerNotifications() {
	s.bus.Subscribe("TicketOpened", func(ctx context.Context, event Event) {
		s.notifyDiscord("# Prompt", event.Payload)
	})
	s.bus.Subscribe("TicketClosed", func(ctx context.Context, event Event) {
		s.notifyDiscord("# Summary", event.Payload)
	})
	s.bus.Subscribe("TicketReopened", func(ctx context.Context, event Event) {
		s.notifyDiscord("# Prompt", event.Payload)
	})
	for _, kind := range []repopkg.EventKind{
		repopkg.EventTicketOpenEnded, repopkg.EventTicketCloseEnded, repopkg.EventTicketReopenEnded, repopkg.EventTicketChangeEnded,
		repopkg.EventGoalOpenEnded, repopkg.EventGoalCloseEnded, repopkg.EventGoalReopenEnded, repopkg.EventGoalChangeEnded,
		repopkg.EventContributorAddEnded, repopkg.EventContributorRemoveEnded,
		repopkg.EventTodoCreateEnded, repopkg.EventTodoChangeEnded, repopkg.EventTodoDeleteEnded,
	} {
		k := kind
		s.bus.Subscribe(string(k), func(ctx context.Context, event Event) {
			s.onCLIEvent(ctx, k, event)
		})
	}
	s.bus.Subscribe(string(repopkg.EventCheckpointEnded), func(ctx context.Context, event Event) {
		s.onCheckpointEvent(ctx, event)
	})
}

// 💿onCLIEvent holds the data fields for a onCLIEvent record.
func (s *Server) onCLIEvent(ctx context.Context, kind repopkg.EventKind, event Event) {
	s.notifyDiscord(string(kind), event.Payload)
	author, items := s.extractAuthorAndItems(kind, event.Payload)
	if author == "" {
		return
	}
	for _, item := range items {
		if item.Kind == "" || item.ID == "" {
			continue
		}
		others, _ := s.db.listContributorsOnItem(ctx, item.Kind, item.ID)
		others = filterOut(others, author)
		if len(others) > 0 {
			s.notifyDiscord("⚠️ Conflict", fmt.Sprintf("%s working on %s:%s (others: %v)", author, item.Kind, item.ID, others))
		}
		_ = s.db.addContributorWork(ctx, author, item.Kind, item.ID)
	}
}

// 💾onCheckpointEvent holds the data fields for a onCheckpointEvent record.
func (s *Server) onCheckpointEvent(ctx context.Context, event Event) {
	var p repopkg.CheckpointPayload
	if json.Unmarshal([]byte(event.Payload), &p) != nil {
		return
	}
	files := p.FilesChanged
	if len(files) == 0 {
		files = p.Files
	}
	_ = s.db.removeContributorWorkForCheckpoint(ctx, p.Author, files)
}

// 🧲extractAuthorAndItems holds the data fields for a extractAuthorAndItems record.
func (s *Server) extractAuthorAndItems(kind repopkg.EventKind, payloadJSON string) (author string, items []repopkg.WorkItem) {
	switch kind {
	case repopkg.EventTicketOpenEnded, repopkg.EventTicketCloseEnded, repopkg.EventTicketReopenEnded, repopkg.EventTicketChangeEnded:
		var p repopkg.TicketPayload
		if json.Unmarshal([]byte(payloadJSON), &p) != nil {
			return "", nil
		}
		author = getAuthorFromPayload(payloadJSON)
		if author == "" {
			return "", nil
		}
		id := p.ID
		if id == "" && (p.Year|p.Month|p.Day) != 0 {
			id = fmt.Sprintf("%d/%02d/%02d/%s", p.Year, p.Month, p.Day, p.Slug)
		}
		return author, []repopkg.WorkItem{{Kind: "ticket", ID: id}}
	case repopkg.EventGoalOpenEnded, repopkg.EventGoalCloseEnded, repopkg.EventGoalReopenEnded, repopkg.EventGoalChangeEnded:
		var p repopkg.GoalPayload
		if json.Unmarshal([]byte(payloadJSON), &p) != nil {
			return "", nil
		}
		author = getAuthorFromPayload(payloadJSON)
		return author, []repopkg.WorkItem{{Kind: "goal", ID: p.ID}}
	case repopkg.EventContributorAddEnded, repopkg.EventContributorRemoveEnded:
		var p repopkg.ContributorPayload
		if json.Unmarshal([]byte(payloadJSON), &p) != nil {
			return "", nil
		}
		return p.Author, []repopkg.WorkItem{{Kind: "contributor", ID: p.Github}}
	case repopkg.EventTodoCreateEnded, repopkg.EventTodoChangeEnded, repopkg.EventTodoDeleteEnded:
		var p repopkg.TodoPayload
		if json.Unmarshal([]byte(payloadJSON), &p) != nil {
			return "", nil
		}
		return p.Author, []repopkg.WorkItem{{Kind: "todo", ID: p.ID}}
	default:
		return "", nil
	}
}

// 📦getAuthorFromPayload holds the data fields for a getAuthorFromPayload record.
func getAuthorFromPayload(payloadJSON string) string {
	var m map[string]interface{}
	if json.Unmarshal([]byte(payloadJSON), &m) != nil {
		return ""
	}
	if a, ok := m["author"].(string); ok {
		return a
	}
	return ""
}

// 🧹filterOut holds the data fields for a filterOut record.
func filterOut(list []string, exclude string) []string {
	var out []string
	for _, x := range list {
		if x != exclude {
			out = append(out, x)
		}
	}
	return out
}

// #endregion 📔Discord

// 💿newID holds the data fields for a newID record.
func newID() string {
	return fmt.Sprintf("%d-%d", time.Now().UTC().UnixNano(), rand.Int63())
}

// #endregion 🔖Utilities

// 🔷main holds the data fields for a main record.
func main() {
	config := loadConfig()
	db, err := openDatabase(config.DatabasePath)
	if err != nil {
		log.Fatal(err)
	}
	bus := NewEventBus(db)
	server := NewServer(config, db, bus)
	server.registerNotifications()
	bus.Start()
	defer bus.Stop()
	mux := http.NewServeMux()
	mux.HandleFunc("/healthz", server.handleHealth)
	mux.HandleFunc("/tickets/open", server.handleTicketOpen)
	mux.HandleFunc("/tickets/close", server.handleTicketClose)
	mux.HandleFunc("/tickets/reopen", server.handleTicketReopen)
	mux.HandleFunc("/tickets", server.handleTicketsQuery)
	mux.HandleFunc("/tickets/", server.handleTicketDetail)
	mux.HandleFunc("/diff/ingest", server.handleDiffIngest)
	mux.HandleFunc("/repo/reindex", server.handleReindex)
	mux.HandleFunc("/repo/index-file", server.handleIndexFile)
	mux.HandleFunc("/warnings", server.handleWarnings)
	mux.HandleFunc("/breachs", server.handleBreachs)
	mux.HandleFunc("/scopes", server.handleScopes)
	mux.HandleFunc("/events", server.handleEvents)
	mux.HandleFunc("/webhooks/github", server.handleGitHubWebhook)
	log.Printf("repo server listening on %s", config.Address)
	if err := http.ListenAndServe(config.Address, mux); err != nil {
		log.Fatal(err)
	}
}

// #endregion 🔖Main
