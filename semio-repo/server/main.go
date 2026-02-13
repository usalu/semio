// #region 🔖Header

// [💻semio-repo/server/main.go](semiorepo://file/semio-repo/server/main.go)

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// GraphQL server for the monorepo management API.

// #endregion 🔖Header

// #region 🔖Package

// [🔖semio-repo/server/main.go#Package](semiorepo://section/semio-repo/server/main.go/PACKAGE)
// Package declaration for the semio repo server binary. MUST be package main.
package main

// #endregion 🔖Package

// #region 🔖Imports

// [🔖semio-repo/server/main.go#Imports](semiorepo://section/semio-repo/server/main.go/IMPORTS)
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

	_ "modernc.org/sqlite"
)

// #endregion 🔖Imports

// #region 🔖Config

// [🔖semio-repo/server/main.go#Config](semiorepo://section/semio-repo/server/main.go/CONFIG)
// Server configuration loading from environment variables. MUST provide sensible defaults.

// Config holds all server configuration values.
// [🛠️semio-repo/server/main.go#Config§Config](semiorepo://definition/semio-repo/server/main.go/CONFIG/CONFIG)
type Config struct {
	Address          string
	DatabasePath     string
	RepoRoot         string
	Token            string
	GitHubSecret     string
	DiscordWebhook   string
	RequestBodyLimit int64
}

// loadConfig reads server configuration from environment variables with fallback defaults.
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

// envOrDefault returns the environment variable value or the fallback if empty.
func envOrDefault(key, fallback string) string {
	if value := strings.TrimSpace(os.Getenv(key)); value != "" {
		return value
	}
	return fallback
}

// envOrDefaultInt64 returns the parsed int64 environment variable or the fallback.
func envOrDefaultInt64(key string, fallback int64) int64 {
	if value := strings.TrimSpace(os.Getenv(key)); value != "" {
		if parsed, err := strconv.ParseInt(value, 10, 64); err == nil {
			return parsed
		}
	}
	return fallback
}

// #endregion 🔖Config

// #region 🔖Models

// [🔖semio-repo/server/main.go#Models](semiorepo://section/semio-repo/server/main.go/MODELS)
// Data model types for tickets, scopes, warnings, violations, events, and API request/response payloads. MUST mirror the server SQLite schema.

// Ticket represents a tracked work item with lifecycle status.
// [🛠️semio-repo/server/main.go#Models§Ticket](semiorepo://definition/semio-repo/server/main.go/MODELS/TICKET)
type Ticket struct {
	ID        string     `json:"id"`
	Status    string     `json:"status"`
	Title     string     `json:"title"`
	Prompt    string     `json:"prompt"`
	Summary   string     `json:"summary"`
	LLM       string     `json:"llm"`
	Client        string     `json:"client"`
	Author    string     `json:"author"`
	GitHub    string     `json:"github_issue"`
	CreatedAt time.Time  `json:"created_at"`
	ClosedAt  *time.Time `json:"closed_at"`
}

// Scope represents a code region (file, section, or definition) with line range.
// [🛠️semio-repo/server/main.go#Models§Scope](semiorepo://definition/semio-repo/server/main.go/MODELS/SCOPE)
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

// Warning represents a detected issue such as a scope conflict between tickets.
// [🛠️semio-repo/server/main.go#Models§Warning](semiorepo://definition/semio-repo/server/main.go/MODELS/WARNING)
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

// Violation represents a policy violation detected in source code.
// [🛠️semio-repo/server/main.go#Models§Violation](semiorepo://definition/semio-repo/server/main.go/MODELS/VIOLATION)
type Violation struct {
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

// Event represents a system event persisted to the event log.
// [🛠️semio-repo/server/main.go#Models§Event](semiorepo://definition/semio-repo/server/main.go/MODELS/EVENT)
type Event struct {
	ID        string    `json:"id"`
	Type      string    `json:"type"`
	Source    string    `json:"source"`
	Payload   string    `json:"payload_json"`
	CreatedAt time.Time `json:"created_at"`
}

// LineRange represents a contiguous range of line numbers.
// [🛠️semio-repo/server/main.go#Models§LineRange](semiorepo://definition/semio-repo/server/main.go/MODELS/LINERANGE)
type LineRange struct {
	Start int
	End   int
}

// DiffHunk represents a single hunk with old and new line ranges from a unified diff.
// [🛠️semio-repo/server/main.go#Models§DiffHunk](semiorepo://definition/semio-repo/server/main.go/MODELS/DIFFHUNK)
type DiffHunk struct {
	OldRange LineRange
	NewRange LineRange
}

// DiffFile represents a single file entry in a unified diff with its hunks.
// [🛠️semio-repo/server/main.go#Models§DiffFile](semiorepo://definition/semio-repo/server/main.go/MODELS/DIFFFILE)
type DiffFile struct {
	Path    string
	Hunks   []DiffHunk
	Deleted bool
	Created bool
}

// DiffResult aggregates all parsed diff files from a patch.
// [🛠️semio-repo/server/main.go#Models§DiffResult](semiorepo://definition/semio-repo/server/main.go/MODELS/DIFFRESULT)
type DiffResult struct {
	Files []DiffFile
}

// FileSnapshot holds the full content of a file for snapshot-based indexing.
// [🛠️semio-repo/server/main.go#Models§FileSnapshot](semiorepo://definition/semio-repo/server/main.go/MODELS/FILESNAPSHOT)
type FileSnapshot struct {
	Path    string `json:"path"`
	Content string `json:"content"`
}

// TicketOpenRequest is the JSON payload for opening a new ticket.
// [🛠️semio-repo/server/main.go#Models§TicketOpenRequest](semiorepo://definition/semio-repo/server/main.go/MODELS/TICKETOPENREQUEST)
type TicketOpenRequest struct {
	TicketID    string `json:"ticket_id"`
	Title       string `json:"title"`
	Prompt      string `json:"prompt"`
	LLM         string `json:"llm"`
	Client          string `json:"client"`
	Author      string `json:"author"`
	GitHubIssue string `json:"github_issue"`
}

// TicketCloseRequest is the JSON payload for closing a ticket.
// [🛠️semio-repo/server/main.go#Models§TicketCloseRequest](semiorepo://definition/semio-repo/server/main.go/MODELS/TICKETCLOSEREQUEST)
type TicketCloseRequest struct {
	TicketID string   `json:"ticket_id"`
	Summary  string   `json:"summary"`
	Files    []string `json:"files"`
}

// TicketReopenRequest is the JSON payload for reopening a closed ticket.
// [🛠️semio-repo/server/main.go#Models§TicketReopenRequest](semiorepo://definition/semio-repo/server/main.go/MODELS/TICKETREOPENREQUEST)
type TicketReopenRequest struct {
	TicketID string `json:"ticket_id"`
	Prompt   string `json:"prompt"`
	LLM      string `json:"llm"`
	Title    string `json:"title"`
}

// DiffIngestRequest is the JSON payload for ingesting a diff patch.
// [🛠️semio-repo/server/main.go#Models§DiffIngestRequest](semiorepo://definition/semio-repo/server/main.go/MODELS/DIFFINGESTREQUEST)
type DiffIngestRequest struct {
	TicketID  string         `json:"ticket_id"`
	RepoID    string         `json:"repo_id"`
	Patch     string         `json:"patch"`
	Snapshots []FileSnapshot `json:"snapshots"`
}

// DiffIngestResponse holds the results of a diff ingestion operation.
// [🛠️semio-repo/server/main.go#Models§DiffIngestResponse](semiorepo://definition/semio-repo/server/main.go/MODELS/DIFFINGESTRESPONSE)
type DiffIngestResponse struct {
	ChangedFiles  []string    `json:"changed_files"`
	ClaimedScopes []string    `json:"claimed_scopes"`
	Warnings      []Warning   `json:"warnings"`
	Violations    []Violation `json:"violations"`
	Blockers      []string    `json:"blockers"`
}

// PrecommitRequest is the JSON payload for a pre-commit check.
// [🛠️semio-repo/server/main.go#Models§PrecommitRequest](semiorepo://definition/semio-repo/server/main.go/MODELS/PRECOMMITREQUEST)
type PrecommitRequest struct {
	TicketID      string `json:"ticket_id"`
	Patch         string `json:"patch"`
	Staged        bool   `json:"staged"`
	CommitMessage string `json:"commit_message"`
}

// PrecommitResponse holds the result of a pre-commit check.
// [🛠️semio-repo/server/main.go#Models§PrecommitResponse](semiorepo://definition/semio-repo/server/main.go/MODELS/PRECOMMITRESPONSE)
type PrecommitResponse struct {
	OK           bool        `json:"ok"`
	Blockers     []string    `json:"blockers"`
	Warnings     []Warning   `json:"warnings"`
	Violations   []Violation `json:"violations"`
	AutofixPatch string      `json:"autofix_patch"`
}

// IndexFileRequest is the JSON payload for indexing a single file.
// [🛠️semio-repo/server/main.go#Models§IndexFileRequest](semiorepo://definition/semio-repo/server/main.go/MODELS/INDEXFILEREQUEST)
type IndexFileRequest struct {
	FilePath string `json:"file_path"`
	Content  string `json:"content"`
}

// #endregion 🔖Models

// #region 🔖Database

// [🔖semio-repo/server/main.go#Database](semiorepo://section/semio-repo/server/main.go/DATABASE)
// SQLite database layer for persistent storage of tickets, scopes, claims, warnings, violations, and events. MUST use WAL journal mode.

// Database wraps a sql.DB connection to the SQLite store.
// [🛠️semio-repo/server/main.go#Database§Database](semiorepo://definition/semio-repo/server/main.go/DATABASE/DATABASE)
type Database struct {
	db *sql.DB
}

// openDatabase opens an SQLite database and runs schema migrations.
// MUST enable WAL journal mode and foreign keys.
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

// migrate creates database tables if they do not already exist.
func (d *Database) migrate() error {
	statements := []string{
		"CREATE TABLE IF NOT EXISTS repos (id TEXT PRIMARY KEY, name TEXT, path TEXT, created_at DATETIME)",
		"CREATE TABLE IF NOT EXISTS tickets (id TEXT PRIMARY KEY, status TEXT, title TEXT, prompt TEXT, summary TEXT, llm TEXT, ui TEXT, author TEXT, github_issue TEXT, created_at DATETIME, closed_at DATETIME)",
		"CREATE TABLE IF NOT EXISTS scopes (id TEXT PRIMARY KEY, kind TEXT, file_path TEXT, section_path TEXT, definition_name TEXT, start_line INT, end_line INT, updated_at DATETIME)",
		"CREATE TABLE IF NOT EXISTS ticket_claims (ticket_id TEXT, scope_id TEXT, claim_type TEXT, first_seen_at DATETIME, last_seen_at DATETIME, PRIMARY KEY (ticket_id, scope_id))",
		"CREATE TABLE IF NOT EXISTS violations (id TEXT PRIMARY KEY, kind TEXT, priority TEXT, scope_id TEXT, file_path TEXT, line INT, column INT, summary TEXT, excerpt TEXT, autofixable BOOL, detected_at DATETIME, ticket_id TEXT, resolved_at DATETIME)",
		"CREATE TABLE IF NOT EXISTS warnings (id TEXT PRIMARY KEY, kind TEXT, severity TEXT, message TEXT, ticket_id TEXT, scope_id TEXT, created_at DATETIME, acknowledged_at DATETIME, ack_by TEXT)",
		"CREATE TABLE IF NOT EXISTS events (id TEXT PRIMARY KEY, type TEXT, source TEXT, payload_json TEXT, created_at DATETIME)",
	}
	for _, stmt := range statements {
		if _, err := d.db.Exec(stmt); err != nil {
			return err
		}
	}
	return nil
}

// Close closes the underlying SQL database connection.
// MUST release all database resources.
// [🛠️semio-repo/server/main.go#Database§Close](semiorepo://definition/semio-repo/server/main.go/DATABASE/CLOSE)
func (d *Database) Close() error {
	return d.db.Close()
}

// insertEvent persists a new event record.
func (d *Database) insertEvent(ctx context.Context, event Event) error {
	_, err := d.db.ExecContext(ctx, "INSERT INTO events (id, type, source, payload_json, created_at) VALUES (?, ?, ?, ?, ?)", event.ID, event.Type, event.Source, event.Payload, event.CreatedAt.UTC())
	return err
}

// upsertTicket inserts or updates a ticket record.
func (d *Database) upsertTicket(ctx context.Context, ticket Ticket) error {
	_, err := d.db.ExecContext(ctx, "INSERT INTO tickets (id, status, title, prompt, summary, llm, ui, author, github_issue, created_at, closed_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) ON CONFLICT(id) DO UPDATE SET status=excluded.status, title=excluded.title, prompt=excluded.prompt, summary=excluded.summary, llm=excluded.llm, ui=excluded.ui, author=excluded.author, github_issue=excluded.github_issue, closed_at=excluded.closed_at", ticket.ID, ticket.Status, ticket.Title, ticket.Prompt, ticket.Summary, ticket.LLM, ticket.Client, ticket.Author, ticket.GitHub, ticket.CreatedAt.UTC(), ticket.ClosedAt)
	return err
}

// listTickets queries tickets optionally filtered by status.
func (d *Database) listTickets(ctx context.Context, status string) ([]Ticket, error) {
	query := "SELECT id, status, title, prompt, summary, llm, ui, author, github_issue, created_at, closed_at FROM tickets"
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
		if err := rows.Scan(&ticket.ID, &ticket.Status, &ticket.Title, &ticket.Prompt, &ticket.Summary, &ticket.LLM, &ticket.Client, &ticket.Author, &ticket.GitHub, &ticket.CreatedAt, &closedAt); err != nil {
			return nil, err
		}
		if closedAt.Valid {
			ticket.ClosedAt = &closedAt.Time
		}
		tickets = append(tickets, ticket)
	}
	return tickets, nil
}

// getTicket retrieves a single ticket by ID.
func (d *Database) getTicket(ctx context.Context, ticketID string) (*Ticket, error) {
	row := d.db.QueryRowContext(ctx, "SELECT id, status, title, prompt, summary, llm, ui, author, github_issue, created_at, closed_at FROM tickets WHERE id = ?", ticketID)
	var ticket Ticket
	var closedAt sql.NullTime
	if err := row.Scan(&ticket.ID, &ticket.Status, &ticket.Title, &ticket.Prompt, &ticket.Summary, &ticket.LLM, &ticket.Client, &ticket.Author, &ticket.GitHub, &ticket.CreatedAt, &closedAt); err != nil {
		return nil, err
	}
	if closedAt.Valid {
		ticket.ClosedAt = &closedAt.Time
	}
	return &ticket, nil
}

// replaceScopes deletes existing scopes for the file and inserts the new ones.
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

// listScopesByFile retrieves all scopes for a given file path.
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

// upsertClaim inserts or updates a ticket-scope claim record.
func (d *Database) upsertClaim(ctx context.Context, ticketID string, scopeID string, claimType string, now time.Time) error {
	_, err := d.db.ExecContext(ctx, "INSERT INTO ticket_claims (ticket_id, scope_id, claim_type, first_seen_at, last_seen_at) VALUES (?, ?, ?, ?, ?) ON CONFLICT(ticket_id, scope_id) DO UPDATE SET claim_type=excluded.claim_type, last_seen_at=excluded.last_seen_at", ticketID, scopeID, claimType, now.UTC(), now.UTC())
	return err
}

// listClaimsByTicket retrieves all scopes claimed by a ticket.
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

// replaceWarnings removes conflict warnings and inserts the new set.
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

// listWarnings retrieves warnings optionally filtered by ticket ID.
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

// listViolations retrieves violations optionally filtered by ticket ID.
func (d *Database) listViolations(ctx context.Context, ticketID string) ([]Violation, error) {
	query := "SELECT id, kind, priority, scope_id, file_path, line, column, summary, excerpt, autofixable, detected_at, ticket_id, resolved_at FROM violations"
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
	var violations []Violation
	for rows.Next() {
		var violation Violation
		var line sql.NullInt64
		var column sql.NullInt64
		var resolved sql.NullTime
		if err := rows.Scan(&violation.ID, &violation.Kind, &violation.Priority, &violation.ScopeID, &violation.FilePath, &line, &column, &violation.Summary, &violation.Excerpt, &violation.Autofix, &violation.DetectedAt, &violation.TicketID, &resolved); err != nil {
			return nil, err
		}
		if line.Valid {
			value := int(line.Int64)
			violation.Line = &value
		}
		if column.Valid {
			value := int(column.Int64)
			violation.Column = &value
		}
		if resolved.Valid {
			violation.ResolvedAt = &resolved.Time
		}
		violations = append(violations, violation)
	}
	return violations, nil
}

// listConflicts finds scopes claimed by more than one open ticket.
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

// #endregion 🔖Database

// #region 🔖EventBus

// [🔖semio-repo/server/main.go#EventBus](semiorepo://section/semio-repo/server/main.go/EVENTBUS)
// Asynchronous in-process event bus for decoupled event publishing and subscription. MUST persist events to the database before dispatching.

// EventHandler is a callback invoked when an event of a subscribed type is published.
// [🛠️semio-repo/server/main.go#EventBus§EventHandler](semiorepo://definition/semio-repo/server/main.go/EVENTBUS/EVENTHANDLER)
type EventHandler func(context.Context, Event)

// EventBus is a buffered channel-based event dispatcher with persistent storage.
// [🛠️semio-repo/server/main.go#EventBus§EventBus](semiorepo://definition/semio-repo/server/main.go/EVENTBUS/EVENTBUS)
type EventBus struct {
	ch       chan Event
	handlers map[string][]EventHandler
	db       *Database
	ctx      context.Context
	cancel   context.CancelFunc
	wg       sync.WaitGroup
}

// NewEventBus creates a new event bus backed by the given database.
// MUST initialize the channel buffer to 256 and create a cancellable context.
// [🛠️semio-repo/server/main.go#EventBus§NewEventBus](semiorepo://definition/semio-repo/server/main.go/EVENTBUS/NEWEVENTBUS)
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

// Subscribe registers a handler for the given event type.
// MUST append the handler to the handlers map.
// [🛠️semio-repo/server/main.go#EventBus§Subscribe](semiorepo://definition/semio-repo/server/main.go/EVENTBUS/SUBSCRIBE)
func (b *EventBus) Subscribe(eventType string, handler EventHandler) {
	b.handlers[eventType] = append(b.handlers[eventType], handler)
}

// Publish persists an event and dispatches it to subscribers.
// MUST store the event in the database before sending to the channel.
// [🛠️semio-repo/server/main.go#EventBus§Publish](semiorepo://definition/semio-repo/server/main.go/EVENTBUS/PUBLISH)
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

// Start launches the event dispatch goroutine.
// MUST consume events from the channel and invoke registered handlers.
// [🛠️semio-repo/server/main.go#EventBus§Start](semiorepo://definition/semio-repo/server/main.go/EVENTBUS/START)
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

// Stop cancels the event bus context and waits for the dispatch goroutine to finish.
// MUST block until the goroutine exits.
// [🛠️semio-repo/server/main.go#EventBus§Stop](semiorepo://definition/semio-repo/server/main.go/EVENTBUS/STOP)
func (b *EventBus) Stop() {
	b.cancel()
	b.wg.Wait()
}

// #endregion 🔖EventBus

// #region 🔖DiffParsing

// [🔖semio-repo/server/main.go#DiffParsing](semiorepo://section/semio-repo/server/main.go/DIFFPARSING)
// Unified diff parser that extracts file paths and hunk line ranges from patch text. MUST handle standard git diff output format.

// hunkHeader is a regex pattern matching unified diff hunk headers.
var hunkHeader = regexp.MustCompile(`@@ -([0-9]+)(?:,([0-9]+))? \+([0-9]+)(?:,([0-9]+))? @@`)

// parseUnifiedDiff extracts file paths and hunk ranges from a unified diff patch.
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
				current.Created = true
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

// parseHunkInt parses a hunk header integer value.
func parseHunkInt(value string) int {
	parsed, _ := strconv.Atoi(value)
	return parsed
}

// parseHunkIntWithDefault parses a hunk header integer or returns the fallback.
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

// #endregion 🔖DiffParsing

// #region 🔖Indexing

// [🔖semio-repo/server/main.go#Indexing](semiorepo://section/semio-repo/server/main.go/INDEXING)
// Source code indexer that parses files into scopes covering files, sections, and definitions. MUST support region-marker-based sections and language-specific definition patterns.

// IndexCache holds in-memory caches of indexed scopes partitioned by file path.
// [🛠️semio-repo/server/main.go#Indexing§IndexCache](semiorepo://definition/semio-repo/server/main.go/INDEXING/INDEXCACHE)
type IndexCache struct {
	Sections    map[string][]Scope
	Definitions map[string][]Scope
	Files       map[string]Scope
}

// newIndexCache creates an empty IndexCache with initialized maps.
func newIndexCache() IndexCache {
	return IndexCache{
		Sections:    map[string][]Scope{},
		Definitions: map[string][]Scope{},
		Files:       map[string]Scope{},
	}
}

// buildScopesForFile parses a file into file, section, and definition scopes.
func buildScopesForFile(path string, content string) []Scope {
	lines := strings.Split(content, "\n")
	now := time.Now().UTC()
	var scopes []Scope
	fileScope := Scope{
		ID:        fmt.Sprintf("file:%s", path),
		Kind:      "file",
		FilePath:  path,
		StartLine: 1,
		EndLine:   len(lines),
		UpdatedAt: now,
	}
	scopes = append(scopes, fileScope)
	ext := strings.ToLower(filepath.Ext(path))
	sections := parseSections(lines, ext)
	definitions := parseDefinitions(lines, ext)
	for i := range sections {
		sections[i].FilePath = path
		sections[i].ID = fmt.Sprintf("section:%s#%s", path, sections[i].SectionPath)
	}
	for _, section := range sections {
		section.UpdatedAt = now
		scopes = append(scopes, section)
	}
	sectionByLine := map[int]string{}
	for _, section := range sections {
		for line := section.StartLine; line <= section.EndLine; line++ {
			sectionByLine[line] = section.SectionPath
		}
	}
	for i := range definitions {
		definitions[i].FilePath = path
		definitions[i].ID = fmt.Sprintf("def:%s#%s", path, definitions[i].Definition)
		def := definitions[i]
		def.SectionPath = sectionByLine[def.StartLine]
		def.UpdatedAt = now
		scopes = append(scopes, def)
	}
	return scopes
}

// parseSections extracts section scopes from region markers and markdown headings.
func parseSections(lines []string, ext string) []Scope {
	var scopes []Scope
	type sectionFrame struct {
		Name      string
		StartLine int
		Level     int
		Path      string
	}
	var stack []sectionFrame
	for index, line := range lines {
		lineNumber := index + 1
		if name, ok, isEnd := parseRegionMarker(line); ok {
			if isEnd {
				if len(stack) > 0 {
					frame := stack[len(stack)-1]
					stack = stack[:len(stack)-1]
					scope := Scope{
						ID:          fmt.Sprintf("section:%s#%s", "", frame.Path),
						Kind:        "section",
						SectionPath: frame.Path,
						StartLine:   frame.StartLine,
						EndLine:     lineNumber - 1,
					}
					scopes = append(scopes, scope)
				}
			} else {
				path := name
				if len(stack) > 0 {
					path = stack[len(stack)-1].Path + "." + name
				}
				stack = append(stack, sectionFrame{Name: name, StartLine: lineNumber, Level: 0, Path: path})
			}
			continue
		}
		if ext == ".md" || ext == ".mdx" {
			if level, title := parseMarkdownHeading(line); level > 0 {
				for len(stack) > 0 && stack[len(stack)-1].Level >= level {
					frame := stack[len(stack)-1]
					stack = stack[:len(stack)-1]
					scope := Scope{
						ID:          fmt.Sprintf("section:%s#%s", "", frame.Path),
						Kind:        "section",
						SectionPath: frame.Path,
						StartLine:   frame.StartLine,
						EndLine:     lineNumber - 1,
					}
					scopes = append(scopes, scope)
				}
				path := title
				if len(stack) > 0 {
					path = stack[len(stack)-1].Path + "." + title
				}
				stack = append(stack, sectionFrame{Name: title, StartLine: lineNumber, Level: level, Path: path})
			}
		}
	}
	for _, frame := range stack {
		scope := Scope{
			ID:          fmt.Sprintf("section:%s#%s", "", frame.Path),
			Kind:        "section",
			SectionPath: frame.Path,
			StartLine:   frame.StartLine,
			EndLine:     len(lines),
		}
		scopes = append(scopes, scope)
	}
	return scopes
}

// parseRegionMarker detects region start/end markers in a line.
func parseRegionMarker(line string) (string, bool, bool) {
	trimmed := strings.TrimSpace(line)
	trimmed = strings.TrimPrefix(trimmed, "//")
	trimmed = strings.TrimPrefix(trimmed, "#")
	trimmed = strings.TrimPrefix(trimmed, "/*")
	trimmed = strings.TrimSuffix(trimmed, "*/")
	trimmed = strings.TrimSpace(trimmed)
	if strings.HasPrefix(trimmed, "#region 🔖") {
		return strings.TrimSpace(strings.TrimPrefix(trimmed, "#region 🔖")), true, false
	}
	if strings.HasPrefix(trimmed, "#endregion 🔖") {
		return strings.TrimSpace(strings.TrimPrefix(trimmed, "#endregion 🔖")), true, true
	}
	return "", false, false
}

// parseMarkdownHeading parses a markdown heading line into level and title.
func parseMarkdownHeading(line string) (int, string) {
	trimmed := strings.TrimSpace(line)
	if !strings.HasPrefix(trimmed, "#") {
		return 0, ""
	}
	level := 0
	for level < len(trimmed) && trimmed[level] == '#' {
		level++
	}
	if level == 0 || level > 6 {
		return 0, ""
	}
	name := strings.TrimSpace(trimmed[level:])
	if name == "" {
		return 0, ""
	}
	return level, name
}

// assignSectionPaths updates section IDs to include the file path.
func assignSectionPaths(sections []Scope) []Scope {
	for i := range sections {
		sections[i].ID = fmt.Sprintf("section:%s#%s", sections[i].FilePath, sections[i].SectionPath)
	}
	return sections
}

// parseDefinitions extracts definition scopes using language-specific patterns.
func parseDefinitions(lines []string, ext string) []Scope {
	var scopes []Scope
	patterns := definitionPatterns(ext)
	for index, line := range lines {
		lineNumber := index + 1
		for _, pattern := range patterns {
			matches := pattern.FindStringSubmatch(line)
			if len(matches) > 1 {
				name := matches[len(matches)-1]
				scopes = append(scopes, Scope{
					ID:         fmt.Sprintf("def:%s#%s", "", name),
					Kind:       "definition",
					Definition: name,
					StartLine:  lineNumber,
					EndLine:    lineNumber,
				})
				break
			}
		}
	}
	return scopes
}

// definitionPatterns returns language-specific regex patterns for extracting definitions.
func definitionPatterns(ext string) []*regexp.Regexp {
	switch ext {
	case ".go":
		return []*regexp.Regexp{
			regexp.MustCompile(`^\s*func\s+(?:\([^\)]*\)\s*)?([A-Za-z0-9_]+)`),
			regexp.MustCompile(`^\s*type\s+([A-Za-z0-9_]+)`),
			regexp.MustCompile(`^\s*var\s+([A-Za-z0-9_]+)`),
			regexp.MustCompile(`^\s*const\s+([A-Za-z0-9_]+)`),
		}
	case ".ts", ".tsx", ".js", ".jsx":
		return []*regexp.Regexp{
			regexp.MustCompile(`^\s*(?:export\s+)?(?:async\s+)?function\s+([A-Za-z0-9_]+)`),
			regexp.MustCompile(`^\s*(?:export\s+)?class\s+([A-Za-z0-9_]+)`),
			regexp.MustCompile(`^\s*(?:export\s+)?interface\s+([A-Za-z0-9_]+)`),
			regexp.MustCompile(`^\s*(?:export\s+)?type\s+([A-Za-z0-9_]+)`),
		}
	case ".py":
		return []*regexp.Regexp{
			regexp.MustCompile(`^\s*def\s+([A-Za-z0-9_]+)`),
			regexp.MustCompile(`^\s*class\s+([A-Za-z0-9_]+)`),
		}
	case ".cs":
		return []*regexp.Regexp{
			regexp.MustCompile(`^\s*(?:public|private|protected|internal)?\s*(?:static\s+)?(?:class|struct|interface|enum|record)\s+([A-Za-z0-9_]+)`),
		}
	case ".md", ".mdx":
		return []*regexp.Regexp{}
	default:
		return []*regexp.Regexp{}
	}
}

// #endregion 🔖Indexing

// #region 🔖Claims

// [🔖semio-repo/server/main.go#Claims](semiorepo://section/semio-repo/server/main.go/CLAIMS)
// Scope claim mapping logic that associates diff hunks with overlapping scopes. MUST detect multi-ticket conflicts.

// mapClaims maps diff hunks to overlapping scopes and returns claimed IDs.
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
						claimedIDs = appendIfMissing(claimedIDs, scope.ID)
					}
				}
			}
		}
	}
	sort.Strings(claimedIDs)
	return claimedIDs, claimed
}

// filterScopesByFile returns scopes matching the given file path.
func filterScopesByFile(scopes []Scope, filePath string) []Scope {
	var filtered []Scope
	for _, scope := range scopes {
		if scope.FilePath == filePath {
			filtered = append(filtered, scope)
		}
	}
	return filtered
}

// rangesOverlap tests whether two line ranges overlap.
func rangesOverlap(a LineRange, b LineRange) bool {
	if a.Start == 0 || b.Start == 0 {
		return false
	}
	return a.Start <= b.End && b.Start <= a.End
}

// appendIfMissing appends a string to a slice only if it is not already present.
func appendIfMissing(list []string, value string) []string {
	for _, item := range list {
		if item == value {
			return list
		}
	}
	return append(list, value)
}

// #endregion 🔖Claims

// #region 🔖Warnings

// [🔖semio-repo/server/main.go#Warnings](semiorepo://section/semio-repo/server/main.go/WARNINGS)
// Conflict warning generation from multi-ticket scope overlaps. MUST produce error-severity warnings for blocking conflicts.

// buildConflictWarnings creates warning records from detected scope conflicts.
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

// #endregion 🔖Warnings

// #region 🔖Server

// [🔖semio-repo/server/main.go#Server](semiorepo://section/semio-repo/server/main.go/SERVER)
// HTTP server with ticket lifecycle, diff ingestion, pre-commit checks, indexing, and webhook endpoints. MUST enforce authentication on mutating routes.

// Server is the main HTTP server holding configuration, database, event bus, and caches.
// [🛠️semio-repo/server/main.go#Server§Server](semiorepo://definition/semio-repo/server/main.go/SERVER/SERVER)
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

// NewServer creates a new Server with the given config, database, and event bus.
// MUST initialize the index cache and GitHub comment cache.
// [🛠️semio-repo/server/main.go#Server§NewServer](semiorepo://definition/semio-repo/server/main.go/SERVER/NEWSERVER)
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

// newRequestContext creates a request-scoped context with a 15-second timeout.
func (s *Server) newRequestContext(r *http.Request) (context.Context, context.CancelFunc) {
	return context.WithTimeout(r.Context(), 15*time.Second)
}

// requireAuth checks the bearer token against the configured server token.
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

// decodeJSON reads and decodes a JSON request body with size limits.
func (s *Server) decodeJSON(r *http.Request, payload interface{}) error {
	decoder := json.NewDecoder(io.LimitReader(r.Body, s.config.RequestBodyLimit))
	decoder.DisallowUnknownFields()
	return decoder.Decode(payload)
}

// writeJSON writes a JSON response with the given status code.
func (s *Server) writeJSON(w http.ResponseWriter, status int, payload interface{}) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(status)
	_ = json.NewEncoder(w).Encode(payload)
}

// respondError writes a JSON error response.
func (s *Server) respondError(w http.ResponseWriter, status int, message string) {
	s.writeJSON(w, status, map[string]string{"error": message})
}

// handleHealth responds with 200 OK for liveness checks.
func (s *Server) handleHealth(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	w.WriteHeader(http.StatusOK)
	_, _ = w.Write([]byte("ok"))
}

// handleTicketOpen creates a new ticket from the request payload.
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
		Client:        payload.Client,
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

// handleTicketClose closes an existing ticket with a summary.
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

// handleTicketReopen reopens a closed ticket with a new prompt.
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

// handleTicketsQuery lists tickets optionally filtered by status.
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

// handleTicketDetail returns a single ticket by its path-extracted ID.
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

// handleTicketClaims returns scope claims for a ticket.
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

// handleDiffIngest ingests a diff patch, indexes changed files, maps claims, and returns results.
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
	result, warnings, violations, err := s.processDiff(ctx, payload.TicketID, payload.Patch, payload.Snapshots)
	if err != nil {
		s.respondError(w, http.StatusInternalServerError, err.Error())
		return
	}
	response := DiffIngestResponse{
		ChangedFiles:  result.ChangedFiles,
		ClaimedScopes: result.ClaimedScopes,
		Warnings:      warnings,
		Violations:    violations,
		Blockers:      result.Blockers,
	}
	s.writeJSON(w, http.StatusOK, response)
}

// handlePrecommit runs a pre-commit check against a diff patch.
func (s *Server) handlePrecommit(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	if !s.requireAuth(r) {
		s.respondError(w, http.StatusUnauthorized, "unauthorized")
		return
	}
	var payload PrecommitRequest
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
	result, warnings, violations, err := s.processDiff(ctx, payload.TicketID, payload.Patch, nil)
	if err != nil {
		s.respondError(w, http.StatusInternalServerError, err.Error())
		return
	}
	blockers := result.Blockers
	ok := len(blockers) == 0
	response := PrecommitResponse{
		OK:           ok,
		Blockers:     blockers,
		Warnings:     warnings,
		Violations:   violations,
		AutofixPatch: "",
	}
	s.writeJSON(w, http.StatusOK, response)
}

// handleReindex walks the repo and re-indexes all files.
func (s *Server) handleReindex(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	if !s.requireAuth(r) {
		s.respondError(w, http.StatusUnauthorized, "unauthorized")
		return
	}
	files, err := s.walkRepoFiles()
	if err != nil {
		s.respondError(w, http.StatusInternalServerError, err.Error())
		return
	}
	ctx, cancel := s.newRequestContext(r)
	defer cancel()
	for _, file := range files {
		content, err := os.ReadFile(filepath.Join(s.config.RepoRoot, file))
		if err != nil {
			continue
		}
		s.updateIndexForFile(ctx, file, string(content))
	}
	s.writeJSON(w, http.StatusOK, map[string]int{"files": len(files)})
}

// handleIndexFile indexes a single file from the request payload.
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

// handleWarnings returns warnings optionally filtered by ticket ID.
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

// handleViolations returns violations optionally filtered by ticket ID.
func (s *Server) handleViolations(w http.ResponseWriter, r *http.Request) {
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
	violations, err := s.db.listViolations(ctx, r.URL.Query().Get("ticket_id"))
	if err != nil {
		s.respondError(w, http.StatusInternalServerError, err.Error())
		return
	}
	s.writeJSON(w, http.StatusOK, violations)
}

// handleScopes returns scopes for a given file query parameter.
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

// #endregion 🔖Server

// #region 🔖Processing

// [🔖semio-repo/server/main.go#Processing](semiorepo://section/semio-repo/server/main.go/PROCESSING)
// Diff processing pipeline that indexes changed files, maps claims, detects conflicts, and produces warnings. MUST be transactional per request.

// ProcessResult holds the outcome of a diff processing operation.
// [🛠️semio-repo/server/main.go#Processing§ProcessResult](semiorepo://definition/semio-repo/server/main.go/PROCESSING/PROCESSRESULT)
type ProcessResult struct {
	ChangedFiles  []string
	ClaimedScopes []string
	Blockers      []string
}

// processDiff parses the patch, indexes changed files, maps claims, and detects conflicts.
// MUST return warnings and violations alongside the processing result.
func (s *Server) processDiff(ctx context.Context, ticketID string, patch string, snapshots []FileSnapshot) (ProcessResult, []Warning, []Violation, error) {
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
	claimedIDs, _ := mapClaims(scopes, diff)
	now := time.Now().UTC()
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
	return result, warnings, []Violation{}, nil
}

// uniqueFiles extracts deduplicated file paths from a diff result.
func uniqueFiles(files []DiffFile) []string {
	var list []string
	for _, file := range files {
		if file.Path != "" {
			list = appendIfMissing(list, file.Path)
		}
	}
	return list
}

// snapshotMap converts a slice of file snapshots into a path-to-content map.
func snapshotMap(snapshots []FileSnapshot) map[string]string {
	mapping := map[string]string{}
	for _, snapshot := range snapshots {
		mapping[snapshot.Path] = snapshot.Content
	}
	return mapping
}

// updateIndexForFile builds scopes from file content and updates both the database and cache.
func (s *Server) updateIndexForFile(ctx context.Context, filePath string, content string) {
	scopes := buildScopesForFile(filePath, content)
	var fileScope Scope
	var sections []Scope
	var definitions []Scope
	for _, scope := range scopes {
		scope.FilePath = filePath
		scope.ID = buildScopeID(scope)
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

// buildScopeID generates a deterministic scope ID from the scope's kind and path.
func buildScopeID(scope Scope) string {
	if scope.Kind == "file" {
		return fmt.Sprintf("file:%s", scope.FilePath)
	}
	if scope.Kind == "section" {
		return fmt.Sprintf("section:%s#%s", scope.FilePath, scope.SectionPath)
	}
	if scope.SectionPath != "" {
		return fmt.Sprintf("def:%s#%s::%s", scope.FilePath, scope.SectionPath, scope.Definition)
	}
	return fmt.Sprintf("def:%s#%s", scope.FilePath, scope.Definition)
}

// walkRepoFiles walks the repo root and returns all non-hidden file paths.
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

// #endregion 🔖Processing

// #region 🔖Webhooks

// [🔖semio-repo/server/main.go#Webhooks](semiorepo://section/semio-repo/server/main.go/WEBHOOKS)
// GitHub webhook handlers for issue comment caching and issue event processing. MUST verify HMAC signatures when a secret is configured.

// GitHubComment stores a cached GitHub issue comment for correlating close/reopen events.
// [🛠️semio-repo/server/main.go#Webhooks§GitHubComment](semiorepo://definition/semio-repo/server/main.go/WEBHOOKS/GITHUBCOMMENT)
type GitHubComment struct {
	Body      string
	Actor     string
	Repo      string
	Issue     int
	Timestamp time.Time
}

// handleGitHubWebhook processes incoming GitHub webhook events.
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
	w.WriteHeader(http.StatusOK)
}

// verifyGitHubSignature validates the HMAC-SHA256 signature of a webhook payload.
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

// cacheGitHubComment stores a GitHub comment for correlating subsequent events.
func (s *Server) cacheGitHubComment(payload map[string]interface{}) {
	issue, repo, actor, body := extractIssueComment(payload)
	if issue == 0 || repo == "" || actor == "" || body == "" {
		return
	}
	key := fmt.Sprintf("%s#%d#%s", repo, issue, actor)
	s.ghLock.Lock()
	s.githubCache[key] = GitHubComment{
		Body:      body,
		Actor:     actor,
		Repo:      repo,
		Issue:     issue,
		Timestamp: time.Now().UTC(),
	}
	s.ghLock.Unlock()
}

// handleGitHubIssueEvent processes GitHub issue close/reopen events.
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

// findCachedComment retrieves a recently cached GitHub comment for the given issue.
func (s *Server) findCachedComment(repo string, issue int, actor string) GitHubComment {
	key := fmt.Sprintf("%s#%d#%s", repo, issue, actor)
	s.ghLock.Lock()
	defer s.ghLock.Unlock()
	comment, ok := s.githubCache[key]
	if !ok {
		return GitHubComment{}
	}
	if time.Since(comment.Timestamp) > 90*time.Second {
		delete(s.githubCache, key)
		return GitHubComment{}
	}
	return comment
}

// extractIssueComment extracts issue number, repo, actor, and body from a webhook payload.
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

// extractIssueNumber extracts the issue number from a GitHub webhook payload.
func extractIssueNumber(payload map[string]interface{}) int {
	if issue, ok := payload["issue"].(map[string]interface{}); ok {
		if number, ok := issue["number"].(float64); ok {
			return int(number)
		}
	}
	return 0
}

// extractRepoFullName extracts the repository full name from a GitHub webhook payload.
func extractRepoFullName(payload map[string]interface{}) string {
	if repo, ok := payload["repository"].(map[string]interface{}); ok {
		if name, ok := repo["full_name"].(string); ok {
			return name
		}
	}
	return ""
}

// extractActorLogin extracts the sender login from a GitHub webhook payload.
func extractActorLogin(payload map[string]interface{}) string {
	if sender, ok := payload["sender"].(map[string]interface{}); ok {
		if login, ok := sender["login"].(string); ok {
			return login
		}
	}
	return ""
}

// #endregion 🔖Webhooks

// #region 🔖Discord

// [🔖semio-repo/server/main.go#Discord](semiorepo://section/semio-repo/server/main.go/DISCORD)
// Discord notification integration for ticket lifecycle events. MUST silently skip when no webhook URL is configured.

// notifyDiscord sends a message to the configured Discord webhook.
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

// registerNotifications subscribes to ticket lifecycle events and sends Discord notifications.
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
}

// #endregion 🔖Discord

// #region 🔖Utilities

// [🔖semio-repo/server/main.go#Utilities](semiorepo://section/semio-repo/server/main.go/UTILITIES)
// Shared utility functions used across the server. MUST produce unique identifiers.

// newID generates a unique identifier from the current timestamp and a random value.
func newID() string {
	return fmt.Sprintf("%d-%d", time.Now().UTC().UnixNano(), rand.Int63())
}

// #endregion 🔖Utilities

// #region 🔖Main

// [🔖semio-repo/server/main.go#Main](semiorepo://section/semio-repo/server/main.go/MAIN)
// Application entry point that initializes the database, event bus, server, and HTTP routes. MUST register all handlers before listening.

// main initializes the server and starts listening for HTTP requests.
// MUST open the database, start the event bus, register all routes, and block on ListenAndServe.
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
	mux.HandleFunc("/checks/precommit", server.handlePrecommit)
	mux.HandleFunc("/repo/reindex", server.handleReindex)
	mux.HandleFunc("/repo/index-file", server.handleIndexFile)
	mux.HandleFunc("/warnings", server.handleWarnings)
	mux.HandleFunc("/violations", server.handleViolations)
	mux.HandleFunc("/scopes", server.handleScopes)
	mux.HandleFunc("/webhooks/github", server.handleGitHubWebhook)
	log.Printf("semio repo server listening on %s", config.Address)
	if err := http.ListenAndServe(config.Address, mux); err != nil {
		log.Fatal(err)
	}
}

// #endregion 🔖Main
