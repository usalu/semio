#!/usr/bin/env python3
"""Script to add starting Emit calls in main.go."""

import re

file_path = "/workspaces/semio/repo/cli/main.go"
with open(file_path, "r") as f:
    content = f.read()

# ToolTicketOpen: add starting before OpenTicket call
old = "func ToolTicketOpen(title, prompt, llm, client, draft string, noIssue bool, goal string, parent string, noManagement bool, issue string) ToolResult {\n\tticket, err := OpenTicket("
new = """func ToolTicketOpen(title, prompt, llm, client, draft string, noIssue bool, goal string, parent string, noManagement bool, issue string) ToolResult {
\trepopkg.Emit(repopkg.EventTicketOpenStarting, "repo-cli", repopkg.TicketOpenPayload{
\t\tTitle: title, Prompt: prompt, LLM: llm, Client: client, Goal: goal, Parent: parent,
\t})
\tticket, err := OpenTicket("""
content = content.replace(old, new, 1)

# ToolTicketClose: add starting before FinishTicket call
old = "func ToolTicketClose(year, month, day int, slug, summary string, files []string, title string, noManagement bool) ToolResult {\n\tticket, err := ReadTicket("
new = """func ToolTicketClose(year, month, day int, slug, summary string, files []string, title string, noManagement bool) ToolResult {
\trepopkg.Emit(repopkg.EventTicketCloseStarting, "repo-cli", repopkg.TicketClosePayload{
\t\tTicketPayload: repopkg.TicketPayload{ID: fmt.Sprintf("%d/%02d/%02d/%s", year, month, day, slug), Year: year, Month: month, Day: day, Slug: slug},
\t\tSummary: summary, Files: files,
\t})
\tticket, err := ReadTicket("""
content = content.replace(old, new, 1)

# ToolTicketReopen: add starting before ReadTicket call
old = "func ToolTicketReopen(year, month, day int, slug, prompt, llm, client, draft string, title string, goal string, parent string, noManagement bool) ToolResult {\n\toutput := NewOutput()\n\tticket, err := ReadTicket("
new = """func ToolTicketReopen(year, month, day int, slug, prompt, llm, client, draft string, title string, goal string, parent string, noManagement bool) ToolResult {
\trepopkg.Emit(repopkg.EventTicketReopenStarting, "repo-cli", repopkg.TicketReopenPayload{
\t\tTicketPayload: repopkg.TicketPayload{ID: fmt.Sprintf("%d/%02d/%02d/%s", year, month, day, slug), Year: year, Month: month, Day: day, Slug: slug},
\t\tPrompt: prompt, LLM: llm, Client: client,
\t})
\toutput := NewOutput()
\tticket, err := ReadTicket("""
content = content.replace(old, new, 1)

# ToolGoalCreate: add starting at the beginning
old = "func ToolGoalCreate(title, description, prompt, dueDate, llm, client string, noManagement bool, parent, milestone string) ToolResult {\n\tctx, err := GetRepoContext()"
new = """func ToolGoalCreate(title, description, prompt, dueDate, llm, client string, noManagement bool, parent, milestone string) ToolResult {
\trepopkg.Emit(repopkg.EventGoalOpenStarting, "repo-cli", repopkg.GoalOpenPayload{
\t\tTitle: title, Description: description, Parent: parent,
\t})
\tctx, err := GetRepoContext()"""
content = content.replace(old, new, 1)

# ToolGoalClose: add starting at the beginning
old = "func ToolGoalClose(id, summary string, noManagement bool) ToolResult {\n\tctx, err := GetRepoContext()"
new = """func ToolGoalClose(id, summary string, noManagement bool) ToolResult {
\trepopkg.Emit(repopkg.EventGoalCloseStarting, "repo-cli", repopkg.GoalClosePayload{
\t\tGoalPayload: repopkg.GoalPayload{ID: id},
\t\tSummary: summary,
\t})
\tctx, err := GetRepoContext()"""
content = content.replace(old, new, 1)

# ToolGoalReopen: add starting at the beginning
old = "func ToolGoalReopen(id, prompt, llm, client, title, description, dueDate string, noManagement bool) ToolResult {\n\tctx, err := GetRepoContext()"
new = """func ToolGoalReopen(id, prompt, llm, client, title, description, dueDate string, noManagement bool) ToolResult {
\trepopkg.Emit(repopkg.EventGoalReopenStarting, "repo-cli", repopkg.GoalReopenPayload{
\t\tGoalPayload: repopkg.GoalPayload{ID: id},
\t\tPrompt: prompt, LLM: llm, Client: client,
\t})
\tctx, err := GetRepoContext()"""
content = content.replace(old, new, 1)

# ToolContributorAdd: add starting at the beginning
old = "func ToolContributorAdd(github string) ToolResult {\n\toutput := NewOutput()"
new = """func ToolContributorAdd(github string) ToolResult {
\trepopkg.Emit(repopkg.EventContributorAddStarting, "repo-cli", repopkg.ContributorPayload{
\t\tGithub: github,
\t})
\toutput := NewOutput()"""
content = content.replace(old, new, 1)

# ToolContributorRemove: add starting at the beginning
old = "func ToolContributorRemove(github string) ToolResult {\n\toutput := NewOutput()"
new = """func ToolContributorRemove(github string) ToolResult {
\trepopkg.Emit(repopkg.EventContributorRemoveStarting, "repo-cli", repopkg.ContributorPayload{
\t\tGithub: github,
\t})
\toutput := NewOutput()"""
content = content.replace(old, new, 1)

# ToolDraftCreate: add starting at the beginning
old = "func ToolDraftCreate(title string, files []string) ToolResult {\n\toutput := NewOutput()"
new = """func ToolDraftCreate(title string, files []string) ToolResult {
\trepopkg.Emit(repopkg.EventDraftCreateStarting, "repo-cli", repopkg.DraftPayload{
\t\tTitle: title,
\t})
\toutput := NewOutput()"""
content = content.replace(old, new, 1)

# ToolDraftDelete: add starting at the beginning
old = "func ToolDraftDelete(slug string) ToolResult {\n\toutput := NewOutput()"
new = """func ToolDraftDelete(slug string) ToolResult {
\trepopkg.Emit(repopkg.EventDraftDeleteStarting, "repo-cli", repopkg.DraftPayload{
\t\tSlug: slug,
\t})
\toutput := NewOutput()"""
content = content.replace(old, new, 1)

# ToolFolderCreate: add starting + ended
old = 'func ToolFolderCreate(path string) ToolResult {\n\toutput := NewOutput()\n\tabsPath := filepath.Join(rootDir, path)\n\tif FileExists(absPath) {\n\t\treturn toolErrorMsg(fmt.Sprintf("Folder already exists: %s", path))\n\t}\n\tif err := EnsureDir(absPath); err != nil {\n\t\treturn toolErrorResult(err)\n\t}\n\toutput.Success(fmt.Sprintf("\\n📁Created folder: %s", path))\n\treturn ToolResult{Output: *output}\n}'
new = """func ToolFolderCreate(path string) ToolResult {
\trepopkg.Emit(repopkg.EventFolderCreateStarting, "repo-cli", repopkg.FolderPayload{Path: path})
\toutput := NewOutput()
\tabsPath := filepath.Join(rootDir, path)
\tif FileExists(absPath) {
\t\treturn toolErrorMsg(fmt.Sprintf("Folder already exists: %s", path))
\t}
\tif err := EnsureDir(absPath); err != nil {
\t\treturn toolErrorResult(err)
\t}
\trepopkg.Emit(repopkg.EventFolderCreateEnded, "repo-cli", repopkg.FolderPayload{Path: path})
\toutput.Success(fmt.Sprintf("\\n📁Created folder: %s", path))
\treturn ToolResult{Output: *output}
}"""
content = content.replace(old, new, 1)

# ToolFolderMove: add starting + ended
old = """func ToolFolderMove(source, target string) ToolResult {
\toutput := NewOutput()
\tabsSource := filepath.Join(rootDir, source)
\tabsTarget := filepath.Join(rootDir, target)
\tif !FileExists(absSource) {
\t\treturn toolErrorMsg(fmt.Sprintf("Source folder not found: %s", source))
\t}
\tif FileExists(absTarget) {
\t\treturn toolErrorMsg(fmt.Sprintf("Target folder already exists: %s", target))
\t}
\tif err := EnsureDir(filepath.Dir(absTarget)); err != nil {
\t\treturn toolErrorResult(err)
\t}
\tif err := os.Rename(absSource, absTarget); err != nil {
\t\treturn toolErrorResult(err)
\t}
\tUpdateAgentsDocsPath(source, target)
\toutput.Success(fmt.Sprintf("\\n📁Moved folder: %s → %s", source, target))
\treturn ToolResult{Output: *output}
}"""
new = """func ToolFolderMove(source, target string) ToolResult {
\trepopkg.Emit(repopkg.EventFolderMoveStarting, "repo-cli", repopkg.FolderPayload{Path: target, From: source})
\toutput := NewOutput()
\tabsSource := filepath.Join(rootDir, source)
\tabsTarget := filepath.Join(rootDir, target)
\tif !FileExists(absSource) {
\t\treturn toolErrorMsg(fmt.Sprintf("Source folder not found: %s", source))
\t}
\tif FileExists(absTarget) {
\t\treturn toolErrorMsg(fmt.Sprintf("Target folder already exists: %s", target))
\t}
\tif err := EnsureDir(filepath.Dir(absTarget)); err != nil {
\t\treturn toolErrorResult(err)
\t}
\tif err := os.Rename(absSource, absTarget); err != nil {
\t\treturn toolErrorResult(err)
\t}
\tUpdateAgentsDocsPath(source, target)
\trepopkg.Emit(repopkg.EventFolderMoveEnded, "repo-cli", repopkg.FolderPayload{Path: target, From: source})
\toutput.Success(fmt.Sprintf("\\n📁Moved folder: %s → %s", source, target))
\treturn ToolResult{Output: *output}
}"""
content = content.replace(old, new, 1)

# ToolFolderDelete: add starting + ended
old = """func ToolFolderDelete(path string) ToolResult {
\toutput := NewOutput()
\tabsPath := filepath.Join(rootDir, path)
\tif !FileExists(absPath) {
\t\treturn toolErrorMsg(fmt.Sprintf("Folder not found: %s", path))
\t}
\tif err := os.RemoveAll(absPath); err != nil {
\t\treturn toolErrorResult(err)
\t}
\toutput.Success(fmt.Sprintf("\\n🗑 Deleted folder: %s", path))
\treturn ToolResult{Output: *output}
}"""
new = """func ToolFolderDelete(path string) ToolResult {
\trepopkg.Emit(repopkg.EventFolderDeleteStarting, "repo-cli", repopkg.FolderPayload{Path: path})
\toutput := NewOutput()
\tabsPath := filepath.Join(rootDir, path)
\tif !FileExists(absPath) {
\t\treturn toolErrorMsg(fmt.Sprintf("Folder not found: %s", path))
\t}
\tif err := os.RemoveAll(absPath); err != nil {
\t\treturn toolErrorResult(err)
\t}
\trepopkg.Emit(repopkg.EventFolderDeleteEnded, "repo-cli", repopkg.FolderPayload{Path: path})
\toutput.Success(fmt.Sprintf("\\n🗑 Deleted folder: %s", path))
\treturn ToolResult{Output: *output}
}"""
content = content.replace(old, new, 1)

with open(file_path, "w") as f:
    f.write(content)

print(
    "Done - added starting emits for ticket, goal, contributor, draft, and folder operations"
)
