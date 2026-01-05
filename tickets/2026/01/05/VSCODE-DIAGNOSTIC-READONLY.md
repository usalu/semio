---
slug: VSCODE-DIAGNOSTIC-READONLY
prompt: Fix VSCode diagnostics opening files as read-only preview instead of editable files when clicking on semio violation diagnostics.
status: open
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
    created: "2026-01-05T11:12:30Z"
commit: 393dfeadd9c012eb01d37dad9cd10065832c6c1c
model: claude-opus-4-5
iterations:
    - prompt: Fix VSCode diagnostics opening files as read-only preview instead of editable files when clicking on semio violation diagnostics.
      model: claude-opus-4-5
      date:
        started: "2026-01-05T11:12:30Z"
        ended: "2026-01-05T11:19:30Z"
      author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
      commit: 393dfeadd9c012eb01d37dad9cd10065832c6c1c
      bundles:
        '@semio':
            files:
                contributors/usalu/contributor.json:
                    sections:
                        _root:
                            lines:
                                added: 3
                                removed: 2
                go/repo/main.go:
                    sections:
                        _root:
                            lines:
                                added: 1
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
                                added: 23
                                removed: 23
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
                                added: 22
                                removed: 5
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
                                added: 117
                                removed: 10
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
                                added: 7
                                removed: 6
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
                                added: 26
                                removed: 39
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
                                added: 71
                                removed: 22
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
                                added: 16
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
                                removed: 4
                        Constants:
                            definitions:
                                - runningProcesses
                                - SEMIO_KIT_LANGUAGE
                                - DIAGNOSTIC_SOURCE
                                - cachedProjects
                                - cachedRepoBaseUrl
                                - UI_STRINGS
                            lines:
                                added: 16
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
                                added: 25
                                removed: 8
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
                                removed: 2
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
                                added: 104
                                removed: 15
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
                                added: 18
                                removed: 8
                prompts/ueli.md:
                    sections:
                        Prompt history:
                            lines:
                                added: 106
                                removed: 14
                        State managment:
                            lines:
                                added: 1
                                removed: 1
                reports/playwright.json:
                    sections:
                        _root:
                            lines:
                                added: 23
                                removed: 549
      files:
        updated:
            - path: contributors/usalu/contributor.json
              lines:
                added: 3
                removed: 2
            - path: go/repo/main.go
              lines:
                added: 177
                removed: 11
            - path: js/vscode/extension.ts
              lines:
                added: 10
                removed: 4
            - path: prompts/ueli.md
              lines:
                added: 3
                removed: 1
            - path: reports/playwright.json
              lines:
                added: 23
                removed: 549
      lines:
        added: 216
        removed: 567
    - prompt: Continue investigating - the read-only preview issue persists. Need to find the root cause.
      model: claude-opus-4-5
      date:
        started: "2026-01-05T11:56:37Z"
        ended: "2026-01-05T12:00:03Z"
      author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
      commit: 393dfeadd9c012eb01d37dad9cd10065832c6c1c
      declared:
        updated:
            - path: js/vscode/extension.ts
      bundles:
        '@semio':
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
                                added: 16
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
                                removed: 4
                        Constants:
                            definitions:
                                - runningProcesses
                                - SEMIO_KIT_LANGUAGE
                                - DIAGNOSTIC_SOURCE
                                - cachedProjects
                                - cachedRepoBaseUrl
                                - UI_STRINGS
                            lines:
                                added: 16
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
                                added: 25
                                removed: 8
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
                                removed: 2
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
                                added: 104
                                removed: 15
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
                                added: 18
                                removed: 8
      files:
        updated:
            - path: js/vscode/extension.ts
              lines:
                added: 17
                removed: 7
      lines:
        added: 17
        removed: 7
---
# Previously

# Plan

# Changes