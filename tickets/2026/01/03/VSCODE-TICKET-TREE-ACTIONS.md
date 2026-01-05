---
slug: VSCODE-TICKET-TREE-ACTIONS
prompt: 'vscode extension: Remove Open Ticket button from tickets. Instead add reopen and close icons and execute the command once pressed. Remove status emoji from ticket. Add commit tree item. Just show description on ticket tree item hover.'
status: closed
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
    created: "2026-01-03T00:11:53Z"
    finished: "2026-01-03T00:33:52Z"
commit: 97d1f2878938222b14d1919804fc3a4918a8f8eb
iterations:
    - prompt: Remove Open Ticket button from tickets
      date:
        started: "2026-01-03T00:12:02Z"
        ended: "2026-01-03T00:17:05Z"
      author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
      commit: 97d1f2878938222b14d1919804fc3a4918a8f8eb
      bundles:
        '@semio':
            files:
                go/repo/main.go:
                    sections:
                        _root:
                            lines:
                                added: 3
                                removed: 1
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
                                added: 105
                                removed: 0
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
                                added: 50
                                removed: 2
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
                                added: 20
                                removed: 1
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
                                added: 446
                                removed: 132
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
                                added: 35
                                removed: 25
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
                                added: 509
                                removed: 60
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
                                added: 351
                                removed: 98
                        Utils:
                            definitions:
                                - rootDir
                                - init
                                - findRepoRoot
                                - getGitignorePatterns
                                - isGitIgnored
                                - policyAppliesToScope
                                - isSourceFile
                                - GetRootDir
                                - SetRootDir
                                - NormalizePath
                                - EnsureDir
                                - GetRelativePath
                                - ReadTextFile
                                - WriteTextFile
                                - WriteJSONFile
                                - ReadJSONFile
                                - FileExists
                                - IsDir
                                - LoadGitignore
                                - patterns
                                - SimpleGlob
                                - gitignorePatterns
                                - err
                                - files
                                - ISOTimestamp
                                - FormatDate
                                - PadNumber
                                - Slugify
                                - ExecCommand
                                - stdoutBuf
                                - GetGitAuthor
                                - GetGitCommit
                                - GetGitIgnoredSet
                                - GetLanguageFromPath
                                - NewOutput
                                - Info
                                - Success
                                - Error
                                - Warn
                                - Plain
                                - Print
                                - ListDirEntries
                                - names
                                - WalkDir
                                - ParseScope
                                - ReadLines
                                - lines
                            lines:
                                added: 8
                                removed: 56
                js/vscode/extension.ts:
                    sections:
                        _root:
                            lines:
                                added: 1
                                removed: 1
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
                                added: 46
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
                                added: 108
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
                                removed: 1
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
                                added: 73
                                removed: 7
                        Imports:
                            definitions:
                                - execAsync
                            lines:
                                added: 1
                                removed: 1
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
                                added: 25
                                removed: 16
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
                                added: 653
                                removed: 98
                        Types:
                            definitions:
                                - TextEdit
                                - AutoFix
                                - Violation
                                - AnalyzeReport
                                - SectionInfo
                            lines:
                                added: 9
                                removed: 16
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
                                added: 127
                                removed: 71
                js/vscode/package.json:
                    sections: {}
      files:
        updated:
            - path: go/repo/main.go
            - path: js/vscode/extension.ts
            - path: js/vscode/package.json
    - prompt: Remove status emoji from ticket
      date:
        started: "2026-01-03T00:17:42Z"
        ended: "2026-01-03T00:19:58Z"
      author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
      commit: 97d1f2878938222b14d1919804fc3a4918a8f8eb
      bundles:
        '@semio':
            files:
                js/vscode/extension.ts:
                    sections:
                        _root:
                            lines:
                                added: 1
                                removed: 1
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
                                added: 46
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
                                added: 108
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
                                removed: 1
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
                                added: 73
                                removed: 7
                        Imports:
                            definitions:
                                - execAsync
                            lines:
                                added: 1
                                removed: 1
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
                                added: 25
                                removed: 16
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
                                added: 653
                                removed: 98
                        Types:
                            definitions:
                                - TextEdit
                                - AutoFix
                                - Violation
                                - AnalyzeReport
                                - SectionInfo
                            lines:
                                added: 9
                                removed: 16
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
                                added: 127
                                removed: 71
      files:
        updated:
            - path: js/vscode/extension.ts
    - prompt: Add commit tree item
      date:
        started: "2026-01-03T00:20:05Z"
        ended: "2026-01-03T00:20:57Z"
      author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
      commit: 97d1f2878938222b14d1919804fc3a4918a8f8eb
      bundles:
        '@semio':
            files:
                js/vscode/extension.ts:
                    sections:
                        _root:
                            lines:
                                added: 1
                                removed: 1
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
                                added: 46
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
                                added: 108
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
                                removed: 1
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
                                added: 73
                                removed: 7
                        Imports:
                            definitions:
                                - execAsync
                            lines:
                                added: 1
                                removed: 1
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
                                added: 25
                                removed: 16
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
                                added: 653
                                removed: 98
                        Types:
                            definitions:
                                - TextEdit
                                - AutoFix
                                - Violation
                                - AnalyzeReport
                                - SectionInfo
                            lines:
                                added: 9
                                removed: 16
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
                                added: 127
                                removed: 71
      files:
        updated:
            - path: js/vscode/extension.ts
    - prompt: Just show description on ticket tree item hover
      date:
        started: "2026-01-03T00:21:06Z"
        ended: "2026-01-03T00:21:20Z"
      author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
      commit: 97d1f2878938222b14d1919804fc3a4918a8f8eb
      bundles:
        '@semio':
            files:
                js/vscode/extension.ts:
                    sections:
                        _root:
                            lines:
                                added: 1
                                removed: 1
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
                                added: 46
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
                                added: 108
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
                                removed: 1
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
                                added: 73
                                removed: 7
                        Imports:
                            definitions:
                                - execAsync
                            lines:
                                added: 1
                                removed: 1
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
                                added: 25
                                removed: 16
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
                                added: 653
                                removed: 98
                        Types:
                            definitions:
                                - TextEdit
                                - AutoFix
                                - Violation
                                - AnalyzeReport
                                - SectionInfo
                            lines:
                                added: 9
                                removed: 16
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
                                added: 127
                                removed: 71
      files:
        updated:
            - path: js/vscode/extension.ts
---
# Previously

# Plan

# Changes
- Updated VS Code ticket actions, hover tooltip behavior, and commit tree items.
- Added repo ticket reopen command.
- Documented ticket tree behavior in README and AGENTS.
