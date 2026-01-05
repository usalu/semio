---
slug: CONTRIBUTORS-FROM-TICKETS-FILES
prompt: contributions should be derived from the tickets (frontmatter) and files (headers) instead of being hardcoded. Sort contributors by amount of tickets contributed. Extend the contributions list command. Adjust also the vscode extension tree.
status: open
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
    created: "2026-01-03T03:08:28Z"
commit: 757a4d5aa0cf14f288561eed3253d5195a96e75e
iterations:
    - prompt: contributions should be derived from the tickets (frontmatter) and files (headers) instead of being hardcoded. Sort contributors by amount of tickets contributed. Extend the contributions list command. Adjust also the vscode extension tree.
      date:
        started: "2026-01-03T03:08:28Z"
        ended: "2026-01-03T03:28:18Z"
      author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
      commit: 757a4d5aa0cf14f288561eed3253d5195a96e75e
      bundles:
        '@semio':
            files:
                AGENTS.md:
                    sections:
                        "\U0001F4C4 js/js/sketchpad/Home.tsx":
                            lines:
                                added: 2
                                removed: 0
                        "\U0001F4C4 js/js/sketchpad/elements.tsx":
                            lines:
                                added: 4
                                removed: 0
                        Code Hygiene:
                            lines:
                                added: 2
                                removed: 0
                        Codebase:
                            lines:
                                added: 2
                                removed: 2
                        Contributor:
                            lines:
                                added: 6
                                removed: 0
                        Fix Comparison Notes:
                            lines:
                                added: 1
                                removed: 0
                        Format:
                            lines:
                                added: 0
                                removed: 2
                        Frontmatter Format:
                            lines:
                                added: 4
                                removed: 0
                        Interaction State:
                            lines:
                                added: 0
                                removed: 1
                        MCP Server:
                            lines:
                                added: 3
                                removed: 0
                        MCP Tools:
                            lines:
                                added: 6
                                removed: 0
                        Repo CLI:
                            lines:
                                added: 0
                                removed: 2
                        Scope Hierarchy:
                            lines:
                                added: 0
                                removed: 2
                        Ticket:
                            lines:
                                added: 12
                                removed: 0
                        Updating Metabolism Assets:
                            lines:
                                added: 1
                                removed: 0
                        Using TypeScript fallback:
                            lines:
                                added: 21
                                removed: 7
                        VS Code Extension:
                            lines:
                                added: 7
                                removed: 0
                        Validation Constraints:
                            lines:
                                added: 8
                                removed: 0
                README.md:
                    sections:
                        ?? MCP Tool Gateway [↑](#-components-):
                            lines:
                                added: 5
                                removed: 0
                        ♻️ Ecosystems [↑](#-overview):
                            lines:
                                added: 1
                                removed: 0
                        ⚖️ License:
                            lines:
                                added: 1
                                removed: 0
                        ⚖️ Principles [↑](#-repo-):
                            lines:
                                added: 2
                                removed: 3
                        ✅ Validation System [↑](#-components-):
                            lines:
                                added: 0
                                removed: 1
                        "\U0001F3AB Ticket System [↑](#-components-)":
                            lines:
                                added: 6
                                removed: 0
                        "\U0001F465 Contributors [↑](#-components-)":
                            lines:
                                added: 5
                                removed: 0
                        "\U0001F4C4 Typography [↑](#-brand-)":
                            lines:
                                added: 0
                                removed: 1
                        "\U0001F4D2 Tickets and reports":
                            lines:
                                added: 1
                                removed: 1
                        "\U0001F4DA [@semio/docs](https://github.com/usalu/semio/tree/main/js/docs) [↑](#-components-)":
                            lines:
                                added: 0
                                removed: 1
                        "\U0001F504 CI/CD [↑](#-development-)":
                            lines:
                                added: 1
                                removed: 2
                        "\U0001F991 GitKraken [↑](#-git-)":
                            lines:
                                added: 1
                                removed: 0
                        "\U0001F9E9 Sections Explorer [↑](#-components-)":
                            lines:
                                added: 3
                                removed: 0
                        "\U0001F9ED Command Tree [↑](#-components-)":
                            lines:
                                added: 5
                                removed: 0
                        "\U0001F9FE Code Report [↑](#-components-)":
                            lines:
                                added: 1
                                removed: 0
                        Skip Mechanism:
                            lines:
                                added: 1
                                removed: 0
                        Tickets:
                            lines:
                                added: 1
                                removed: 0
                        VS Code Extension [↑](#%EF%B8%8F-products-):
                            lines:
                                added: 9
                                removed: 1
                        Violation Diagnostics:
                            lines:
                                added: 1
                                removed: 0
                        What Preflight Does:
                            lines:
                                added: 0
                                removed: 1
                go/repo/main.go:
                    sections:
                        _root:
                            lines:
                                added: 3
                                removed: 0
                        Commands:
                            definitions:
                                - Execute
                                - rootCmd
                                - init
                                - analyzeCmd
                                - fixCmd
                                - policyCmd
                                - policyListCmd
                                - policyCheckCmd
                                - policyViolationCmd
                                - policyViolationListCmd
                                - ticketCmd
                                - ticketCreateCmd
                                - ticketListCmd
                                - year
                                - ticketReadCmd
                                - ticketIterateCmd
                                - ticketIterateStartCmd
                                - ticketIterateEndCmd
                                - ticketFinishCmd
                                - ticketReopenCmd
                                - ticketMigrateCmd
                                - contributorCmd
                                - contributorAddCmd
                                - contributorListCmd
                                - contributorRemoveCmd
                                - projectCmd
                                - projectListCmd
                                - projectTreeCmd
                                - folderCmd
                                - folderCreateCmd
                                - folderMoveCmd
                                - folderDeleteCmd
                                - folderListCmd
                                - folderTreeCmd
                                - fileCmd
                                - fileCreateCmd
                                - fileMoveCmd
                                - fileDeleteCmd
                                - fileListCmd
                                - fileTreeCmd
                                - sectionCmd
                                - sectionCreateCmd
                                - sectionMoveCmd
                                - sectionDeleteCmd
                                - sectionListCmd
                                - sectionTreeCmd
                                - definitionCmd
                                - definitionListCmd
                                - definitionTreeCmd
                                - updateMetabolismCmd
                                - outputResult
                                - AnalyzeFile
                                - ToolAnalyze
                                - scopeRaws
                                - allViolations
                                - bundles
                                - projectsLoaded
                                - ToolFix
                                - fixable
                                - ToolPolicyList
                                - ToolPolicyCheck
                                - ToolPolicyViolationList
                                - foundPolicy
                                - ToolTicketCreate
                                - ToolTicketList
                                - ToolTicketRead
                                - ToolTicketIterateStart
                                - ToolTicketIterateEnd
                                - ToolTicketFinish
                                - ToolTicketReopen
                                - needsBundleMigration
                                - MigrateTicket
                                - ToolTicketMigrate
                                - ToolContributorAdd
                                - ToolContributorList
                                - ToolContributorRemove
                                - ToolProjectList
                                - ToolProjectTree
                                - ToolFolderCreate
                                - ToolFolderMove
                                - ToolFolderDelete
                                - ToolFolderList
                                - relPaths
                                - filtered
                                - ToolFolderTree
                                - printTree
                                - items
                                - ToolFileCreate
                                - generateFileHeader
                            lines:
                                added: 50
                                removed: 3
                        Contributors:
                            definitions:
                                - GetContributorsDir
                                - GetContributorPath
                                - GetContributorJsonPath
                                - GetContributorAvatarPath
                                - GetContributorAvatarRoundPath
                                - ContributorExists
                                - CreateContributor
                                - ReadContributor
                                - contributor
                                - SaveContributor
                                - ContributorContributionState
                                - ParseContributorIdentity
                                - ResolveContributorGithub
                                - GetGitCommitTitle
                                - ListContributors
                                - contributors
                                - projectRoots
                                - RemoveContributor
                                - DownloadGitHubAvatar
                                - AddContributorProject
                            lines:
                                added: 41
                                removed: 17
                        Nx:
                            definitions:
                                - GetProjectNames
                                - names
                                - GetProjectDetails
                                - config
                                - GetProjects
                                - RunNxTarget
                                - filterGitIgnored
                                - filtered
                                - ScopeToFiles
                                - files
                                - err
                            lines:
                                added: 4
                                removed: 0
                        Policies:
                            definitions:
                                - PolicyFunc
                                - RegisteredPolicy
                                - policyMetas
                                - policyFuncs
                                - getRegisteredPolicies
                                - policies
                                - GetRegisteredPolicies
                                - PolicyContext
                                - NewPolicyContext
                                - Files
                                - ReadText
                                - Sections
                                - CreateViolation
                                - randomString
                                - letters
                                - CheckPolicies
                                - violations
                                - policiesToRun
                                - matchesScope
                                - headerPolicy
                                - headerSection
                                - sectionPolicy
                                - stackItem
                                - stack
                                - checkSection
                                - markCovered
                                - CommentTemplateState
                                - CommentScanState
                                - InTemplateRaw
                                - commentPolicy
                                - truncate
                                - codePolicy
                                - devDocsPolicy
                                - fileSections
                                - folderSections
                                - sketchpadPolicy
                            lines:
                                added: 339
                                removed: 93
                        Sections:
                            definitions:
                                - ParseCodeSections
                                - stack
                                - roots
                                - lang
                                - ParseMarkdownSections
                                - sections
                                - stackItem
                                - ParseSections
                                - FindSection
                            lines:
                                added: 22
                                removed: 41
                        Tickets:
                            definitions:
                                - GetTicketsDir
                                - GetTicketPath
                                - CreateTicket
                                - declaredFiles
                                - ReadTicket
                                - parseFrontmatter
                                - frontmatter
                                - SaveTicket
                                - ListTickets
                                - tickets
                                - years
                                - months
                                - days
                                - StartIteration
                                - CollectTicketFilePaths
                                - BuildGitDiffArgs
                                - GetGitDiffFileLineStats
                                - ResolveBundleForPath
                                - longestRoot
                                - bundleName
                                - BuildTicketBundles
                                - GetGitDiffSectionLineStats
                                - sections
                                - lineChange
                                - parseGitDiffHunks
                                - changes
                                - currentAddLine
                                - flattenSections
                                - result
                                - flatten
                                - ExtractDefinitionsFromSection
                                - section
                                - defs
                                - EndIteration
                                - FinishTicket
                                - ReopenTicket
                                - CanCloseTicket
                                - reasons
                            lines:
                                added: 486
                                removed: 32
                        Types:
                            definitions:
                                - ScopeKind
                                - Scope
                                - ViolationPriority
                                - TextEdit
                                - Fix
                                - Violation
                                - Bundle
                                - SectionInfo
                                - DefinitionKind
                                - DefinitionInfo
                                - TicketStatus
                                - LineStats
                                - SectionStats
                                - FileStats
                                - BundleStats
                                - TicketBundles
                                - Language
                                - languages
                                - GetLanguage
                                - TicketIterationFiles
                                - FileLineStats
                                - TicketDate
                                - TicketIteration
                                - TicketFiles
                                - TicketFrontmatter
                                - TicketDateCreated
                                - Ticket
                                - ViolationKind
                                - ViolationKindMeta
                                - violationKindMetas
                                - GetViolationKindMeta
                                - PolicyMeta
                                - AnalyzeReport
                                - Summary
                                - FileCache
                                - OutputType
                                - OutputLine
                                - CommandOutput
                                - ToolResult
                                - Contributor
                                - ContributorTicket
                                - ContributorCommit
                                - ContributorContributions
                            lines:
                                added: 145
                                removed: 83
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
                                added: 18
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
                                added: 77
                                removed: 0
                        Constants:
                            definitions:
                                - runningProcesses
                                - SEMIO_KIT_LANGUAGE
                                - DIAGNOSTIC_SOURCE
                                - cachedProjects
                                - cachedRepoBaseUrl
                                - UI_STRINGS
                            lines:
                                added: 19
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
                                added: 45
                                removed: 2
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
                                added: 2
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
                                added: 427
                                removed: 48
                        Types:
                            definitions:
                                - TextEdit
                                - AutoFix
                                - Violation
                                - AnalyzeReport
                                - SectionInfo
                            lines:
                                added: 9
                                removed: 0
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
                                added: 88
                                removed: 17
                js/vscode/package.json:
                    sections:
                        _root:
                            lines:
                                added: 32
                                removed: 0
                prompts/ueli.md:
                    sections:
                        Prompt history:
                            lines:
                                added: 184
                                removed: 13
                        State managment:
                            lines:
                                added: 1
                                removed: 1
                tickets/2026/01/03/COMMENT-STRING-DETECT.md:
                    sections:
                        _root:
                            lines:
                                added: 61
                                removed: 0
                tickets/2026/01/03/CONTRIBUTORS-FROM-TICKETS-FILES.md:
                    sections:
                        _root:
                            lines:
                                added: 93
                                removed: 0
                tickets/2026/01/03/COPILOT-MCP-VALIDATE-ERROR.md:
                    sections:
                        _root:
                            lines:
                                added: 69
                                removed: 0
                tickets/2026/01/03/ITERATION-LINES.md:
                    sections:
                        _root:
                            lines:
                                added: 61
                                removed: 0
                tickets/2026/01/03/TICKET-DIFF-FILES-SCOPE.md:
                    sections:
                        _root:
                            lines:
                                added: 70
                                removed: 0
                tickets/2026/01/03/VIOLATION-TREE-STRUCTURE.md:
                    sections:
                        _root:
                            lines:
                                added: 15
                                removed: 0
                tickets/2026/01/03/VSCODE-COMMAND-TREE.md:
                    sections:
                        _root:
                            lines:
                                added: 69
                                removed: 0
                tickets/2026/01/03/VSCODE-FIX-COMMAND.md:
                    sections:
                        _root:
                            lines:
                                added: 102
                                removed: 0
                tickets/2026/01/03/VSCODE-PROBLEM-PREVIEW.md:
                    sections:
                        _root:
                            lines:
                                added: 117
                                removed: 0
                tickets/2026/01/03/VSCODE-TICKET-TOGGLE.md:
                    sections:
                        _root:
                            lines:
                                added: 33
                                removed: 0
      files:
        updated:
            - path: AGENTS.md
              lines:
                added: 57
                removed: 8
            - path: README.md
              lines:
                added: 22
                removed: 4
            - path: go/repo/main.go
              lines:
                added: 843
                removed: 188
            - path: js/vscode/extension.ts
              lines:
                added: 441
                removed: 46
            - path: js/vscode/package.json
              lines:
                added: 32
                removed: 0
            - path: prompts/ueli.md
              lines:
                added: 78
                removed: 1
        created:
            - path: tickets/2026/01/03/COMMENT-STRING-DETECT.md
              lines:
                added: 61
                removed: 0
            - path: tickets/2026/01/03/CONTRIBUTORS-FROM-TICKETS-FILES.md
              lines:
                added: 35
                removed: 0
            - path: tickets/2026/01/03/COPILOT-MCP-VALIDATE-ERROR.md
              lines:
                added: 69
                removed: 0
            - path: tickets/2026/01/03/ITERATION-LINES.md
              lines:
                added: 61
                removed: 0
            - path: tickets/2026/01/03/TICKET-DIFF-FILES-SCOPE.md
              lines:
                added: 70
                removed: 0
            - path: tickets/2026/01/03/VIOLATION-TREE-STRUCTURE.md
              lines:
                added: 15
                removed: 0
            - path: tickets/2026/01/03/VSCODE-COMMAND-TREE.md
              lines:
                added: 69
                removed: 0
            - path: tickets/2026/01/03/VSCODE-FIX-COMMAND.md
              lines:
                added: 102
                removed: 0
            - path: tickets/2026/01/03/VSCODE-PROBLEM-PREVIEW.md
              lines:
                added: 117
                removed: 0
            - path: tickets/2026/01/03/VSCODE-TICKET-TOGGLE.md
              lines:
                added: 33
                removed: 0
      lines:
        added: 2105
        removed: 247
---
# Previously

# Plan

# Changes