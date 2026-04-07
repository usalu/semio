#!/usr/bin/env python3
"""Script to add starting Emit calls for file, section, integrate, extract, export, analyze, fix operations."""

import re

file_path = "/workspaces/semio/repo/cli/main.go"
with open(file_path, "r") as f:
    content = f.read()

# ToolFileCreate: add starting + ended
old = """func ToolFileCreate(path string) ToolResult {
\toutput := NewOutput()
\tabsPath := filepath.Join(rootDir, path)
\tif FileExists(absPath) {
\t\treturn toolErrorMsg(fmt.Sprintf("File already exists: %s", path))
\t}
\tlanguage := GetLanguage(path)
\tcontent := generateFileHeader(path, language)
\tif err := WriteTextFile(absPath, content); err != nil {
\t\treturn toolErrorResult(err)
\t}
\toutput.Success(fmt.Sprintf("\\n📄Created file: %s", path))
\treturn ToolResult{Output: *output}
}"""
new = """func ToolFileCreate(path string) ToolResult {
\trepopkg.Emit(repopkg.EventFileCreateStarting, "repo-cli", repopkg.FilePayload{Path: path})
\toutput := NewOutput()
\tabsPath := filepath.Join(rootDir, path)
\tif FileExists(absPath) {
\t\treturn toolErrorMsg(fmt.Sprintf("File already exists: %s", path))
\t}
\tlanguage := GetLanguage(path)
\tcontent := generateFileHeader(path, language)
\tif err := WriteTextFile(absPath, content); err != nil {
\t\treturn toolErrorResult(err)
\t}
\trepopkg.Emit(repopkg.EventFileCreateEnded, "repo-cli", repopkg.FilePayload{Path: path})
\toutput.Success(fmt.Sprintf("\\n📄Created file: %s", path))
\treturn ToolResult{Output: *output}
}"""
content = content.replace(old, new, 1)
if old not in open(file_path).read():
    print("ToolFileCreate: replaced")
else:
    print("ToolFileCreate: NOT replaced - pattern not found")

# ToolFileMove: add starting + ended
old_move = (
    "func ToolFileMove(source, target string) ToolResult {\n\toutput := NewOutput()"
)
new_move = """func ToolFileMove(source, target string) ToolResult {
\trepopkg.Emit(repopkg.EventFileMoveStarting, "repo-cli", repopkg.FilePayload{Path: target, From: source})
\toutput := NewOutput()"""
content = content.replace(old_move, new_move, 1)

# Find ToolFileMove's success output and add ended before return
# Let's look at the return of ToolFileMove - it should return after writing
# We'll find the pattern: output.Success + return inside ToolFileMove context
old_filemove_end = '\toutput.Success(fmt.Sprintf("\\n📄Moved file: %s → %s", source, target))\n\treturn ToolResult{Output: *output}\n}\n\n// ToolFileDelete'
new_filemove_end = '\trepopkg.Emit(repopkg.EventFileMoveEnded, "repo-cli", repopkg.FilePayload{Path: target, From: source})\n\toutput.Success(fmt.Sprintf("\\n📄Moved file: %s → %s", source, target))\n\treturn ToolResult{Output: *output}\n}\n\n// ToolFileDelete'
content = content.replace(old_filemove_end, new_filemove_end, 1)

# ToolFileDelete: add starting + ended
old_fdel = "func ToolFileDelete(path string) ToolResult {\n\toutput := NewOutput()"
new_fdel = """func ToolFileDelete(path string) ToolResult {
\trepopkg.Emit(repopkg.EventFileDeleteStarting, "repo-cli", repopkg.FilePayload{Path: path})
\toutput := NewOutput()"""
content = content.replace(old_fdel, new_fdel, 1)

old_fdel_end = '\toutput.Success(fmt.Sprintf("\\n🗑 Deleted file: %s", path))\n\treturn ToolResult{Output: *output}\n}\n\n// ToolFileList'
new_fdel_end = '\trepopkg.Emit(repopkg.EventFileDeleteEnded, "repo-cli", repopkg.FilePayload{Path: path})\n\toutput.Success(fmt.Sprintf("\\n🗑 Deleted file: %s", path))\n\treturn ToolResult{Output: *output}\n}\n\n// ToolFileList'
content = content.replace(old_fdel_end, new_fdel_end, 1)

with open(file_path, "w") as f:
    f.write(content)

print("Done - file operations")
