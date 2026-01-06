---
slug: ITERATION-LINES
prompt: Compute git line stats on iteration and ticket end; derive files and lines instead of null.
status: closed
author: Ueli Saluz <ueli.saluz@iek.uni-hannover.de>
date:
  created: 2026-01-03T01:16:15Z
  finished: 2026-01-03T01:23:31Z
commit: 757a4d5aa0cf14f288561eed3253d5195a96e75e
iterations:
  - prompt: Compute git-derived line stats on iteration end and ticket end, plus file aggregation.
    date:
      started: 2026-01-03T01:16:18Z
      ended: 2026-01-03T01:22:59Z
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
---
# Previously

# Plan

# Changes
