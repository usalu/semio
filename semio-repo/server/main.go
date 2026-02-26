// #region 🔖Header
// [🧰semiorepo⌨️server💻main](semiorepo://p/i/semio-repo/b/b/server/f/main.go)

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
// [🧰semiorepo⌨️server💻main🔖package](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Package)
// Package declaration for the semio repo server binary. MUST be package main.
package main

// #endregion 🔖Package

// #region 🔖Imports
// [🧰semiorepo⌨️server💻main🔖imports](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Imports)
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

	repopkg "github.com/usalu/semio/semio-repo/go"
	_ "modernc.org/sqlite"
)

// #endregion 🔖Imports

// #region 🔖Config
// [🧰semiorepo⌨️server💻main🔖config](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Config)
// Server configuration loading from environment variables. MUST provide sensible defaults.

// Config holds all server configuration values.
// [🧰semiorepo⌨️server💻main🔖config✂️config](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Config/d/i/Config)
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
// [🧰semiorepo⌨️server💻main🔖config🛠️loadconfig](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Config/d/i/loadConfig)
// loadConfig MUST perform the loadConfig operation.
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
// [🧰semiorepo⌨️server💻main🔖config🛠️envordefault](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Config/d/i/envOrDefault)
// envOrDefault MUST perform the envOrDefault operation.
func envOrDefault(key, fallback string) string {
	if value := strings.TrimSpace(os.Getenv(key)); value != "" {
		return value
	}
	return fallback
}

// envOrDefaultInt64 returns the parsed int64 environment variable or the fallback.
// [🧰semiorepo⌨️server💻main🔖config🛠️envordefaultint64](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Config/d/i/envOrDefaultInt64)
// envOrDefaultInt64 MUST perform the envOrDefaultInt64 operation.
func envOrDefaultInt64(key string, fallback int64) int64 {
	if value := strings.TrimSpace(os.Getenv(key)); value != "" {
		if parsed, err := strconv.ParseInt(value, 10, 64); err == nil {
			return parsed
		}
	}
}

// #endregion 🔖Config

// #region 🔖Models
// [🧰semiorepo⌨️server💻main🔖models](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Models)
// Data model types for tickets, scopes, warnings, breachs, events, and API request/response payloads. MUST mirror the server SQLite schema.

// Ticket represents a tracked work item with lifecycle status.
// [🧰semiorepo⌨️server💻main🔖models✂️ticket](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Models/d/i/Ticket)
type Ticket struct {
	ID        string     `json:"id"`
	Status    string     `json:"status"`
	Title     string     `json:"title"`
	Prompt    string     `json:"prompt"`
	Summary   string     `json:"summary"`
	LLM       string     `json:"llm"`
	Client    string     `json:"client"`
	Author    string     `json:"author"`
	GitHub    string     `json:"github_issue"`
	CreatedAt time.Time  `json:"created_at"`
	ClosedAt  *time.Time `json:"closed_at"`
}

// Scope represents a code region (file, section, or definition) with line range.
// [🧰semiorepo⌨️server💻main🔖models✂️scope](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Models/d/i/Scope)
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
// [🧰semiorepo⌨️server💻main🔖models✂️warning](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Models/d/i/Warning)
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

// Breach represents a policy breach detected in source code.
// [🧰semiorepo⌨️server💻main🔖models✂️breach](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Models/d/i/Breach)
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

// Event represents a system event persisted to the event log.
// [🧰semiorepo⌨️server💻main🔖models✂️event](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Models/d/i/Event)
type Event struct {
	ID        string    `json:"id"`
	Type      string    `json:"type"`
	Source    string    `json:"source"`
	Payload   string    `json:"payload_json"`
	CreatedAt time.Time `json:"created_at"`
}

// LineRange represents a contiguous range of line numbers.
// [🧰semiorepo⌨️server💻main🔖models✂️linerange](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Models/d/i/LineRange)
type LineRange struct {
	Start int
	End   int
}

// DiffHunk represents a single hunk with old and new line ranges from a unified diff.
// [🧰semiorepo⌨️server💻main🔖models✂️diffhunk](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Models/d/i/DiffHunk)
type DiffHunk struct {
	OldRange LineRange
	NewRange LineRange
}

// DiffFile represents a single file entry in a unified diff with its hunks.
// [🧰semiorepo⌨️server💻main🔖models✂️difffile](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Models/d/i/DiffFile)
type DiffFile struct {
	Path    string
	Hunks   []DiffHunk
	Deleted bool
	Created bool
}

// DiffResult aggregates all parsed diff files from a patch.
// [🧰semiorepo⌨️server💻main🔖models✂️diffresult](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Models/d/i/DiffResult)
type DiffResult struct {
	Files []DiffFile
}

// FileSnapshot holds the full content of a file for snapshot-based indexing.
// [🧰semiorepo⌨️server💻main🔖models✂️filesnapshot](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Models/d/i/FileSnapshot)
type FileSnapshot struct {
	Path    string `json:"path"`
	Content string `json:"content"`
}

// TicketOpenRequest is the JSON payload for opening a new ticket.
// [🧰semiorepo⌨️server💻main🔖models✂️ticketopenrequest](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Models/d/i/TicketOpenRequest)
type TicketOpenRequest struct {
	TicketID    string `json:"ticket_id"`
	Title       string `json:"title"`
	Prompt      string `json:"prompt"`
	LLM         string `json:"llm"`
	Client      string `json:"client"`
	Author      string `json:"author"`
	GitHubIssue string `json:"github_issue"`
}

// TicketCloseRequest is the JSON payload for closing a ticket.
// [🧰semiorepo⌨️server💻main🔖models✂️ticketcloserequest](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Models/d/i/TicketCloseRequest)
type TicketCloseRequest struct {
	TicketID string   `json:"ticket_id"`
	Summary  string   `json:"summary"`
	Files    []string `json:"files"`
}

// TicketReopenRequest is the JSON payload for reopening a closed ticket.
// [🧰semiorepo⌨️server💻main🔖models✂️ticketreopenrequest](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Models/d/i/TicketReopenRequest)
type TicketReopenRequest struct {
	TicketID string `json:"ticket_id"`
	Prompt   string `json:"prompt"`
	LLM      string `json:"llm"`
	Title    string `json:"title"`
}

// DiffIngestRequest is the JSON payload for ingesting a diff patch.
// [🧰semiorepo⌨️server💻main🔖models✂️diffingestrequest](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Models/d/i/DiffIngestRequest)
type DiffIngestRequest struct {
	TicketID  string         `json:"ticket_id"`
	RepoID    string         `json:"repo_id"`
	Patch     string         `json:"patch"`
	Snapshots []FileSnapshot `json:"snapshots"`
}

// DiffIngestResponse holds the results of a diff ingestion operation.
// [🧰semiorepo⌨️server💻main🔖models✂️diffingestresponse](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Models/d/i/DiffIngestResponse)
type DiffIngestResponse struct {
	ChangedFiles  []string  `json:"changed_files"`
	ClaimedScopes []string  `json:"claimed_scopes"`
	Warnings      []Warning `json:"warnings"`
	Breachs       []Breach  `json:"breachs"`
	Blockers      []string  `json:"blockers"`
}

// PrecheckpointRequest is the JSON payload for a pre-checkpoint check.
// [🧰semiorepo⌨️server💻main🔖models✂️precheckpointrequest](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Models/d/i/PrecheckpointRequest)
type PrecheckpointRequest struct {
	TicketID      string `json:"ticket_id"`
	Patch         string `json:"patch"`
	Staged        bool   `json:"staged"`
	CheckpointMessage string `json:"checkpoint_message"`
}

// PrecheckpointResponse holds the result of a pre-checkpoint check.
// [🧰semiorepo⌨️server💻main🔖models✂️precheckpointresponse](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Models/d/i/PrecheckpointResponse)
type PrecheckpointResponse struct {
	OK           bool      `json:"ok"`
	Blockers     []string  `json:"blockers"`
	Warnings     []Warning `json:"warnings"`
	Breachs      []Breach  `json:"breachs"`
	AutofixPatch string    `json:"autofix_patch"`
}

// IndexFileRequest is the JSON payload for indexing a single file.
// [🧰semiorepo⌨️server💻main🔖models✂️indexfilerequest](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Models/d/i/IndexFileRequest)
type IndexFileRequest struct {
	FilePath string `json:"file_path"`
}

// #endregion 🔖Models

// #region 🔖Database
// [🧰semiorepo⌨️server💻main🔖database](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Database)
// SQLite database layer for persistent storage of tickets, scopes, claims, warnings, breachs, and events. MUST use WAL journal mode.

// Database wraps a sql.DB connection to the SQLite store.
// [🧰semiorepo⌨️server💻main🔖database✂️database](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Database/d/i/Database)
type Database struct {
	db *sql.DB
}

// openDatabase opens an SQLite database and runs schema migrations.
// MUST enable WAL journal mode and foreign keys.
// [🧰semiorepo⌨️server💻main🔖database🛠️opendatabase](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Database/d/i/openDatabase)
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
// [🧰semiorepo⌨️server💻main🔖database🛠️migrate](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Database/d/i/migrate)
// migrate MUST perform the migrate operation.
func (d *Database) migrate() error {
	statements := []string{
		"CREATE TABLE IF NOT EXISTS repos (id TEXT PRIMARY KEY, name TEXT, path TEXT, created_at DATETIME)",
		"CREATE TABLE IF NOT EXISTS tickets (id TEXT PRIMARY KEY, status TEXT, title TEXT, prompt TEXT, summary TEXT, llm TEXT, ui TEXT, author TEXT, github_issue TEXT, created_at DATETIME, closed_at DATETIME)",
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

// Close closes the underlying SQL database connection.
// MUST release all database resources.
// [🧰semiorepo⌨️server💻main🔖database🛠️close](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Database/d/i/Close)
func (d *Database) Close() error {
	return d.db.Close()
}

// insertEvent persists a new event record.
// [🧰semiorepo⌨️server💻main🔖database🛠️insertevent](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Database/d/i/insertEvent)
// insertEvent MUST perform the insertEvent operation.
func (d *Database) insertEvent(ctx context.Context, event Event) error {
	_, err := d.db.ExecContext(ctx, "INSERT INTO events (id, type, source, payload_json, created_at) VALUES (?, ?, ?, ?, ?)", event.ID, event.Type, event.Source, event.Payload, event.CreatedAt.UTC())
	return err
}

// upsertTicket inserts or updates a ticket record.
// [🧰semiorepo⌨️server💻main🔖database🛠️upsertticket](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Database/d/i/upsertTicket)
// upsertTicket MUST perform the upsertTicket operation.
func (d *Database) upsertTicket(ctx context.Context, ticket Ticket) error {
	_, err := d.db.ExecContext(ctx, "INSERT INTO tickets (id, status, title, prompt, summary, llm, ui, author, github_issue, created_at, closed_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) ON CONFLICT(id) DO UPDATE SET status=excluded.status, title=excluded.title, prompt=excluded.prompt, summary=excluded.summary, llm=excluded.llm, ui=excluded.ui, author=excluded.author, github_issue=excluded.github_issue, closed_at=excluded.closed_at", ticket.ID, ticket.Status, ticket.Title, ticket.Prompt, ticket.Summary, ticket.LLM, ticket.Client, ticket.Author, ticket.GitHub, ticket.CreatedAt.UTC(), ticket.ClosedAt)
	return err
}

// listTickets queries tickets optionally filtered by status.
// [🧰semiorepo⌨️server💻main🔖database🛠️listtickets](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Database/d/i/listTickets)
// listTickets MUST perform the listTickets operation.
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
// [🧰semiorepo⌨️server💻main🔖database🛠️getticket](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Database/d/i/getTicket)
// getTicket MUST perform the getTicket operation.
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
// [🧰semiorepo⌨️server💻main🔖database🛠️replacescopes](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Database/d/i/replaceScopes)
// replaceScopes MUST perform the replaceScopes operation.
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
// [🧰semiorepo⌨️server💻main🔖database🛠️listscopesbyfile](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Database/d/i/listScopesByFile)
// listScopesByFile MUST perform the listScopesByFile operation.
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
// [🧰semiorepo⌨️server💻main🔖database🛠️upsertclaim](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Database/d/i/upsertClaim)
// upsertClaim MUST perform the upsertClaim operation.
func (d *Database) upsertClaim(ctx context.Context, ticketID string, scopeID string, claimType string, now time.Time) error {
	_, err := d.db.ExecContext(ctx, "INSERT INTO ticket_claims (ticket_id, scope_id, claim_type, first_seen_at, last_seen_at) VALUES (?, ?, ?, ?, ?) ON CONFLICT(ticket_id, scope_id) DO UPDATE SET claim_type=excluded.claim_type, last_seen_at=excluded.last_seen_at", ticketID, scopeID, claimType, now.UTC(), now.UTC())
	return err
}

// listClaimsByTicket retrieves all scopes claimed by a ticket.
// [🧰semiorepo⌨️server💻main🔖database🛠️listclaimsbyticket](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Database/d/i/listClaimsByTicket)
// listClaimsByTicket MUST perform the listClaimsByTicket operation.
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
// replaceWarnings MUST perform the replaceWarnings operation.
// [🧰semiorepo⌨️server💻main🔖database🛠️replacewarnings](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Database/d/i/replaceWarnings)
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
// [🧰semiorepo⌨️server💻main🔖database🛠️listwarnings](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Database/d/i/listWarnings)
// listWarnings MUST perform the listWarnings operation.
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

// listBreachs retrieves breachs optionally filtered by ticket ID.
// [🧰semiorepo⌨️server💻main🔖database🛠️listbreachs](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Database/d/i/listBreachs)
// listBreachs MUST perform the listBreachs operation.
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

// listConflicts finds scopes claimed by more than one open ticket.
// [🧰semiorepo⌨️server💻main🔖database🛠️listconflicts](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Database/d/i/listConflicts)
// listConflicts MUST perform the listConflicts operation.
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

// [🧰semiorepo⌨️server💻main🔖database🛠️addcontributorwork](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Database/d/i/addContributorWork)
// addContributorWork holds the data fields for a addContributorWork record.
// addContributorWork MUST perform the addContributorWork operation.
func (d *Database) addContributorWork(ctx context.Context, github string, kind string, itemID string) error {
	_, err := d.db.ExecContext(ctx, "INSERT OR REPLACE INTO contributor_work (github, kind, item_id) VALUES (?, ?, ?)", github, kind, itemID)
	return err

// removeContributorWork MUST perform the removeContributorWork operation.
// removeContributorWork holds the data fields for a removeContributorWork record.
// [🧰semiorepo⌨️server💻main🔖database🛠️removecontributorwork](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Database/d/i/removeContributorWork)
func (d *Database) removeContributorWork(ctx context.Context, github string, kindsAndIDs []struct{ Kind, ID string }) error {
	for _, kv := range kindsAndIDs {
		_, _ = d.db.ExecContext(ctx, "DELETE FROM contributor_work WHERE github = ? AND kind = ? AND item_id = ?", github, kv.Kind, kv.ID)
	}
	return nil
}

// [🧰semiorepo⌨️server💻main🔖database🛠️listcontributorsonitem](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Database/d/i/listContributorsOnItem)
// listContributorsOnItem holds the data fields for a listContributorsOnItem record.
// listContributorsOnItem MUST perform the listContributorsOnItem operation.
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

// [🧰semiorepo⌨️server💻main🔖database🛠️removecontributorworkforcheckpoint](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Database/d/i/removeContributorWorkForCheckpoint)
// removeContributorWorkForCheckpoint holds the data fields for a removeContributorWorkForCheckpoint record.
// removeContributorWorkForCheckpoint MUST perform the removeContributorWorkForCheckpoint operation.
func (d *Database) removeContributorWorkForCheckpoint(ctx context.Context, github string, files []string) error {
	for _, f := range files {
		_, _ = d.db.ExecContext(ctx, "DELETE FROM contributor_work WHERE github = ? AND kind = 'file' AND item_id = ?", github, f)
	}
	return nil
}

// #endregion 🔖Database

// #region 🔖EventBus
// [🧰semiorepo⌨️server💻main🔖eventbus](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/EventBus)
// Asynchronous in-process event bus for decoupled event publishing and subscription. MUST persist events to the database before dispatching.

// EventHandler is a callback invoked when an event of a subscribed type is published.
// [🧰semiorepo⌨️server💻main🔖eventbus✂️eventhandler](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/EventBus/d/i/EventHandler)
type EventHandler func(context.Context, Event)

// EventBus is a buffered channel-based event dispatcher with persistent storage.
// [🧰semiorepo⌨️server💻main🔖eventbus✂️eventbus](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/EventBus/d/i/EventBus)
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
// [🧰semiorepo⌨️server💻main🔖eventbus🛠️neweventbus](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/EventBus/d/i/NewEventBus)
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
// [🧰semiorepo⌨️server💻main🔖eventbus🛠️subscribe](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/EventBus/d/i/Subscribe)
func (b *EventBus) Subscribe(eventType string, handler EventHandler) {
	b.handlers[eventType] = append(b.handlers[eventType], handler)
}

// Publish persists an event and dispatches it to subscribers.
// MUST store the event in the database before sending to the channel.
// [🧰semiorepo⌨️server💻main🔖eventbus🛠️publish](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/EventBus/d/i/Publish)
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
// [🧰semiorepo⌨️server💻main🔖eventbus🛠️start](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/EventBus/d/i/Start)
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
// [🧰semiorepo⌨️server💻main🔖eventbus🛠️stop](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/EventBus/d/i/Stop)
func (b *EventBus) Stop() {
	b.cancel()
	b.wg.Wait()
}

// #endregion 🔖EventBus

// #region 🔖DiffParsing
// [🧰semiorepo⌨️server💻main🔖diffparsing](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/DiffParsing)
// Unified diff parser that extracts file paths and hunk line ranges from patch text. MUST handle standard git diff output format.

// hunkHeader is a regex pattern matching unified diff hunk headers.
// [🧰semiorepo⌨️server💻main🔖diffparsing🪨hunkheader](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/DiffParsing/d/i/hunkHeader)
var hunkHeader = regexp.MustCompile(`@@ -([0-9]+)(?:,([0-9]+))? \+([0-9]+)(?:,([0-9]+))? @@`)

// parseUnifiedDiff extracts file paths and hunk ranges from a unified diff patch.
// [🧰semiorepo⌨️server💻main🔖diffparsing🛠️parseunifieddiff](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/DiffParsing/d/i/parseUnifiedDiff)
// parseUnifiedDiff MUST perform the parseUnifiedDiff operation.
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

// parseHunkInt parses a hunk header integer value.
// [🧰semiorepo⌨️server💻main🔖diffparsing🛠️parsehunkint](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/DiffParsing/d/i/parseHunkInt)
// parseHunkInt MUST perform the parseHunkInt operation.
func parseHunkInt(value string) int {
	parsed, _ := strconv.Atoi(value)
	return parsed
}

// parseHunkIntWithDefault parses a hunk header integer or returns the fallback.
// [🧰semiorepo⌨️server💻main🔖diffparsing🛠️parsehunkintwithdefault](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/DiffParsing/d/i/parseHunkIntWithDefault)
// parseHunkIntWithDefault MUST perform the parseHunkIntWithDefault operation.
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
// [🧰semiorepo⌨️server💻main🔖indexing](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Indexing)
// Source code indexer that parses files into scopes covering files, sections, and definitions. MUST support region-marker-based sections and language-specific definition patterns.

// IndexCache holds in-memory caches of indexed scopes partitioned by file path.
// [🧰semiorepo⌨️server💻main🔖indexing✂️indexcache](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Indexing/d/i/IndexCache)
type IndexCache struct {
	Sections    map[string][]Scope
	Definitions map[string][]Scope
	Files       map[string]Scope
}

// newIndexCache creates an empty IndexCache with initialized maps.
// [🧰semiorepo⌨️server💻main🔖indexing🛠️newindexcache](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Indexing/d/i/newIndexCache)
// newIndexCache MUST perform the newIndexCache operation.
func newIndexCache() IndexCache {
	return IndexCache{
		Sections:    map[string][]Scope{},
		Definitions: map[string][]Scope{},
		Files:       map[string]Scope{},
	}
}

// buildScopesForFile parses a file into file, section, and definition scopes.
// [🧰semiorepo⌨️server💻main🔖indexing🛠️buildscopesforfile](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Indexing/d/i/buildScopesForFile)
// buildScopesForFile MUST perform the buildScopesForFile operation.
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
// [🧰semiorepo⌨️server💻main🔖indexing🛠️parsesections](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Indexing/d/i/parseSections)
// parseSections MUST perform the parseSections operation.
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
// parseRegionMarker MUST perform the parseRegionMarker operation.
// [🧰semiorepo⌨️server💻main🔖indexing🛠️parseregionmarker](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Indexing/d/i/parseRegionMarker)
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
// [🧰semiorepo⌨️server💻main🔖indexing🛠️parsemarkdownheading](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Indexing/d/i/parseMarkdownHeading)
// parseMarkdownHeading MUST perform the parseMarkdownHeading operation.
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
// [🧰semiorepo⌨️server💻main🔖indexing🛠️assignsectionpaths](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Indexing/d/i/assignSectionPaths)
// assignSectionPaths MUST perform the assignSectionPaths operation.
func assignSectionPaths(sections []Scope) []Scope {
	for i := range sections {
		sections[i].ID = fmt.Sprintf("section:%s#%s", sections[i].FilePath, sections[i].SectionPath)
	return sections
}

// parseDefinitions extracts definition scopes using language-specific patterns.
// [🧰semiorepo⌨️server💻main🔖indexing🛠️parsedefinitions](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Indexing/d/i/parseDefinitions)
// parseDefinitions MUST perform the parseDefinitions operation.
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
// [🧰semiorepo⌨️server💻main🔖indexing🛠️definitionpatterns](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Indexing/d/i/definitionPatterns)
// definitionPatterns MUST perform the definitionPatterns operation.
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
// [🧰semiorepo⌨️server💻main🔖claims](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Claims)
// Scope claim mapping logic that associates diff hunks with overlapping scopes. MUST detect multi-ticket conflicts.

// mapClaims maps diff hunks to overlapping scopes and returns claimed IDs.
// [🧰semiorepo⌨️server💻main🔖claims🛠️mapclaims](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Claims/d/i/mapClaims)
// mapClaims MUST perform the mapClaims operation.
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

// filterScopesByFile returns scopes matching the given file path.
// [🧰semiorepo⌨️server💻main🔖claims🛠️filterscopesbyfile](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Claims/d/i/filterScopesByFile)
// filterScopesByFile MUST perform the filterScopesByFile operation.
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
// [🧰semiorepo⌨️server💻main🔖claims🛠️rangesoverlap](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Claims/d/i/rangesOverlap)
// rangesOverlap MUST perform the rangesOverlap operation.
func rangesOverlap(a LineRange, b LineRange) bool {
	if a.Start == 0 || b.Start == 0 {
		return false
	}
	return a.Start <= b.End && b.Start <= a.End
}

// appendIfMissing appends a string to a slice only if it is not already present.
// [🧰semiorepo⌨️server💻main🔖claims🛠️appendifmissing](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Claims/d/i/appendIfMissing)
// appendIfMissing MUST perform the appendIfMissing operation.
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
// [🧰semiorepo⌨️server💻main🔖warnings](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Warnings)
// Conflict warning generation from multi-ticket scope overlaps. MUST produce error-severity warnings for blocking conflicts.

// buildConflictWarnings creates warning records from detected scope conflicts.
// buildConflictWarnings MUST perform the buildConflictWarnings operation.
// [🧰semiorepo⌨️server💻main🔖warnings🛠️buildconflictwarnings](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Warnings/d/i/buildConflictWarnings)
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
// [🧰semiorepo⌨️server💻main🔖server](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Server)
// HTTP server with ticket lifecycle, diff ingestion, pre-checkpoint checks, indexing, and webhook endpoints. MUST enforce authentication on mutating routes.

// Server is the main HTTP server holding configuration, database, event bus, and caches.
// [🧰semiorepo⌨️server💻main🔖server✂️server](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Server/d/i/Server)
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
// [🧰semiorepo⌨️server💻main🔖server🛠️newserver](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Server/d/i/NewServer)
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
// newRequestContext MUST perform the newRequestContext operation.
// [🧰semiorepo⌨️server💻main🔖server🛠️newrequestcontext](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Server/d/i/newRequestContext)
func (s *Server) newRequestContext(r *http.Request) (context.Context, context.CancelFunc) {
	return context.WithTimeout(r.Context(), 15*time.Second)
}

// requireAuth checks the bearer token against the configured server token.
// requireAuth MUST perform the requireAuth operation.
// [🧰semiorepo⌨️server💻main🔖server🛠️requireauth](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Server/d/i/requireAuth)
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
// [🧰semiorepo⌨️server💻main🔖server🛠️decodejson](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Server/d/i/decodeJSON)
// decodeJSON MUST perform the decodeJSON operation.
func (s *Server) decodeJSON(r *http.Request, payload interface{}) error {
	decoder := json.NewDecoder(io.LimitReader(r.Body, s.config.RequestBodyLimit))
	decoder.DisallowUnknownFields()
	return decoder.Decode(payload)
}

// writeJSON writes a JSON response with the given status code.
// [🧰semiorepo⌨️server💻main🔖server🛠️writejson](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Server/d/i/writeJSON)
// writeJSON MUST perform the writeJSON operation.
func (s *Server) writeJSON(w http.ResponseWriter, status int, payload interface{}) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(status)
	_ = json.NewEncoder(w).Encode(payload)
}

// respondError writes a JSON error response.
// respondError MUST perform the respondError operation.
// [🧰semiorepo⌨️server💻main🔖server🛠️responderror](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Server/d/i/respondError)
func (s *Server) respondError(w http.ResponseWriter, status int, message string) {
	s.writeJSON(w, status, map[string]string{"error": message})
}

// handleEvents accepts CLI event payloads and persists/publishes them.
// [🧰semiorepo⌨️server💻main🔖server🛠️handleevents](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Server/d/i/handleEvents)
// handleEvents MUST perform the handleEvents operation.
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

// handleHealth responds with 200 OK for liveness checks.
// [🧰semiorepo⌨️server💻main🔖server🛠️handlehealth](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Server/d/i/handleHealth)
// handleHealth MUST perform the handleHealth operation.
func (s *Server) handleHealth(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	w.WriteHeader(http.StatusOK)
	_, _ = w.Write([]byte("ok"))
}

// handleTicketOpen creates a new ticket from the request payload.
// handleTicketOpen MUST perform the handleTicketOpen operation.
// [🧰semiorepo⌨️server💻main🔖server🛠️handleticketopen](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Server/d/i/handleTicketOpen)
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

// handleTicketClose closes an existing ticket with a summary.
// [🧰semiorepo⌨️server💻main🔖server🛠️handleticketclose](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Server/d/i/handleTicketClose)
// handleTicketClose MUST perform the handleTicketClose operation.
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
// [🧰semiorepo⌨️server💻main🔖server🛠️handleticketreopen](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Server/d/i/handleTicketReopen)
// handleTicketReopen MUST perform the handleTicketReopen operation.
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
// [🧰semiorepo⌨️server💻main🔖server🛠️handleticketsquery](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Server/d/i/handleTicketsQuery)
// handleTicketsQuery MUST perform the handleTicketsQuery operation.
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
// [🧰semiorepo⌨️server💻main🔖server🛠️handleticketdetail](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Server/d/i/handleTicketDetail)
// handleTicketDetail MUST perform the handleTicketDetail operation.
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
// [🧰semiorepo⌨️server💻main🔖server🛠️handleticketclaims](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Server/d/i/handleTicketClaims)
// handleTicketClaims MUST perform the handleTicketClaims operation.
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
// handleDiffIngest MUST perform the handleDiffIngest operation.
// [🧰semiorepo⌨️server💻main🔖server🛠️handlediffingest](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Server/d/i/handleDiffIngest)
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

// handlePrecheckpoint runs a pre-checkpoint check against a diff patch.
// [🧰semiorepo⌨️server💻main🔖server🛠️handleprecheckpoint](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Server/d/i/handlePrecheckpoint)
// handlePrecheckpoint MUST perform the handlePrecheckpoint operation.
func (s *Server) handlePrecheckpoint(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	if !s.requireAuth(r) {
		s.respondError(w, http.StatusUnauthorized, "unauthorized")
		return
	}
	var payload PrecheckpointRequest
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
	result, warnings, breachs, err := s.processDiff(ctx, payload.TicketID, payload.Patch, nil)
	if err != nil {
		s.respondError(w, http.StatusInternalServerError, err.Error())
		return
	}
	blockers := result.Blockers
	ok := len(blockers) == 0
	response := PrecheckpointResponse{
		OK:           ok,
		Blockers:     blockers,
		Warnings:     warnings,
		Breachs:      breachs,
		AutofixPatch: "",
	}
	s.writeJSON(w, http.StatusOK, response)
}

// handleReindex walks the repo and re-indexes all files.
// [🧰semiorepo⌨️server💻main🔖server🛠️handlereindex](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Server/d/i/handleReindex)
// handleReindex MUST perform the handleReindex operation.
func (s *Server) handleReindex(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}
	if !s.requireAuth(r) {
		s.respondError(w, http.StatusUnauthorized, "unauthorized")
		return
	}
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
// [🧰semiorepo⌨️server💻main🔖server🛠️handleindexfile](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Server/d/i/handleIndexFile)
// handleIndexFile MUST perform the handleIndexFile operation.
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
// [🧰semiorepo⌨️server💻main🔖server🛠️handlewarnings](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Server/d/i/handleWarnings)
// handleWarnings MUST perform the handleWarnings operation.
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

// handleBreachs returns breachs optionally filtered by ticket ID.
// [🧰semiorepo⌨️server💻main🔖server🛠️handlebreachs](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Server/d/i/handleBreachs)
// handleBreachs MUST perform the handleBreachs operation.
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

// handleScopes returns scopes for a given file query parameter.
// handleScopes MUST perform the handleScopes operation.
// [🧰semiorepo⌨️server💻main🔖server🛠️handlescopes](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Server/d/i/handleScopes)
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
// [🧰semiorepo⌨️server💻main🔖processing](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Processing)
// Diff processing pipeline that indexes changed files, maps claims, detects conflicts, and produces warnings. MUST be transactional per request.

// ProcessResult holds the outcome of a diff processing operation.
// [🧰semiorepo⌨️server💻main🔖processing✂️processresult](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Processing/d/i/ProcessResult)
type ProcessResult struct {
	ChangedFiles  []string
	ClaimedScopes []string
	Blockers      []string
}

// processDiff parses the patch, indexes changed files, maps claims, and detects conflicts.
// MUST return warnings and breachs alongside the processing result.
// [🧰semiorepo⌨️server💻main🔖processing🛠️processdiff](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Processing/d/i/processDiff)
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

// uniqueFiles extracts deduplicated file paths from a diff result.
// [🧰semiorepo⌨️server💻main🔖processing🛠️uniquefiles](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Processing/d/i/uniqueFiles)
// uniqueFiles MUST perform the uniqueFiles operation.
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
// snapshotMap MUST perform the snapshotMap operation.
// [🧰semiorepo⌨️server💻main🔖processing🛠️snapshotmap](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Processing/d/i/snapshotMap)
func snapshotMap(snapshots []FileSnapshot) map[string]string {
	mapping := map[string]string{}
	for _, snapshot := range snapshots {
		mapping[snapshot.Path] = snapshot.Content
	}
	return mapping
}

// updateIndexForFile builds scopes from file content and updates both the database and cache.
// updateIndexForFile MUST perform the updateIndexForFile operation.
// [🧰semiorepo⌨️server💻main🔖processing🛠️updateindexforfile](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Processing/d/i/updateIndexForFile)
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
// [🧰semiorepo⌨️server💻main🔖processing🛠️buildscopeid](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Processing/d/i/buildScopeID)
// buildScopeID MUST perform the buildScopeID operation.
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
// [🧰semiorepo⌨️server💻main🔖processing🛠️walkrepofiles](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Processing/d/i/walkRepoFiles)
// walkRepoFiles MUST perform the walkRepoFiles operation.
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
// [🧰semiorepo⌨️server💻main🔖webhooks](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Webhooks)
// GitHub webhook handlers for issue comment caching and issue event processing. MUST verify HMAC signatures when a secret is configured.

// GitHubComment stores a cached GitHub issue comment for correlating close/reopen events.
// [🧰semiorepo⌨️server💻main🔖webhooks✂️githubcomment](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Webhooks/d/i/GitHubComment)
type GitHubComment struct {
	Body      string
	Actor     string
	Repo      string
	Issue     int
	Second time.Time
}

// handleGitHubWebhook processes incoming GitHub webhook events.
// [🧰semiorepo⌨️server💻main🔖webhooks🛠️handlegithubwebhook](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Webhooks/d/i/handleGitHubWebhook)
// handleGitHubWebhook MUST perform the handleGitHubWebhook operation.
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

// verifyGitHubSignature validates the HMAC-SHA256 signature of a webhook payload.
// verifyGitHubSignature MUST perform the verifyGitHubSignature operation.
// [🧰semiorepo⌨️server💻main🔖webhooks🛠️verifygithubsignature](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Webhooks/d/i/verifyGitHubSignature)
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
// [🧰semiorepo⌨️server💻main🔖webhooks🛠️cachegithubcomment](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Webhooks/d/i/cacheGitHubComment)
// cacheGitHubComment MUST perform the cacheGitHubComment operation.
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
		Second: time.Now().UTC(),
	}
	s.ghLock.Unlock()
}

// handleGitHubIssueEvent processes GitHub issue close/reopen events.
// [🧰semiorepo⌨️server💻main🔖webhooks🛠️handlegithubissueevent](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Webhooks/d/i/handleGitHubIssueEvent)
// handleGitHubIssueEvent MUST perform the handleGitHubIssueEvent operation.
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
// findCachedComment MUST perform the findCachedComment operation.
// [🧰semiorepo⌨️server💻main🔖webhooks🛠️findcachedcomment](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Webhooks/d/i/findCachedComment)
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

// extractIssueComment extracts issue number, repo, actor, and body from a webhook payload.
// [🧰semiorepo⌨️server💻main🔖webhooks🛠️extractissuecomment](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Webhooks/d/i/extractIssueComment)
// extractIssueComment MUST perform the extractIssueComment operation.
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
// [🧰semiorepo⌨️server💻main🔖webhooks🛠️extractissuenumber](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Webhooks/d/i/extractIssueNumber)
// extractIssueNumber MUST perform the extractIssueNumber operation.
func extractIssueNumber(payload map[string]interface{}) int {
	if issue, ok := payload["issue"].(map[string]interface{}); ok {
		if number, ok := issue["number"].(float64); ok {
			return int(number)
		}
	}
	return 0
}

// extractRepoFullName extracts the repository full name from a GitHub webhook payload.
// extractRepoFullName MUST perform the extractRepoFullName operation.
// [🧰semiorepo⌨️server💻main🔖webhooks🛠️extractrepofullname](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Webhooks/d/i/extractRepoFullName)
func extractRepoFullName(payload map[string]interface{}) string {
	if repo, ok := payload["repository"].(map[string]interface{}); ok {
		if name, ok := repo["full_name"].(string); ok {
			return name
		}
	}
	return ""
}

// handleGitHubPushEvent MUST perform the handleGitHubPushEvent operation.
// handleGitHubPushEvent holds the data fields for a handleGitHubPushEvent record.
// [🧰semiorepo⌨️server💻main🔖webhooks🛠️handlegithubpushevent](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Webhooks/d/i/handleGitHubPushEvent)
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

// extractActorLogin extracts the sender login from a GitHub webhook payload.
// [🧰semiorepo⌨️server💻main🔖webhooks🛠️extractactorlogin](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Webhooks/d/i/extractActorLogin)
// extractActorLogin MUST perform the extractActorLogin operation.
func extractActorLogin(payload map[string]interface{}) string {
	if sender, ok := payload["sender"].(map[string]interface{}); ok {
		if login, ok := sender["login"].(string); ok {
			return login
	}
	return ""
}

// #endregion 🔖Webhooks

// #region 🔖Discord
// [🧰semiorepo⌨️server💻main🔖discord](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Discord)
// Discord notification integration for ticket lifecycle events. MUST silently skip when no webhook URL is configured.

// notifyDiscord performs the notifyDiscord operation.
// [🧰semiorepo⌨️server💻main🔖discord🛠️notifydiscord](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Discord/d/i/notifyDiscord)
// notifyDiscord MUST perform the notifyDiscord operation.
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
// registerNotifications MUST perform the registerNotifications operation.
// [🧰semiorepo⌨️server💻main🔖discord🛠️registernotifications](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Discord/d/i/registerNotifications)
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
		repopkg.EventTicketOpen, repopkg.EventTicketClose, repopkg.EventTicketReopen, repopkg.EventTicketChange,
		repopkg.EventGoalOpen, repopkg.EventGoalClose, repopkg.EventGoalReopen, repopkg.EventGoalChange,
		repopkg.EventContributorAdd, repopkg.EventContributorRemove,
		repopkg.EventTodoCreate, repopkg.EventTodoChange, repopkg.EventTodoDelete,
	} {
		k := kind
		s.bus.Subscribe(string(k), func(ctx context.Context, event Event) {
			s.onCLIEvent(ctx, k, event)
		})
	}
	s.bus.Subscribe(string(repopkg.EventCheckpoint), func(ctx context.Context, event Event) {
		s.onCheckpointEvent(ctx, event)
	})
}

// onCLIEvent holds the data fields for a onCLIEvent record.
// [🧰semiorepo⌨️server💻main🔖discord🛠️onclievent](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Discord/d/i/onCLIEvent)
// onCLIEvent MUST perform the onCLIEvent operation.
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

// [🧰semiorepo⌨️server💻main🔖discord🛠️oncheckpointevent](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Discord/d/i/onCheckpointEvent)
// onCheckpointEvent holds the data fields for a onCheckpointEvent record.
// onCheckpointEvent MUST perform the onCheckpointEvent operation.
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

// extractAuthorAndItems holds the data fields for a extractAuthorAndItems record.
// extractAuthorAndItems MUST perform the extractAuthorAndItems operation.
// [🧰semiorepo⌨️server💻main🔖discord🛠️extractauthoranditems](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Discord/d/i/extractAuthorAndItems)
func (s *Server) extractAuthorAndItems(kind repopkg.EventKind, payloadJSON string) (author string, items []repopkg.WorkItem) {
	switch kind {
	case repopkg.EventTicketOpen, repopkg.EventTicketClose, repopkg.EventTicketReopen, repopkg.EventTicketChange:
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
	case repopkg.EventGoalOpen, repopkg.EventGoalClose, repopkg.EventGoalReopen, repopkg.EventGoalChange:
		var p repopkg.GoalPayload
		if json.Unmarshal([]byte(payloadJSON), &p) != nil {
			return "", nil
		}
		author = getAuthorFromPayload(payloadJSON)
		return author, []repopkg.WorkItem{{Kind: "goal", ID: p.ID}}
	case repopkg.EventContributorAdd, repopkg.EventContributorRemove:
		var p repopkg.ContributorPayload
		if json.Unmarshal([]byte(payloadJSON), &p) != nil {
			return "", nil
		}
		return p.Author, []repopkg.WorkItem{{Kind: "contributor", ID: p.Github}}
	case repopkg.EventTodoCreate, repopkg.EventTodoChange, repopkg.EventTodoDelete:
		var p repopkg.TodoPayload
		if json.Unmarshal([]byte(payloadJSON), &p) != nil {
			return "", nil
		}
		return p.Author, []repopkg.WorkItem{{Kind: "todo", ID: p.ID}}
	default:
		return "", nil
	}
}

// [🧰semiorepo⌨️server💻main🔖discord🛠️getauthorfrompayload](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Discord/d/i/getAuthorFromPayload)
// getAuthorFromPayload holds the data fields for a getAuthorFromPayload record.
// getAuthorFromPayload MUST perform the getAuthorFromPayload operation.
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

// [🧰semiorepo⌨️server💻main🔖discord🛠️filterout](semiorepo://p/i/semio-repo/b/b/server/f/main.go/s/Discord/d/i/filterOut)
// filterOut holds the data fields for a filterOut record.
// filterOut MUST perform the filterOut operation.
func filterOut(list []string, exclude string) []string {
	var out []string
	for _, x := range list {
		if x != exclude {
			out = append(out, x)
		}
	}
	return out
}

// #endregion 🔖Discord

func newID() string {
	return fmt.Sprintf("%d-%d", time.Now().UTC().UnixNano(), rand.Int63())
}

// #endregion 🔖Utilities

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
	mux.HandleFunc("/checks/precheckpoint", server.handlePrecheckpoint)
	mux.HandleFunc("/repo/reindex", server.handleReindex)
	mux.HandleFunc("/repo/index-file", server.handleIndexFile)
	mux.HandleFunc("/warnings", server.handleWarnings)
	mux.HandleFunc("/breachs", server.handleBreachs)
	mux.HandleFunc("/scopes", server.handleScopes)
	mux.HandleFunc("/events", server.handleEvents)
	mux.HandleFunc("/webhooks/github", server.handleGitHubWebhook)
	log.Printf("semio repo server listening on %s", config.Address)
	if err := http.ListenAndServe(config.Address, mux); err != nil {
		log.Fatal(err)
	}
}

// #endregion 🔖Main
