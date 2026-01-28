package main

import (
"os"
"os/exec"
"path/filepath"
"testing"
)

func TestTicketLifecycle_NoGithub(t *testing.T) {
tmpDir := t.TempDir()

// Setup generic git repo
run := func(name string, field ...string) {
cmd := exec.Command(name, field...)
cmd.Dir = tmpDir
out, err := cmd.CombinedOutput()
if err != nil {
t.Fatalf("run %s %v failed: %v\nOutput: %s", name, field, err, out)
}
}
run("git", "init")
run("git", "config", "user.email", "test@test.com")
run("git", "config", "user.name", "Test")
run("git", "config", "commit.gpgsign", "false")
run("git", "commit", "--allow-empty", "-m", "initial")

// Mock global rootDir
oldRoot := rootDir
rootDir = tmpDir
defer func() { rootDir = oldRoot }()

// Create tickets dir
if err := os.MkdirAll(filepath.Join(tmpDir, ".semio-repo", "tickets"), 0755); err != nil {
t.Fatal(err)
}

// 1. Open Ticket
ticket, err := OpenTicket("Test Title NoGH", "Test Prompt", "gemini-3-pro", "copilot-chat", "", false, "", "", true)
if err != nil {
t.Fatalf("OpenTicket failed: %v", err)
}
if ticket.Data.GitHub != nil {
t.Error("OpenTicket: GitHub data should be nil")
}

// Create a file to finish
testFile := "test.txt"
if err := os.WriteFile(filepath.Join(tmpDir, testFile), []byte("content"), 0644); err != nil {
t.Fatal(err)
}

// 2. Open Goal
// Note: OpenGoal creates .semio-repo/goals/SLUG/goal.json using Slugify which uppercases the title.
goal, err := OpenGoal("Goal Title", "Goal Description", "Goal Prompt", "2026-02-15", "copilot-chat", "gemini-3-pro", true)
if err != nil {
t.Fatalf("OpenGoal failed: %v", err)
}
if goal.Title != "Goal Title" {
t.Errorf("expected title 'Goal Title', got '%s'", goal.Title)
}
if goal.Prompt != "Goal Prompt" {
t.Errorf("expected prompt 'Goal Prompt', got '%s'", goal.Prompt)
}
if goal.UI != "copilot-chat" {
t.Errorf("expected ui 'copilot-chat', got '%s'", goal.UI)
}
if goal.LLM != "gemini-3-pro" {
t.Errorf("expected llm 'gemini-3-pro', got '%s'", goal.LLM)
}
if goal.GitHub != nil {
t.Error("OpenGoal: GitHub data should be nil")
}

// Verify goal file exists
// Slugify("Goal Title") -> "GOAL-TITLE"
goalPath := filepath.Join(tmpDir, ".semio-repo", "goals", "GOAL-TITLE", "goal.json")
if _, err := os.Stat(goalPath); os.IsNotExist(err) {
t.Errorf("goal file not created at %s", goalPath)
}

run("git", "add", testFile)
run("git", "commit", "-m", "add test file")

// 2. Finish Ticket
// ticket.Data.GitHub is nil, so FinishTicket logic for labels should skip safely.
err = FinishTicket(ticket, "Summary", []string{testFile}, true)
if err != nil {
t.Fatalf("FinishTicket failed: %v", err)
}
if ticket.GetStatus() != TicketStatusClosed {
t.Errorf("Ticket status mismatch: got %v, want closed", ticket.GetStatus())
}

// 3. Reopen Ticket
// Provide valid UI (e.g. "copilot-chat")
err = ReopenTicket(ticket, "Reopen Prompt", "gemini-3-pro", "copilot-chat", "", true)
if err != nil {
t.Fatalf("ReopenTicket failed: %v", err)
}
if ticket.GetStatus() != TicketStatusOpen {
t.Errorf("Ticket status mismatch: got %v, want open", ticket.GetStatus())
}

// 4. Goal Lifecycle
ctx := NewRepoContext(tmpDir)

goalInput := GoalCreateInput{
Title:       "Test Goal NoGH 2",
Description: "Desc",
Prompt:      "Prompt",
DueDate:     "2026-02-15",
UI:          "cursor",
LLM:         "gpt-5-2-codex",
NoGithub:    true,
}

goal2, err := ctx.GoalCreate(goalInput)
if err != nil {
t.Fatalf("GoalCreate failed: %v", err)
}
if goal2.Title != "Test Goal NoGH 2" {
t.Errorf("expected title 'Test Goal NoGH 2', got '%s'", goal2.Title)
}
}
