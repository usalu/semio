---
slug: VSCODE-COMMAND-TREE
prompt: Show VS Code extension commands in a command/subcommand tree
status: open
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: 2026-01-03T02:39:23Z
commit: 757a4d5aa0cf14f288561eed3253d5195a96e75e
iterations:
  - prompt: Show VS Code extension commands in a command/subcommand tree
    date:
      started: 2026-01-03T02:39:23Z
      ended: 2026-01-03T02:44:53Z
    author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
    commit: 757a4d5aa0cf14f288561eed3253d5195a96e75e
    bundles:
      "@semio":
        files:
          AGENTS.md:
            sections:
              📄 js/js/sketchpad/Home.tsx:
                lines:
                  added: 2
                  removed: 0
              📄 js/js/sketchpad/elements.tsx:
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
              🎫 Ticket System [↑](#-components-):
                lines:
                  added: 6
                  removed: 0
              👥 Contributors [↑](#-components-):
                lines:
                  added: 5
                  removed: 0
              📄 Typography [↑](#-brand-):
                lines:
                  added: 0
                  removed: 1
              📒 Tickets and reports:
                lines:
                  added: 1
                  removed: 1
              📚 [@semio/docs](https://github.com/usalu/semio/tree/main/js/docs) [↑](#-components-):
                lines:
                  added: 0
                  removed: 1
              🔄 CI/CD [↑](#-development-):
                lines:
                  added: 1
                  removed: 2
              🦑 GitKraken [↑](#-git-):
                lines:
                  added: 1
                  removed: 0
              🧩 Sections Explorer [↑](#-components-):
                lines:
                  added: 3
                  removed: 0
              🧭 Command Tree [↑](#-components-):
                lines:
                  added: 5
                  removed: 0
              🧾 Code Report [↑](#-components-):
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
              Main:
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
              COMMENT STRING DETECT:
                lines:
                  added: 61
                  removed: 0
          tickets/2026/01/03/ITERATION-LINES.md:
            sections:
              ITERATION LINES:
                lines:
                  added: 61
                  removed: 0
          tickets/2026/01/03/VIOLATION-TREE-STRUCTURE.md:
            sections:
              VIOLATION TREE STRUCTURE:
                lines:
                  added: 15
                  removed: 0
          tickets/2026/01/03/VSCODE-FIX-COMMAND.md:
            sections:
              VSCODE FIX COMMAND:
                lines:
                  added: 102
                  removed: 0
          tickets/2026/01/03/VSCODE-TICKET-TOGGLE.md:
            sections:
              VSCODE TICKET TOGGLE:
                lines:
                  added: 33
                  removed: 0
---
# Previously

# Plan

# Changes