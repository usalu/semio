#!/usr/bin/env python3
"""Add starting emits to contributor, draft, section, integrate, extract, export, analyze, fix, todo."""

file_path = "/workspaces/semio/repo/cli/main.go"
with open(file_path, "r") as f:
    content = f.read()

replacements = [
    # ToolContributorAdd
    (
        "func ToolContributorAdd(github string) ToolResult {\n\tcontributor, err := CreateContributor(github)",
        'func ToolContributorAdd(github string) ToolResult {\n\trepopkg.Emit(repopkg.EventContributorAddStarting, "repo-cli", repopkg.ContributorPayload{Github: github})\n\tcontributor, err := CreateContributor(github)',
    ),
    # ToolContributorRemove
    (
        "func ToolContributorRemove(github string) ToolResult {\n\toutput := NewOutput()",
        'func ToolContributorRemove(github string) ToolResult {\n\trepopkg.Emit(repopkg.EventContributorRemoveStarting, "repo-cli", repopkg.ContributorPayload{Github: github})\n\toutput := NewOutput()',
    ),
    # ToolDraftCreate - check actual signature
    (
        "func ToolDraftCreate(title string, files []string) ToolResult {\n\toutput := NewOutput()\n\tdraft, err := CreateDraft",
        'func ToolDraftCreate(title string, files []string) ToolResult {\n\trepopkg.Emit(repopkg.EventDraftCreateStarting, "repo-cli", repopkg.DraftPayload{Title: title})\n\toutput := NewOutput()\n\tdraft, err := CreateDraft',
    ),
    # ToolDraftDelete
    (
        "func ToolDraftDelete(slug string) ToolResult {\n\toutput := NewOutput()\n\tif err := DeleteDraft",
        'func ToolDraftDelete(slug string) ToolResult {\n\trepopkg.Emit(repopkg.EventDraftDeleteStarting, "repo-cli", repopkg.DraftPayload{Slug: slug})\n\toutput := NewOutput()\n\tif err := DeleteDraft',
    ),
    # ToolSectionCreate
    (
        "func ToolSectionCreate(filePath, sectionPath string) ToolResult {\n\toutput := NewOutput()",
        'func ToolSectionCreate(filePath, sectionPath string) ToolResult {\n\trepopkg.Emit(repopkg.EventSectionCreateStarting, "repo-cli", repopkg.SectionPayload{File: filePath, Name: sectionPath})\n\toutput := NewOutput()',
    ),
    # ToolSectionMove
    (
        "func ToolSectionMove(filePath, oldPath, newPath string) ToolResult {\n\toutput := NewOutput()",
        'func ToolSectionMove(filePath, oldPath, newPath string) ToolResult {\n\trepopkg.Emit(repopkg.EventSectionMoveStarting, "repo-cli", repopkg.SectionPayload{File: filePath, Name: newPath, OldName: oldPath})\n\toutput := NewOutput()',
    ),
    # ToolSectionDelete
    (
        "func ToolSectionDelete(filePath, sectionPath string) ToolResult {\n\toutput := NewOutput()",
        'func ToolSectionDelete(filePath, sectionPath string) ToolResult {\n\trepopkg.Emit(repopkg.EventSectionDeleteStarting, "repo-cli", repopkg.SectionPayload{File: filePath, Name: sectionPath})\n\toutput := NewOutput()',
    ),
    # ToolIntegrate
    (
        "func ToolIntegrate(sourcePath, targetSectionName, targetFilePath, targetParentSectionName string) ToolResult {\n\toutput := NewOutput()",
        'func ToolIntegrate(sourcePath, targetSectionName, targetFilePath, targetParentSectionName string) ToolResult {\n\trepopkg.Emit(repopkg.EventIntegrateStarting, "repo-cli", repopkg.IntegratePayload{Source: sourcePath, TargetFile: targetFilePath, TargetSection: targetSectionName})\n\toutput := NewOutput()',
    ),
    # ToolExtract
    (
        "func ToolExtract(sourceFilePath, sourceSectionName, targetFilePath string) ToolResult {\n\toutput := NewOutput()",
        'func ToolExtract(sourceFilePath, sourceSectionName, targetFilePath string) ToolResult {\n\trepopkg.Emit(repopkg.EventExtractStarting, "repo-cli", repopkg.ExtractPayload{SourceFile: sourceFilePath, SourceSection: sourceSectionName, TargetFile: targetFilePath})\n\toutput := NewOutput()',
    ),
    # ToolExport
    (
        "func ToolExport(outputPath string) ToolResult {\n\toutput := NewOutput()",
        'func ToolExport(outputPath string) ToolResult {\n\trepopkg.Emit(repopkg.EventExportStarting, "repo-cli", repopkg.FilePayload{Path: outputPath})\n\toutput := NewOutput()',
    ),
    # ToolAnalyze
    (
        "func ToolAnalyze(scopeRaw string, policyIDs []string) ToolResult {\n\toutput := NewOutput()",
        'func ToolAnalyze(scopeRaw string, policyIDs []string) ToolResult {\n\trepopkg.Emit(repopkg.EventAnalyzeStarting, "repo-cli", repopkg.FolderPayload{Path: scopeRaw})\n\toutput := NewOutput()',
    ),
    # ToolFix
    (
        "func ToolFix(scopeRaw string) ToolResult {\n\toutput := NewOutput()",
        'func ToolFix(scopeRaw string) ToolResult {\n\trepopkg.Emit(repopkg.EventFixStarting, "repo-cli", repopkg.FolderPayload{Path: scopeRaw})\n\toutput := NewOutput()',
    ),
]

count = 0
for old, new in replacements:
    if old in content:
        content = content.replace(old, new, 1)
        count += 1
        print(f"Replaced: {old[:60].strip()!r}")
    else:
        print(f"NOT FOUND: {old[:60].strip()!r}")

with open(file_path, "w") as f:
    f.write(content)

print(f"\nTotal replacements: {count}/{len(replacements)}")
