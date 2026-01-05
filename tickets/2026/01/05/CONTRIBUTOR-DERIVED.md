---
slug: CONTRIBUTOR-DERIVED
prompt: Derive contributions from tickets and file headers instead of hardcoding. Sort by ticket count. Add commits/tickets/files to VS Code extension tree.
status: open
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: 2026-01-05T14:05:02Z
commit: 612efdddc47caf10aac48cf7c57eab357e6695cd
iterations:
  - prompt: Derive contributions from tickets and file headers instead of hardcoding. Sort by ticket count. Add commits/tickets/files to VS Code extension tree.
    date:
      started: 2026-01-05T14:05:02Z
      ended: 2026-01-05T14:10:21Z
    author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
    commit: 612efdddc47caf10aac48cf7c57eab357e6695cd
    declared:
      updated:
        - path: go/repo/main.go
        - path: js/vscode/extension.ts
    bundles:
      "@semio":
        files:
          js/vscode/extension.ts:
            sections:
              Activation:
                definitions:
                  - outputChannel
                  - log
                  - message
                  - logError
                  - activate
                  - root
                  - relativePath
                  - fileUri
                  - processKey
                  - controller
                  - deactivate
                lines:
                  added: 8
                  removed: 0
              Commands:
                definitions:
                  - registerCommands
                  - editor
                  - relativePath
                  - title
                  - activeFile
                  - files
                  - slug
                  - fileArgs
                  - ticket
                  - prompt
                  - promptArg
                  - resolvedTicket
                  - root
                  - ticketUri
                  - doc
                  - github
                  - position
                  - newName
                  - parts
                  - childName
                  - confirmPath
                  - folderPath
                  - sourcePath
                  - targetPath
                  - filePath
                  - sectionPath
                  - policy
                lines:
                  added: 69
                  removed: 5
              Constants:
                definitions:
                  - runningProcesses
                  - SEMIO_KIT_LANGUAGE
                  - DIAGNOSTIC_SOURCE
                  - cachedProjects
                  - cachedRepoBaseUrl
                  - UI_STRINGS
                lines:
                  added: 6
                  removed: 0
              File Analysis:
                definitions:
                  - repoDiagnosticCollection
                  - fileViolationsMap
                  - extractFilePathFromScope
                  - shouldAnalyzeFile
                  - supportedLanguages
                  - analyzeFile
                  - root
                  - relativePath
                  - fileUri
                  - processKey
                  - controller
                  - result
                  - updateFileDiagnostics
                  - diagnosticsByUri
                  - filePath
                  - absPath
                  - uriKey
                  - line
                  - column
                  - endColumn
                  - range
                  - severity
                  - diagnostic
                  - RepoCodeActionProvider
                  - repoDiagnostics
                  - violations
                  - actions
                  - diagnosticLine
                  - policyId
                  - violation
                  - action
                  - createRepoCodeAction
                  - edit
                  - uri
                  - sortedEdits
                  - startPos
                  - endPos
                  - fixViolation
                  - command
                  - openDoc
                  - newContent
                  - fullRange
                lines:
                  added: 22
                  removed: 11
              Kit Validation:
                definitions:
                  - kitDiagnosticCollection
                  - isKitDocument
                  - basename
                  - problemToDiagnostic
                  - range
                  - diagnostic
                  - line
                  - relatedRange
                  - locationToRange
                  - text
                  - tree
                  - entityNode
                  - startPos
                  - endPos
                  - findEntityNode
                  - entityKindToArrayName
                  - arrayName
                  - arrayNode
                  - guidNode
                  - fieldNode
                  - designsNode
                  - subArrayNode
                  - typesNode
                  - findGuidRange
                  - node
                  - findNodeByGuid
                  - result
                  - validateKitDocument
                  - kit
                  - diagnostics
                  - KitCodeActionProvider
                  - kitDiagnostics
                  - actions
                  - diagnosticCode
                  - problem
                  - action
                  - createKitCodeAction
                  - fixedKit
                  - fixedJson
                  - edit
                  - fullRange
                lines:
                  added: 0
                  removed: 1
              Sidebar Views:
                definitions:
                  - globalSearchQuery
                  - globalMatchCase
                  - globalMatchWholeWord
                  - globalUseRegex
                  - matchesSearchText
                  - flags
                  - pattern
                  - regex
                  - query
                  - target
                  - wordRegex
                  - SearchViewProvider
                  - vscode
                  - searchInput
                  - matchCaseBtn
                  - matchWholeWordBtn
                  - useRegexBtn
                  - matchCase
                  - sendSearch
                  - TicketFilter
                  - TicketTreeItem
                  - TicketYearItem
                  - TicketMonthItem
                  - TicketDayItem
                  - TicketItem
                  - TicketAuthorItem
                  - TicketCommitsItem
                  - TicketCommitItem
                  - TicketsProvider
                  - filters
                  - currentIndex
                  - searchable
                  - result
                  - tickets
                  - years
                  - yearTickets
                  - months
                  - monthTickets
                  - days
                  - dayTickets
                  - children
                  - commits
                  - PolicyTreeItem
                  - PolicyItem
                  - ViolationKindGroupItem
                  - segments
                  - name
                  - ViolationKindItem
                  - PoliciesProvider
                  - groups
                  - leafKinds
                  - rest
                  - colonIndex
                  - groupName
                  - groupPath
                  - items
                  - matchingPolicies
                  - kinds
                  - filtered
                  - ContributorData
                  - ContributorTicketData
                  - ContributorCommitData
                  - ContributorLineStats
                  - ContributorTreeItem
                  - ContributorItem
                  - displayName
                  - ContributorEmailsItem
                  - ContributorEmailItem
                  - ContributorLinksItem
                  - ContributorLinkItem
                  - ContributorContributionsItem
                  - ContributorProjectsItem
                  - ContributorProjectItem
                  - ContributorTicketsItem
                  - ContributorTicketYearItem
                  - ContributorTicketMonthItem
                  - ContributorTicketDayItem
                  - ContributorTicketItem
                  - ContributorFilesItem
                  - ContributorFileFolderItem
                  - ContributorFileItem
                  - ContributorCommitsItem
                  - ContributorCommitItem
                  - ContributorsProvider
                  - root
                  - avatarPath
                  - c
                  - files
                  - folderMap
                  - folder
                  - SectionTreeItem
                  - SectionStatusItem
                  - SectionItem
                  - SectionsProvider
                  - editor
                  - relativePath
                  - SectionsDragAndDropController
                  - item
                  - raw
                  - parsed
                  - sourcePath
                  - targetPath
                  - CommandInfo
                  - CommandNode
                  - CommandTreeItem
                  - CommandGroupItem
                  - CommandItem
                  - SIDEBAR_COMMANDS
                  - CommandsProvider
                  - lower
                  - node
                  - groupMatches
                  - ticketsProvider
                  - contributorsProvider
                  - policiesProvider
                  - commandsProvider
                  - sectionsProvider
                  - registerSidebarViews
                  - searchProvider
                  - resolvedTicket
                  - filePath
                  - resolvedPath
                  - uri
                  - repoFilePath
                  - doc
                  - content
                  - functionPattern
                  - match
                  - position
                  - contributorPath
                  - extensionPath
                  - commandPattern
                  - bundles
                  - bundle
                  - projectRoot
                  - projectJson
                  - packageJson
                  - sha
                  - baseUrl
                lines:
                  added: 56
                  removed: 18
              Utilities:
                definitions:
                  - getWorkspaceRoot
                  - getRepoBinaryPath
                  - root
                  - isWindows
                  - binaryName
                  - binaryPath
                  - getRepoCommand
                  - hasRepoAccess
                  - runRepoCommand
                  - command
                  - fullCommand
                  - terminal
                  - runRepoCommandJson
                  - parsed
                  - getProjectList
                  - result
                  - getGitHubRepoBaseUrl
                  - packagePath
                  - raw
                  - repoUrl
                  - cleaned
                  - match
                  - resolveTicketData
                  - resolveTicketPath
                  - resolveCommitSha
                  - getUiString
                  - language
                  - bundle
                  - ToolResult
                  - LineStats
                  - FileStats
                  - BundleStats
                  - TicketBundles
                  - TicketIteration
                  - TicketFrontmatter
                  - TicketData
                  - PolicyData
                  - ProjectData
                  - pickTicket
                  - tickets
                  - items
                  - picked
                  - pickPolicy
                  - pickFiles
                  - files
                  - getActiveFileRelativePath
                  - editor
                  - pinDiagnosticPreview
                  - currentEditor
                  - activeTab
                lines:
                  added: 2
                  removed: 1
---
# Previously

# Plan

# Changes