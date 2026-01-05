---
slug: SKETCHPAD-POLICY
prompt: 'Add a new policy for sketchpad covering: 1) Third party imports must only be in elements.tsx which reexports reusable UI elements, 2) State management rules - single state machine (createMachine once, no createActor, Yjs only for kit data sync not app state), 3) UI elements must use triadic hooks pattern [state, setState, canSetState]=useSELECTOR()'
status: open
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
    created: "2026-01-03T10:14:40Z"
commit: 393dfeadd9c012eb01d37dad9cd10065832c6c1c
model: claude-opus-4-5
iterations:
    - prompt: 'Add a new policy for sketchpad covering: 1) Third party imports must only be in elements.tsx which reexports reusable UI elements, 2) State management rules - single state machine (createMachine once, no createActor, Yjs only for kit data sync not app state), 3) UI elements must use triadic hooks pattern [state, setState, canSetState]=useSELECTOR()'
      model: claude-opus-4-5
      date:
        started: "2026-01-03T10:14:40Z"
        ended: "2026-01-03T10:20:12Z"
      author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
      commit: 393dfeadd9c012eb01d37dad9cd10065832c6c1c
      bundles:
        '@semio':
            files:
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
      files:
        updated:
            - path: go/repo/main.go
              lines:
                added: 177
                removed: 11
            - path: prompts/ueli.md
              lines:
                added: 1
                removed: 1
      lines:
        added: 178
        removed: 12
---
# Previously

# Plan

# Changes