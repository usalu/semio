# 💯 Specs

## [🧰semiorepo⌨️cli](semiorepo://bundle/semio-repo/cli)

## hooks

### git

#### commit

##### starting

##### ended

### agent

#### started

##### vscode-chat

```mermaid
sequenceDiagram
    vscode->>+cli: SessionStart | SubagentStart

```

##### windsurf-chat

##### cursor-chat

##### claude-code

##### droid

#### ended

##### vscode-chat

##### windsurf-chat

##### cursor-chat

##### claude-code

##### droid

#### prompt

##### submit

###### vscode-chat

###### windsurf-chat

###### cursor-chat

###### claude-code

###### droid

#### compacting

##### vscode-chat

##### windsurf-chat

##### cursor-chat

##### claude-code

##### droid

#### tool

##### starting

###### vscode-chat

###### windsurf-chat

###### cursor-chat

###### claude-code

###### droid

##### ended

###### vscode-chat

###### windsurf-chat

###### cursor-chat

###### claude-code

###### droid

##### plan

###### updating

####### vscode-chat

####### windsurf-chat

####### cursor-chat

####### claude-code

####### droid

## [🧰semiorepo📚go](semiorepo://bundle/semio-repo/go)

- Event kinds and payloads are the single source of truth for CLI→server communication.
- All changing interactions (ticket, goal, contributor, todo, commit) emit events with consistent schema.

## [🧰semiorepo🛂sqlite](semiorepo://bundle/semio-repo/sqlite)

```mermaid
erDiagram
    contributor ||--o{ commit : commits
    contributor ||--o{ ticket : opens
    commit ||--o{ repo : belongs_to
    repo ||--o{ folder : contains
    folder ||--o{ file : contains
    folder ||--o{ bundle : contains
    file ||--o{ section : contains
    section ||--o{ definition : contains
    CONTRIBUTOR {
        int id PK
        string github
        string name
        string avatar
    }
    COMMIT {
        int id PK
        string sha
        string message
        int contributor_id FK
        string date
    }
    REPO {
        int id PK
        string sha FK
        string name
    }
    FOLDER {
        int id PK
        int repo_id FK
        int parent_id FK
        string name
        int bundle_id FK
    }
    FILE {
        int id PK
        int parent_folder_id FK
        string name
        string extension
        int bundle_id FK
        int lines
    }
    BUNDLE {
        int id PK
        string kind
        int folder_id FK
    }
    SECTION {
        int id PK
        string name
        string path
        int file_id FK
        int parent_id FK
        int start_line
        int end_line
        int start_column
        int end_column
    }
    DEFINITION {
        int id PK
        string name
        string kind
        int file_id FK
        int section_id FK
        int start_line
        int end_line
        int start_column
        int end_column
    }
```

## [🧰semiorepo🖱️vscode](semiorepo://bundle/semio-repo/vscode)

## Sidebar

The semio-repo sidebar MUST expose exactly two views: Monorepo and Filter.

The Filter view MUST represent each filter kind as a single item and expose filter options as view item menu actions.

Filter view items MUST render emoji plus name labels with tooltip descriptions; filter option menu actions MUST use emoji-only labels and MUST NOT use codeicons.

Filter state MUST apply globally to all Monorepo tree branches.

Monorepo root nodes MUST expand to show children for Projects, Goals, Tickets, Policies, Contributors, and Commits.

## Tickets

Ticket tree items expose inline close and reopen actions that apply to the selected ticket based on status.

Ticket tree hovers show only the ticket description.

Ticket creation prompts for LLM and ticket UI selections.

Ticket tree items list commit entries as child nodes.

Ticket commands collect title/prompt/LLM for open, prompt/LLM for reopen, and operate on YYYY/MM/DD/SLUG ticket identifiers.

Ticket detail views consume git-derived per-file and total line stats stored on interactions and ticket close.

## Commands

Command trees mirror the CLI command and subcommand hierarchy; matching a command group keeps its subtree visible.

## Diagnostics

Problem list diagnostics open in pinned editor tabs for immediate saves.

Repo diagnostics and trees are driven by repo CLI ignore rules for gitignored files and repo directory content.

## Contributors

Contributor tree items list emails with mailto actions, links with external navigation, and contribution nodes with line summary descriptions.

Contributor contributions are grouped into commits, bundles, tickets (year/month/day), and files (folder/file) with navigation actions and inline ticket close/reopen actions.

## Sections View

The built-in Explorer hosts the Sections view; selecting a section navigates to it, F2 renames, drag-and-drop moves sections, JSON keys surface as sections, and inline actions create child sections, rename sections, and delete sections via repo commands.

The Sections view resolves the active file's section tree with line ranges so navigation and section actions match the current editor content.

Monorepo section tree rendering MUST include only section-typed section children and MUST exclude definition-typed children from section rows.

## General

Ticket tooling treats temporary artifacts as part of the active ticket workspace.

Devcontainer setup uninstalls any existing semio-repo extension, clears stale VS Code and Cursor caches, then installs the workspace extension for VS Code, Cursor, Windsurf, and Antigravity on attach without manual installation actions, validating installs per detected editor IPC hook CLI and falling back to extensions directories with extensions.json registration on WSL-only CLI responses.

Extension engine compatibility targets the lowest supported editor version so Cursor accepts the packaged VSIX.

Sidebar view registration keeps a single filter view and monorepo view instance wired to the shared filter state.

## [🧰semiorepo⌨️cli💻maingo🔖mermaid](semiorepo://section/Mermaid)

mermaidEscapeLabel MUST escape double quotes in mermaid labels.

## [🧰semiorepo⌨️cli💻maingo🔖sections](semiorepo://section/Sections)

ParseCodeSections MUST return an error when the input is malformed.

## [🧰semiorepo⌨️cli💻maingo🔖tickets](semiorepo://section/Tickets)

GetTicketsDir MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🔖queryresolvers](semiorepo://section/Query%20Resolvers)

Query MUST execute the query and return matching results.

## [🧰semiorepo⌨️cli💻maingo🔖mutationresolvers](semiorepo://section/Mutation%20Resolvers)

Mutation MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🔖missingutilities](semiorepo://section/Missing%20Utilities)

ScopeToFiles MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🔖resolvermethods](semiorepo://section/Resolver%20Methods)

Bundles MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🔖missingtoolfunctions](semiorepo://section/Missing%20Tool%20Functions)

ToolAnalyze MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🔖fileutilities](semiorepo://section/File%20Utilities)

MoveFile MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🔖goals](semiorepo://section/Goals)

GetRepoGoalsDir MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🔖todos](semiorepo://section/Todos)

GetTodos MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️newengine](semiorepo://definition/semio-repo/cli/main.go/NewEngine)

NewEngine MUST initialize all required fields and return a valid Engine.

## [🧰semiorepo⌨️cli💻maingo🛠️run](semiorepo://definition/semio-repo/cli/main.go/Run)

Run MUST emit start, result or error, and done events in order.

## [🧰semiorepo⌨️cli💻maingo🛠️isjson](semiorepo://definition/semio-repo/cli/main.go/IsJSON)

IsJSON MUST return true only when the condition is met.

## [🧰semiorepo⌨️cli💻maingo🛠️ismarkdown](semiorepo://definition/semio-repo/cli/main.go/IsMarkdown)

IsMarkdown MUST return true only when the condition is met.

## [🧰semiorepo⌨️cli💻maingo🛠️istext](semiorepo://definition/semio-repo/cli/main.go/IsText)

IsText MUST return true only when the condition is met.

## [🧰semiorepo⌨️cli💻maingo🛠️error](semiorepo://definition/semio-repo/cli/main.go/Error)

Error MUST return a formatted string representation.

## [🧰semiorepo⌨️cli💻maingo🛠️newroot](semiorepo://definition/semio-repo/cli/main.go/NewRoot)

NewRoot MUST initialize all required fields and return a valid Root.

## [🧰semiorepo⌨️cli💻maingo🛠️newrootwithconfig](semiorepo://definition/semio-repo/cli/main.go/NewRootWithConfig)

NewRootWithConfig MUST initialize all required fields and return a valid RootWithConfig.

## [🧰semiorepo⌨️cli💻maingo🛠️execute](semiorepo://definition/semio-repo/cli/main.go/Execute)

Execute MUST delegate to the root command and propagate errors.

## [🧰semiorepo⌨️cli💻maingo🛠️hasonlykinds](semiorepo://definition/semio-repo/cli/main.go/HasOnlyKinds)

HasOnlyKinds MUST return true only when the property is present.

## [🧰semiorepo⌨️cli💻maingo🛠️iskindvisible](semiorepo://definition/semio-repo/cli/main.go/IsKindVisible)

IsKindVisible MUST return true only when the condition is met.

## [🧰semiorepo⌨️cli💻maingo🛠️matchessubkind](semiorepo://definition/semio-repo/cli/main.go/MatchesSubKind)

MatchesSubKind MUST operate on the TreeFilter receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️matchesdate](semiorepo://definition/semio-repo/cli/main.go/MatchesDate)

MatchesDate MUST operate on the TreeFilter receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️matchesstatus](semiorepo://definition/semio-repo/cli/main.go/MatchesStatus)

MatchesStatus MUST operate on the TreeFilter receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️matchescontributor](semiorepo://definition/semio-repo/cli/main.go/MatchesContributor)

MatchesContributor MUST operate on the TreeFilter receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️buildmonorepotree](semiorepo://definition/semio-repo/cli/main.go/BuildMonorepoTree)

BuildMonorepoTree MUST assemble the monorepo tree from the available context data.

## [🧰semiorepo⌨️cli💻maingo🛠️propagateparentids](semiorepo://definition/semio-repo/cli/main.go/PropagateParentIDs)

PropagateParentIDs MUST perform the PropagateParentIDs operation.

## [🧰semiorepo⌨️cli💻maingo🛠️filtermonorepotree](semiorepo://definition/semio-repo/cli/main.go/FilterMonorepoTree)

FilterMonorepoTree MUST preserve the tree structure while removing non-matching nodes.

## [🧰semiorepo⌨️cli💻maingo🛠️searchmonorepotree](semiorepo://definition/semio-repo/cli/main.go/SearchMonorepoTree)

SearchMonorepoTree MUST match case-insensitively against node labels and descriptions.

## [🧰semiorepo⌨️cli💻maingo🛠️rendermonorepotree](semiorepo://definition/semio-repo/cli/main.go/RenderMonorepoTree)

RenderMonorepoTree MUST produce a complete monorepo tree output.

## [🧰semiorepo⌨️cli💻maingo🛠️rendermonorepotreemarkdown](semiorepo://definition/semio-repo/cli/main.go/RenderMonorepoTreeMarkdown)

RenderMonorepoTreeMarkdown MUST produce a complete monorepo tree markdown output.

## [🧰semiorepo⌨️cli💻maingo🛠️render](semiorepo://definition/semio-repo/cli/main.go/Render)

Render MUST produce a complete  output.

## [🧰semiorepo⌨️cli💻maingo🛠️render](semiorepo://definition/semio-repo/cli/main.go/Render)

Render MUST produce a complete  output.

## [🧰semiorepo⌨️cli💻maingo🛠️render](semiorepo://definition/semio-repo/cli/main.go/Render)

Render MUST produce a complete  output.

## [🧰semiorepo⌨️cli💻maingo🛠️mermaidescapelabel](semiorepo://definition/semio-repo/cli/main.go/mermaidEscapeLabel)

mermaidEscapeLabel MUST escape double quotes in mermaid labels.

## [🧰semiorepo⌨️cli💻maingo🛠️mermaidprojectemoji](semiorepo://definition/semio-repo/cli/main.go/mermaidProjectEmoji)

mermaidProjectEmoji MUST return the correct emoji for the project kind.

## [🧰semiorepo⌨️cli💻maingo🛠️mermaidbundleemoji](semiorepo://definition/semio-repo/cli/main.go/mermaidBundleEmoji)

mermaidBundleEmoji MUST return the correct emoji for the bundle kind.

## [🧰semiorepo⌨️cli💻maingo🛠️mermaidfileemoji](semiorepo://definition/semio-repo/cli/main.go/mermaidFileEmoji)

mermaidFileEmoji MUST return the correct emoji for the file kind.

## [🧰semiorepo⌨️cli💻maingo🛠️mermaidlocbyprojectsbundlesfoldersfiles](semiorepo://definition/semio-repo/cli/main.go/MermaidLocByProjectsBundlesFoldersFiles)

MermaidLocByProjectsBundlesFoldersFiles MUST generate a treemap-beta mermaid diagram of LOC grouped by project, bundle, folder, and file.

## [🧰semiorepo⌨️cli💻maingo🛠️mermaidlocbycontributors](semiorepo://definition/semio-repo/cli/main.go/MermaidLocByContributors)

MermaidLocByContributors MUST generate a treemap-beta mermaid diagram of LOC grouped by contributor.

## [🧰semiorepo⌨️cli💻maingo🛠️mermaidlocbylanguage](semiorepo://definition/semio-repo/cli/main.go/MermaidLocByLanguage)

MermaidLocByLanguage MUST generate a treemap-beta mermaid diagram of LOC grouped by programming language.

## [🧰semiorepo⌨️cli💻maingo🛠️mermaidcommand](semiorepo://definition/semio-repo/cli/main.go/mermaidCommand)

mermaidCommand MUST return a cobra.Command with loc-by subcommands for mermaid diagram generation.

## [🧰semiorepo⌨️cli💻maingo🛠️isvalid](semiorepo://definition/semio-repo/cli/main.go/IsValid)

IsValid MUST return true only when the condition is met.

## [🧰semiorepo⌨️cli💻maingo🛠️string](semiorepo://definition/semio-repo/cli/main.go/String)

String MUST return the canonical string value.

## [🧰semiorepo⌨️cli💻maingo🛠️derivedefinitionkind](semiorepo://definition/semio-repo/cli/main.go/DeriveDefinitionKind)

DeriveDefinitionKind MUST return a valid value for any recognized input.

## [🧰semiorepo⌨️cli💻maingo🛠️isvalid](semiorepo://definition/semio-repo/cli/main.go/IsValid)

IsValid MUST return true only when the condition is met.

## [🧰semiorepo⌨️cli💻maingo🛠️string](semiorepo://definition/semio-repo/cli/main.go/String)

String MUST return the canonical string value.

## [🧰semiorepo⌨️cli💻maingo🛠️isvalid](semiorepo://definition/semio-repo/cli/main.go/IsValid)

IsValid MUST return true only when the condition is met.

## [🧰semiorepo⌨️cli💻maingo🛠️string](semiorepo://definition/semio-repo/cli/main.go/String)

String MUST return the canonical string value.

## [🧰semiorepo⌨️cli💻maingo🛠️normalizellmslug](semiorepo://definition/semio-repo/cli/main.go/NormalizeLLMSlug)

NormalizeLLMSlug MUST be idempotent for already-normalized values.

## [🧰semiorepo⌨️cli💻maingo🛠️normalizeclientslug](semiorepo://definition/semio-repo/cli/main.go/NormalizeClientSlug)

NormalizeClientSlug MUST be idempotent for already-normalized values.

## [🧰semiorepo⌨️cli💻maingo🛠️resolveallowedllm](semiorepo://definition/semio-repo/cli/main.go/ResolveAllowedLLM)

ResolveAllowedLLM MUST return an error for unrecognized values.

## [🧰semiorepo⌨️cli💻maingo🛠️resolveallowedclient](semiorepo://definition/semio-repo/cli/main.go/ResolveAllowedClient)

ResolveAllowedClient MUST return an error for unrecognized values.

## [🧰semiorepo⌨️cli💻maingo🛠️isnode](semiorepo://definition/semio-repo/cli/main.go/IsNode)

IsNode MUST return true only when the condition is met.

## [🧰semiorepo⌨️cli💻maingo🛠️getid](semiorepo://definition/semio-repo/cli/main.go/GetID)

GetID MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️geturi](semiorepo://definition/semio-repo/cli/main.go/GetURI)

GetURI MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️string](semiorepo://definition/semio-repo/cli/main.go/String)

String MUST return the canonical string value.

## [🧰semiorepo⌨️cli💻maingo🛠️isvalid](semiorepo://definition/semio-repo/cli/main.go/IsValid)

IsValid MUST return true only when the condition is met.

## [🧰semiorepo⌨️cli💻maingo🛠️string](semiorepo://definition/semio-repo/cli/main.go/String)

String MUST return the canonical string value.

## [🧰semiorepo⌨️cli💻maingo🛠️deriveprojectkind](semiorepo://definition/semio-repo/cli/main.go/DeriveProjectKind)

DeriveProjectKind MUST return a valid value for any recognized input.

## [🧰semiorepo⌨️cli💻maingo🛠️derivebundlekind](semiorepo://definition/semio-repo/cli/main.go/DeriveBundleKind)

DeriveBundleKind MUST return a valid value for any recognized input.

## [🧰semiorepo⌨️cli💻maingo🛠️isnode](semiorepo://definition/semio-repo/cli/main.go/IsNode)

IsNode MUST return true only when the condition is met.

## [🧰semiorepo⌨️cli💻maingo🛠️getid](semiorepo://definition/semio-repo/cli/main.go/GetID)

GetID MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️geturi](semiorepo://definition/semio-repo/cli/main.go/GetURI)

GetURI MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️isnode](semiorepo://definition/semio-repo/cli/main.go/IsNode)

IsNode MUST return true only when the condition is met.

## [🧰semiorepo⌨️cli💻maingo🛠️getid](semiorepo://definition/semio-repo/cli/main.go/GetID)

GetID MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️geturi](semiorepo://definition/semio-repo/cli/main.go/GetURI)

GetURI MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️isvalid](semiorepo://definition/semio-repo/cli/main.go/IsValid)

IsValid MUST return true only when the condition is met.

## [🧰semiorepo⌨️cli💻maingo🛠️string](semiorepo://definition/semio-repo/cli/main.go/String)

String MUST return the canonical string value.

## [🧰semiorepo⌨️cli💻maingo🛠️derivefolderkind](semiorepo://definition/semio-repo/cli/main.go/DeriveFolderKind)

DeriveFolderKind MUST return a valid value for any recognized input.

## [🧰semiorepo⌨️cli💻maingo🛠️isgeneratedfolder](semiorepo://definition/semio-repo/cli/main.go/IsGeneratedFolder)

IsGeneratedFolder MUST return true only when the condition is met.

## [🧰semiorepo⌨️cli💻maingo🛠️isnode](semiorepo://definition/semio-repo/cli/main.go/IsNode)

IsNode MUST return true only when the condition is met.

## [🧰semiorepo⌨️cli💻maingo🛠️getid](semiorepo://definition/semio-repo/cli/main.go/GetID)

GetID MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️geturi](semiorepo://definition/semio-repo/cli/main.go/GetURI)

GetURI MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️derivefilekind](semiorepo://definition/semio-repo/cli/main.go/DeriveFileKind)

DeriveFileKind MUST return a valid value for any recognized input.

## [🧰semiorepo⌨️cli💻maingo🛠️isnode](semiorepo://definition/semio-repo/cli/main.go/IsNode)

IsNode MUST return true only when the condition is met.

## [🧰semiorepo⌨️cli💻maingo🛠️isgenerated](semiorepo://definition/semio-repo/cli/main.go/IsGenerated)

IsGenerated MUST return true only when the condition is met.

## [🧰semiorepo⌨️cli💻maingo🛠️issemanticallyignored](semiorepo://definition/semio-repo/cli/main.go/IsSemanticallyIgnored)

IsSemanticallyIgnored MUST return true only when the condition is met.

## [🧰semiorepo⌨️cli💻maingo🛠️getid](semiorepo://definition/semio-repo/cli/main.go/GetID)

GetID MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️geturi](semiorepo://definition/semio-repo/cli/main.go/GetURI)

GetURI MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️isnode](semiorepo://definition/semio-repo/cli/main.go/IsNode)

IsNode MUST return true only when the condition is met.

## [🧰semiorepo⌨️cli💻maingo🛠️getid](semiorepo://definition/semio-repo/cli/main.go/GetID)

GetID MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️geturi](semiorepo://definition/semio-repo/cli/main.go/GetURI)

GetURI MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️isnode](semiorepo://definition/semio-repo/cli/main.go/IsNode)

IsNode MUST return true only when the condition is met.

## [🧰semiorepo⌨️cli💻maingo🛠️getid](semiorepo://definition/semio-repo/cli/main.go/GetID)

GetID MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️geturi](semiorepo://definition/semio-repo/cli/main.go/GetURI)

GetURI MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️isnode](semiorepo://definition/semio-repo/cli/main.go/IsNode)

IsNode MUST return true only when the condition is met.

## [🧰semiorepo⌨️cli💻maingo🛠️getid](semiorepo://definition/semio-repo/cli/main.go/GetID)

GetID MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️geturi](semiorepo://definition/semio-repo/cli/main.go/GetURI)

GetURI MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️isnode](semiorepo://definition/semio-repo/cli/main.go/IsNode)

IsNode MUST return true only when the condition is met.

## [🧰semiorepo⌨️cli💻maingo🛠️getid](semiorepo://definition/semio-repo/cli/main.go/GetID)

GetID MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️geturi](semiorepo://definition/semio-repo/cli/main.go/GetURI)

GetURI MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️getdraftspath](semiorepo://definition/semio-repo/cli/main.go/GetDraftsPath)

GetDraftsPath MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️listdrafts](semiorepo://definition/semio-repo/cli/main.go/ListDrafts)

ListDrafts MUST return a consistent snapshot of available entries.

## [🧰semiorepo⌨️cli💻maingo🛠️createdraft](semiorepo://definition/semio-repo/cli/main.go/CreateDraft)

CreateDraft MUST persist the new entity and return a reference to it.

## [🧰semiorepo⌨️cli💻maingo🛠️deletedraft](semiorepo://definition/semio-repo/cli/main.go/DeleteDraft)

DeleteDraft MUST remove all associated data for the entity.

## [🧰semiorepo⌨️cli💻maingo🛠️getid](semiorepo://definition/semio-repo/cli/main.go/GetID)

GetID MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️geturi](semiorepo://definition/semio-repo/cli/main.go/GetURI)

GetURI MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️isnode](semiorepo://definition/semio-repo/cli/main.go/IsNode)

IsNode MUST return true only when the condition is met.

## [🧰semiorepo⌨️cli💻maingo🛠️getid](semiorepo://definition/semio-repo/cli/main.go/GetID)

GetID MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️geturi](semiorepo://definition/semio-repo/cli/main.go/GetURI)

GetURI MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️gettitle](semiorepo://definition/semio-repo/cli/main.go/GetTitle)

GetTitle MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️getprompt](semiorepo://definition/semio-repo/cli/main.go/GetPrompt)

GetPrompt MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️getlatestprompt](semiorepo://definition/semio-repo/cli/main.go/GetLatestPrompt)

GetLatestPrompt MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️getllm](semiorepo://definition/semio-repo/cli/main.go/GetLLM)

GetLLM MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️getclient](semiorepo://definition/semio-repo/cli/main.go/GetClient)

GetClient MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️getstatus](semiorepo://definition/semio-repo/cli/main.go/GetStatus)

GetStatus MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️getauthor](semiorepo://definition/semio-repo/cli/main.go/GetAuthor)

GetAuthor MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️getcommit](semiorepo://definition/semio-repo/cli/main.go/GetCommit)

GetCommit MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️getsummary](semiorepo://definition/semio-repo/cli/main.go/GetSummary)

GetSummary MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️getdatestarted](semiorepo://definition/semio-repo/cli/main.go/GetDateStarted)

GetDateStarted MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️getdatefinished](semiorepo://definition/semio-repo/cli/main.go/GetDateFinished)

GetDateFinished MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️getinteractionfiles](semiorepo://definition/semio-repo/cli/main.go/GetInteractionFiles)

GetInteractionFiles MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️isnode](semiorepo://definition/semio-repo/cli/main.go/IsNode)

IsNode MUST return true only when the condition is met.

## [🧰semiorepo⌨️cli💻maingo🛠️getid](semiorepo://definition/semio-repo/cli/main.go/GetID)

GetID MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️geturi](semiorepo://definition/semio-repo/cli/main.go/GetURI)

GetURI MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️isnode](semiorepo://definition/semio-repo/cli/main.go/IsNode)

IsNode MUST return true only when the condition is met.

## [🧰semiorepo⌨️cli💻maingo🛠️getid](semiorepo://definition/semio-repo/cli/main.go/GetID)

GetID MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️geturi](semiorepo://definition/semio-repo/cli/main.go/GetURI)

GetURI MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️buildsemanticdiffs](semiorepo://definition/semio-repo/cli/main.go/BuildSemanticDiffs)

BuildSemanticDiffs MUST assemble the semantic diffs from the available context data.

## [🧰semiorepo⌨️cli💻maingo🛠️tostreamoptions](semiorepo://definition/semio-repo/cli/main.go/ToStreamOptions)

ToStreamOptions MUST map all filter input fields to stream options.

## [🧰semiorepo⌨️cli💻maingo🛠️isnode](semiorepo://definition/semio-repo/cli/main.go/IsNode)

IsNode MUST return true only when the condition is met.

## [🧰semiorepo⌨️cli💻maingo🛠️getid](semiorepo://definition/semio-repo/cli/main.go/GetID)

GetID MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️geturi](semiorepo://definition/semio-repo/cli/main.go/GetURI)

GetURI MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️isnode](semiorepo://definition/semio-repo/cli/main.go/IsNode)

IsNode MUST return true only when the condition is met.

## [🧰semiorepo⌨️cli💻maingo🛠️getid](semiorepo://definition/semio-repo/cli/main.go/GetID)

GetID MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️geturi](semiorepo://definition/semio-repo/cli/main.go/GetURI)

GetURI MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️priority](semiorepo://definition/semio-repo/cli/main.go/Priority)

Priority MUST derive the value from the statute metadata.

## [🧰semiorepo⌨️cli💻maingo🛠️autofixable](semiorepo://definition/semio-repo/cli/main.go/Autofixable)

Autofixable MUST return true only for statutes that support auto-fix.

## [🧰semiorepo⌨️cli💻maingo🛠️name](semiorepo://definition/semio-repo/cli/main.go/Name)

Name MUST operate on the BaseLanguage receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️extensions](semiorepo://definition/semio-repo/cli/main.go/Extensions)

Extensions MUST operate on the BaseLanguage receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️commentprefix](semiorepo://definition/semio-repo/cli/main.go/CommentPrefix)

CommentPrefix MUST operate on the BaseLanguage receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️blockcommentstart](semiorepo://definition/semio-repo/cli/main.go/BlockCommentStart)

BlockCommentStart MUST operate on the BaseLanguage receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️blockcommentend](semiorepo://definition/semio-repo/cli/main.go/BlockCommentEnd)

BlockCommentEnd MUST operate on the BaseLanguage receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️usesindentscoping](semiorepo://definition/semio-repo/cli/main.go/UsesIndentScoping)

UsesIndentScoping MUST operate on the BaseLanguage receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️supportssections](semiorepo://definition/semio-repo/cli/main.go/SupportsSections)

SupportsSections MUST operate on the BaseLanguage receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️supportsdefinitions](semiorepo://definition/semio-repo/cli/main.go/SupportsDefinitions)

SupportsDefinitions MUST operate on the BaseLanguage receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️supportscomments](semiorepo://definition/semio-repo/cli/main.go/SupportsComments)

SupportsComments MUST operate on the BaseLanguage receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️supportsheaders](semiorepo://definition/semio-repo/cli/main.go/SupportsHeaders)

SupportsHeaders MUST operate on the BaseLanguage receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️matchesextension](semiorepo://definition/semio-repo/cli/main.go/MatchesExtension)

MatchesExtension MUST operate on the BaseLanguage receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️formatsectionstart](semiorepo://definition/semio-repo/cli/main.go/FormatSectionStart)

FormatSectionStart MUST produce a well-formed section start string.

## [🧰semiorepo⌨️cli💻maingo🛠️formatsectionend](semiorepo://definition/semio-repo/cli/main.go/FormatSectionEnd)

FormatSectionEnd MUST produce a well-formed section end string.

## [🧰semiorepo⌨️cli💻maingo🛠️formatsectionboth](semiorepo://definition/semio-repo/cli/main.go/FormatSectionBoth)

FormatSectionBoth MUST produce a well-formed section both string.

## [🧰semiorepo⌨️cli💻maingo🛠️formatheader](semiorepo://definition/semio-repo/cli/main.go/FormatHeader)

FormatHeader MUST produce a well-formed header string.

## [🧰semiorepo⌨️cli💻maingo🛠️policysectionstartmatch](semiorepo://definition/semio-repo/cli/main.go/PolicySectionStartMatch)

PolicySectionStartMatch MUST operate on the BaseLanguage receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️policysectionendmatch](semiorepo://definition/semio-repo/cli/main.go/PolicySectionEndMatch)

PolicySectionEndMatch MUST operate on the BaseLanguage receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️parsesections](semiorepo://definition/semio-repo/cli/main.go/ParseSections)

ParseSections MUST return an error when the input is malformed.

## [🧰semiorepo⌨️cli💻maingo🛠️parsedefinitions](semiorepo://definition/semio-repo/cli/main.go/ParseDefinitions)

ParseDefinitions MUST return an error when the input is malformed.

## [🧰semiorepo⌨️cli💻maingo🛠️extraorphandefinitions](semiorepo://definition/semio-repo/cli/main.go/ExtraOrphanDefinitions)

ExtraOrphanDefinitions MUST operate on the BaseLanguage receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️skipdirectives](semiorepo://definition/semio-repo/cli/main.go/SkipDirectives)

SkipDirectives MUST operate on the BaseLanguage receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️scancomments](semiorepo://definition/semio-repo/cli/main.go/ScanComments)

ScanComments MUST operate on the BaseLanguage receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️extractimports](semiorepo://definition/semio-repo/cli/main.go/ExtractImports)

ExtractImports MUST return the extracted value without side effects.

## [🧰semiorepo⌨️cli💻maingo🛠️formatimports](semiorepo://definition/semio-repo/cli/main.go/FormatImports)

FormatImports MUST produce a well-formed imports string.

## [🧰semiorepo⌨️cli💻maingo🛠️extractpackage](semiorepo://definition/semio-repo/cli/main.go/ExtractPackage)

ExtractPackage MUST return the extracted value without side effects.

## [🧰semiorepo⌨️cli💻maingo🛠️newtypescriptlanguage](semiorepo://definition/semio-repo/cli/main.go/NewTypeScriptLanguage)

NewTypeScriptLanguage MUST initialize all required fields and return a valid TypeScriptLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️scancomments](semiorepo://definition/semio-repo/cli/main.go/ScanComments)

ScanComments MUST operate on the TypeScriptLanguage receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️extractimports](semiorepo://definition/semio-repo/cli/main.go/ExtractImports)

ExtractImports MUST return the extracted value without side effects.

## [🧰semiorepo⌨️cli💻maingo🛠️formatimports](semiorepo://definition/semio-repo/cli/main.go/FormatImports)

FormatImports MUST produce a well-formed imports string.

## [🧰semiorepo⌨️cli💻maingo🛠️newgolanguage](semiorepo://definition/semio-repo/cli/main.go/NewGoLanguage)

NewGoLanguage MUST initialize all required fields and return a valid GoLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️extraorphandefinitions](semiorepo://definition/semio-repo/cli/main.go/ExtraOrphanDefinitions)

ExtraOrphanDefinitions MUST operate on the GoLanguage receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️extractimports](semiorepo://definition/semio-repo/cli/main.go/ExtractImports)

ExtractImports MUST return the extracted value without side effects.

## [🧰semiorepo⌨️cli💻maingo🛠️formatimports](semiorepo://definition/semio-repo/cli/main.go/FormatImports)

FormatImports MUST produce a well-formed imports string.

## [🧰semiorepo⌨️cli💻maingo🛠️extractpackage](semiorepo://definition/semio-repo/cli/main.go/ExtractPackage)

ExtractPackage MUST return the extracted value without side effects.

## [🧰semiorepo⌨️cli💻maingo🛠️newpythonlanguage](semiorepo://definition/semio-repo/cli/main.go/NewPythonLanguage)

NewPythonLanguage MUST initialize all required fields and return a valid PythonLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️extractimports](semiorepo://definition/semio-repo/cli/main.go/ExtractImports)

ExtractImports MUST return the extracted value without side effects.

## [🧰semiorepo⌨️cli💻maingo🛠️formatimports](semiorepo://definition/semio-repo/cli/main.go/FormatImports)

FormatImports MUST produce a well-formed imports string.

## [🧰semiorepo⌨️cli💻maingo🛠️newcsharplanguage](semiorepo://definition/semio-repo/cli/main.go/NewCSharpLanguage)

NewCSharpLanguage MUST initialize all required fields and return a valid CSharpLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️extractimports](semiorepo://definition/semio-repo/cli/main.go/ExtractImports)

ExtractImports MUST return the extracted value without side effects.

## [🧰semiorepo⌨️cli💻maingo🛠️formatimports](semiorepo://definition/semio-repo/cli/main.go/FormatImports)

FormatImports MUST produce a well-formed imports string.

## [🧰semiorepo⌨️cli💻maingo🛠️newjsonlanguage](semiorepo://definition/semio-repo/cli/main.go/NewJSONLanguage)

NewJSONLanguage MUST initialize all required fields and return a valid JSONLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️supportssections](semiorepo://definition/semio-repo/cli/main.go/SupportsSections)

SupportsSections MUST operate on the JSONLanguage receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️supportsdefinitions](semiorepo://definition/semio-repo/cli/main.go/SupportsDefinitions)

SupportsDefinitions MUST operate on the JSONLanguage receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️supportscomments](semiorepo://definition/semio-repo/cli/main.go/SupportsComments)

SupportsComments MUST operate on the JSONLanguage receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️supportsheaders](semiorepo://definition/semio-repo/cli/main.go/SupportsHeaders)

SupportsHeaders MUST operate on the JSONLanguage receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️parsesections](semiorepo://definition/semio-repo/cli/main.go/ParseSections)

ParseSections MUST return an error when the input is malformed.

## [🧰semiorepo⌨️cli💻maingo🛠️newmarkdownlanguage](semiorepo://definition/semio-repo/cli/main.go/NewMarkdownLanguage)

NewMarkdownLanguage MUST initialize all required fields and return a valid MarkdownLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️supportssections](semiorepo://definition/semio-repo/cli/main.go/SupportsSections)

SupportsSections MUST operate on the MarkdownLanguage receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️supportsdefinitions](semiorepo://definition/semio-repo/cli/main.go/SupportsDefinitions)

SupportsDefinitions MUST operate on the MarkdownLanguage receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️supportscomments](semiorepo://definition/semio-repo/cli/main.go/SupportsComments)

SupportsComments MUST operate on the MarkdownLanguage receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️parsesections](semiorepo://definition/semio-repo/cli/main.go/ParseSections)

ParseSections MUST return an error when the input is malformed.

## [🧰semiorepo⌨️cli💻maingo🛠️newrustlanguage](semiorepo://definition/semio-repo/cli/main.go/NewRustLanguage)

NewRustLanguage MUST initialize all required fields and return a valid RustLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️extraorphandefinitions](semiorepo://definition/semio-repo/cli/main.go/ExtraOrphanDefinitions)

ExtraOrphanDefinitions MUST operate on the RustLanguage receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️newrubylanguage](semiorepo://definition/semio-repo/cli/main.go/NewRubyLanguage)

NewRubyLanguage MUST initialize all required fields and return a valid RubyLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️parsedefinitions](semiorepo://definition/semio-repo/cli/main.go/ParseDefinitions)

ParseDefinitions MUST return an error when the input is malformed.

## [🧰semiorepo⌨️cli💻maingo🛠️extraorphandefinitions](semiorepo://definition/semio-repo/cli/main.go/ExtraOrphanDefinitions)

ExtraOrphanDefinitions MUST operate on the RubyLanguage receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️newshelllanguage](semiorepo://definition/semio-repo/cli/main.go/NewShellLanguage)

NewShellLanguage MUST initialize all required fields and return a valid ShellLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️newtomllanguage](semiorepo://definition/semio-repo/cli/main.go/NewTomlLanguage)

NewTomlLanguage MUST initialize all required fields and return a valid TomlLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️supportssections](semiorepo://definition/semio-repo/cli/main.go/SupportsSections)

SupportsSections MUST operate on the TomlLanguage receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️supportsdefinitions](semiorepo://definition/semio-repo/cli/main.go/SupportsDefinitions)

SupportsDefinitions MUST operate on the TomlLanguage receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️supportscomments](semiorepo://definition/semio-repo/cli/main.go/SupportsComments)

SupportsComments MUST operate on the TomlLanguage receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️supportsheaders](semiorepo://definition/semio-repo/cli/main.go/SupportsHeaders)

SupportsHeaders MUST operate on the TomlLanguage receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️newyamllanguage](semiorepo://definition/semio-repo/cli/main.go/NewYamlLanguage)

NewYamlLanguage MUST initialize all required fields and return a valid YamlLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️supportssections](semiorepo://definition/semio-repo/cli/main.go/SupportsSections)

SupportsSections MUST operate on the YamlLanguage receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️supportsdefinitions](semiorepo://definition/semio-repo/cli/main.go/SupportsDefinitions)

SupportsDefinitions MUST operate on the YamlLanguage receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️supportscomments](semiorepo://definition/semio-repo/cli/main.go/SupportsComments)

SupportsComments MUST operate on the YamlLanguage receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️supportsheaders](semiorepo://definition/semio-repo/cli/main.go/SupportsHeaders)

SupportsHeaders MUST operate on the YamlLanguage receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️newsqllanguage](semiorepo://definition/semio-repo/cli/main.go/NewSqlLanguage)

NewSqlLanguage MUST initialize all required fields and return a valid SqlLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️newgraphqllanguage](semiorepo://definition/semio-repo/cli/main.go/NewGraphqlLanguage)

NewGraphqlLanguage MUST initialize all required fields and return a valid GraphqlLanguage.

## [🧰semiorepo⌨️cli💻maingo🛠️getlanguage](semiorepo://definition/semio-repo/cli/main.go/GetLanguage)

GetLanguage MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️getlanguagebyname](semiorepo://definition/semio-repo/cli/main.go/GetLanguageByName)

GetLanguageByName MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️string](semiorepo://definition/semio-repo/cli/main.go/String)

String MUST return the canonical string value.

## [🧰semiorepo⌨️cli💻maingo🛠️findandupdatecontributor](semiorepo://definition/semio-repo/cli/main.go/FindAndUpdateContributor)

FindAndUpdateContributor MUST return nil when no match is found.

## [🧰semiorepo⌨️cli💻maingo🛠️getsystem](semiorepo://definition/semio-repo/cli/main.go/GetSystem)

GetSystem MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️unmarshaljson](semiorepo://definition/semio-repo/cli/main.go/UnmarshalJSON)

UnmarshalJSON MUST handle both legacy and current JSON layouts.

## [🧰semiorepo⌨️cli💻maingo🛠️listinteractions](semiorepo://definition/semio-repo/cli/main.go/ListInteractions)

ListInteractions MUST perform the ListInteractions operation.

## [🧰semiorepo⌨️cli💻maingo🛠️streaminteractions](semiorepo://definition/semio-repo/cli/main.go/StreamInteractions)

StreamInteractions MUST perform the StreamInteractions operation.

## [🧰semiorepo⌨️cli💻maingo🛠️isnode](semiorepo://definition/semio-repo/cli/main.go/IsNode)

IsNode MUST return true only when the condition is met.

## [🧰semiorepo⌨️cli💻maingo🛠️getid](semiorepo://definition/semio-repo/cli/main.go/GetID)

GetID MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️geturi](semiorepo://definition/semio-repo/cli/main.go/GetURI)

GetURI MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️info](semiorepo://definition/semio-repo/cli/main.go/Info)

Info MUST return the metadata entry for the statute.

## [🧰semiorepo⌨️cli💻maingo🛠️allkinds](semiorepo://definition/semio-repo/cli/main.go/AllKinds)

AllKinds MUST include all statutes from the group and its children.

## [🧰semiorepo⌨️cli💻maingo🛠️getid](semiorepo://definition/semio-repo/cli/main.go/GetID)

GetID MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️geturi](semiorepo://definition/semio-repo/cli/main.go/GetURI)

GetURI MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️allkinds](semiorepo://definition/semio-repo/cli/main.go/AllKinds)

AllKinds MUST include all statutes from the group and its children.

## [🧰semiorepo⌨️cli💻maingo🛠️getrootdir](semiorepo://definition/semio-repo/cli/main.go/GetRootDir)

GetRootDir MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️setrootdir](semiorepo://definition/semio-repo/cli/main.go/SetRootDir)

SetRootDir MUST update the value on the receiver.

## [🧰semiorepo⌨️cli💻maingo🛠️getrepometadir](semiorepo://definition/semio-repo/cli/main.go/GetRepoMetaDir)

GetRepoMetaDir MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️getrepometapath](semiorepo://definition/semio-repo/cli/main.go/GetRepoMetaPath)

GetRepoMetaPath MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️normalizepath](semiorepo://definition/semio-repo/cli/main.go/NormalizePath)

NormalizePath MUST be idempotent for already-normalized values.

## [🧰semiorepo⌨️cli💻maingo🛠️ensuredir](semiorepo://definition/semio-repo/cli/main.go/EnsureDir)

EnsureDir MUST be idempotent and MUST NOT fail if the target already exists.

## [🧰semiorepo⌨️cli💻maingo🛠️getrelativepath](semiorepo://definition/semio-repo/cli/main.go/GetRelativePath)

GetRelativePath MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️readtextfile](semiorepo://definition/semio-repo/cli/main.go/ReadTextFile)

ReadTextFile MUST return the full content from the given path.

## [🧰semiorepo⌨️cli💻maingo🛠️writetextfile](semiorepo://definition/semio-repo/cli/main.go/WriteTextFile)

WriteTextFile MUST persist the content atomically.

## [🧰semiorepo⌨️cli💻maingo🛠️writejsonfile](semiorepo://definition/semio-repo/cli/main.go/WriteJSONFile)

WriteJSONFile MUST persist the content atomically.

## [🧰semiorepo⌨️cli💻maingo🛠️readjsonfile](semiorepo://definition/semio-repo/cli/main.go/ReadJSONFile)

ReadJSONFile MUST return the full content from the given path.

## [🧰semiorepo⌨️cli💻maingo🛠️fileexists](semiorepo://definition/semio-repo/cli/main.go/FileExists)

FileExists MUST complete the operation and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️isdir](semiorepo://definition/semio-repo/cli/main.go/IsDir)

IsDir MUST return true only when the condition is met.

## [🧰semiorepo⌨️cli💻maingo🛠️loadgitignore](semiorepo://definition/semio-repo/cli/main.go/LoadGitignore)

LoadGitignore MUST read from the configured storage path.

## [🧰semiorepo⌨️cli💻maingo🛠️simpleglob](semiorepo://definition/semio-repo/cli/main.go/SimpleGlob)

SimpleGlob MUST complete the operation and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️isotimestamp](semiorepo://definition/semio-repo/cli/main.go/ISOTimestamp)

ISOTimestamp MUST complete the operation and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️formatdate](semiorepo://definition/semio-repo/cli/main.go/FormatDate)

FormatDate MUST produce a well-formed date string.

## [🧰semiorepo⌨️cli💻maingo🛠️padnumber](semiorepo://definition/semio-repo/cli/main.go/PadNumber)

PadNumber MUST complete the operation and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️pathtouripath](semiorepo://definition/semio-repo/cli/main.go/PathToUriPath)

PathToUriPath MUST complete the operation and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️pathfromuripath](semiorepo://definition/semio-repo/cli/main.go/PathFromUriPath)

PathFromUriPath MUST complete the operation and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️flat](semiorepo://definition/semio-repo/cli/main.go/Flat)

Flat MUST preserve only alphanumeric characters and emojis, then lower case.

## [🧰semiorepo⌨️cli💻maingo🛠️slugify](semiorepo://definition/semio-repo/cli/main.go/Slugify)

Slugify MUST complete the operation and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️titleizeslug](semiorepo://definition/semio-repo/cli/main.go/TitleizeSlug)

TitleizeSlug MUST complete the operation and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️statutepathtoidvalue](semiorepo://definition/semio-repo/cli/main.go/StatutePathToIdValue)

StatutePathToIdValue MUST complete the operation and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️statuteidvaluetopath](semiorepo://definition/semio-repo/cli/main.go/StatuteIdValueToPath)

StatuteIdValueToPath MUST complete the operation and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️execcommand](semiorepo://definition/semio-repo/cli/main.go/ExecCommand)

ExecCommand MUST complete the operation and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️getgitauthor](semiorepo://definition/semio-repo/cli/main.go/GetGitAuthor)

GetGitAuthor MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️getgitauthorgithub](semiorepo://definition/semio-repo/cli/main.go/GetGitAuthorGithub)

GetGitAuthorGithub MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️getgitcommit](semiorepo://definition/semio-repo/cli/main.go/GetGitCommit)

GetGitCommit MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️getgitignoredset](semiorepo://definition/semio-repo/cli/main.go/GetGitIgnoredSet)

GetGitIgnoredSet MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️newoutput](semiorepo://definition/semio-repo/cli/main.go/NewOutput)

NewOutput MUST initialize all required fields and return a valid Output.

## [🧰semiorepo⌨️cli💻maingo🛠️info](semiorepo://definition/semio-repo/cli/main.go/Info)

Info MUST return the metadata entry for the statute.

## [🧰semiorepo⌨️cli💻maingo🛠️success](semiorepo://definition/semio-repo/cli/main.go/Success)

Success MUST operate on the CommandOutput receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️error](semiorepo://definition/semio-repo/cli/main.go/Error)

Error MUST return a formatted string representation.

## [🧰semiorepo⌨️cli💻maingo🛠️warn](semiorepo://definition/semio-repo/cli/main.go/Warn)

Warn MUST operate on the CommandOutput receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️plain](semiorepo://definition/semio-repo/cli/main.go/Plain)

Plain MUST operate on the CommandOutput receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️print](semiorepo://definition/semio-repo/cli/main.go/Print)

Print MUST operate on the CommandOutput receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️json](semiorepo://definition/semio-repo/cli/main.go/Json)

Json MUST operate on the CommandOutput receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️listdirentries](semiorepo://definition/semio-repo/cli/main.go/ListDirEntries)

ListDirEntries MUST return a consistent snapshot of available entries.

## [🧰semiorepo⌨️cli💻maingo🛠️walkdir](semiorepo://definition/semio-repo/cli/main.go/WalkDir)

WalkDir MUST visit every entry and MUST stop when the callback returns an error.

## [🧰semiorepo⌨️cli💻maingo🛠️parsescope](semiorepo://definition/semio-repo/cli/main.go/ParseScope)

ParseScope MUST return an error when the input is malformed.

## [🧰semiorepo⌨️cli💻maingo🛠️readlines](semiorepo://definition/semio-repo/cli/main.go/ReadLines)

ReadLines MUST return the full content from the given path.

## [🧰semiorepo⌨️cli💻maingo🛠️parsecodesections](semiorepo://definition/semio-repo/cli/main.go/ParseCodeSections)

ParseCodeSections MUST return an error when the input is malformed.

## [🧰semiorepo⌨️cli💻maingo🛠️parsemarkdownsectionsinternal](semiorepo://definition/semio-repo/cli/main.go/ParseMarkdownSectionsInternal)

ParseMarkdownSectionsInternal MUST return an error when the input is malformed.

## [🧰semiorepo⌨️cli💻maingo🛠️parsejsonsectionsdetailed](semiorepo://definition/semio-repo/cli/main.go/ParseJSONSectionsDetailed)

ParseJSONSectionsDetailed MUST return an error when the input is malformed.

## [🧰semiorepo⌨️cli💻maingo🛠️parsejsonsections](semiorepo://definition/semio-repo/cli/main.go/ParseJSONSections)

ParseJSONSections MUST return an error when the input is malformed.

## [🧰semiorepo⌨️cli💻maingo🛠️parsesections](semiorepo://definition/semio-repo/cli/main.go/ParseSections)

ParseSections MUST return an error when the input is malformed.

## [🧰semiorepo⌨️cli💻maingo🛠️parsedefinitions](semiorepo://definition/semio-repo/cli/main.go/ParseDefinitions)

ParseDefinitions MUST return an error when the input is malformed.

## [🧰semiorepo⌨️cli💻maingo🛠️hydratesectionswithdefinitions](semiorepo://definition/semio-repo/cli/main.go/HydrateSectionsWithDefinitions)

HydrateSectionsWithDefinitions MUST attach all matching child elements to their parents.

## [🧰semiorepo⌨️cli💻maingo🛠️normalizesectionpath](semiorepo://definition/semio-repo/cli/main.go/NormalizeSectionPath)

NormalizeSectionPath MUST be idempotent for already-normalized values.

## [🧰semiorepo⌨️cli💻maingo🛠️findsection](semiorepo://definition/semio-repo/cli/main.go/FindSection)

FindSection MUST return nil when no match is found.

## [🧰semiorepo⌨️cli💻maingo🛠️findpolicy](semiorepo://definition/semio-repo/cli/main.go/FindPolicy)

FindPolicy MUST return nil when no match is found.

## [🧰semiorepo⌨️cli💻maingo🛠️getpolicies](semiorepo://definition/semio-repo/cli/main.go/GetPolicies)

GetPolicies MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️streampolicies](semiorepo://definition/semio-repo/cli/main.go/StreamPolicies)

StreamPolicies MUST emit all matching entries and close the channel when done.

## [🧰semiorepo⌨️cli💻maingo🛠️newpolicycontext](semiorepo://definition/semio-repo/cli/main.go/NewPolicyContext)

NewPolicyContext MUST initialize all required fields and return a valid PolicyContext.

## [🧰semiorepo⌨️cli💻maingo🛠️newpolicycontextwithfiles](semiorepo://definition/semio-repo/cli/main.go/NewPolicyContextWithFiles)

NewPolicyContextWithFiles MUST initialize all required fields and return a valid PolicyContextWithFiles.

## [🧰semiorepo⌨️cli💻maingo🛠️files](semiorepo://definition/semio-repo/cli/main.go/Files)

Files MUST operate on the PolicyContext receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️readtext](semiorepo://definition/semio-repo/cli/main.go/ReadText)

ReadText MUST return the full content from the given path.

## [🧰semiorepo⌨️cli💻maingo🛠️sections](semiorepo://definition/semio-repo/cli/main.go/Sections)

Sections MUST operate on the PolicyContext receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️parseignoredirectives](semiorepo://definition/semio-repo/cli/main.go/ParseIgnoreDirectives)

ParseIgnoreDirectives MUST return an error when the input is malformed.

## [🧰semiorepo⌨️cli💻maingo🛠️ignoredirectives](semiorepo://definition/semio-repo/cli/main.go/IgnoreDirectives)

IgnoreDirectives MUST operate on the PolicyContext receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️isignored](semiorepo://definition/semio-repo/cli/main.go/IsIgnored)

IsIgnored MUST return true only when the condition is met.

## [🧰semiorepo⌨️cli💻maingo🛠️createbreach](semiorepo://definition/semio-repo/cli/main.go/CreateBreach)

CreateBreach MUST persist the new entity and return a reference to it.

## [🧰semiorepo⌨️cli💻maingo🛠️filterignored](semiorepo://definition/semio-repo/cli/main.go/FilterIgnored)

FilterIgnored MUST preserve the tree structure while removing non-matching nodes.

## [🧰semiorepo⌨️cli💻maingo🛠️speclines](semiorepo://definition/semio-repo/cli/main.go/SpecLines)

SpecLines MUST operate on the PolicyContext receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️isspecline](semiorepo://definition/semio-repo/cli/main.go/IsSpecLine)

IsSpecLine MUST return true only when the condition is met.

## [🧰semiorepo⌨️cli💻maingo🛠️isspecblock](semiorepo://definition/semio-repo/cli/main.go/IsSpecBlock)

IsSpecBlock MUST return true only when the condition is met.

## [🧰semiorepo⌨️cli💻maingo🛠️sectiondoclines](semiorepo://definition/semio-repo/cli/main.go/SectionDocLines)

SectionDocLines MUST operate on the PolicyContext receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️issectiondocline](semiorepo://definition/semio-repo/cli/main.go/IsSectionDocLine)

IsSectionDocLine MUST return true only when the condition is met.

## [🧰semiorepo⌨️cli💻maingo🛠️definitiondoclines](semiorepo://definition/semio-repo/cli/main.go/DefinitionDocLines)

DefinitionDocLines MUST operate on the PolicyContext receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️isdefinitiondocline](semiorepo://definition/semio-repo/cli/main.go/IsDefinitionDocLine)

IsDefinitionDocLine MUST return true only when the condition is met.

## [🧰semiorepo⌨️cli💻maingo🛠️checkpolicies](semiorepo://definition/semio-repo/cli/main.go/CheckPolicies)

CheckPolicies MUST run all applicable policies and aggregate breachs.

## [🧰semiorepo⌨️cli💻maingo🛠️checkpolicieswithcontext](semiorepo://definition/semio-repo/cli/main.go/CheckPoliciesWithContext)

CheckPoliciesWithContext MUST run all applicable policies and aggregate breachs.

## [🧰semiorepo⌨️cli💻maingo🛠️intemplateraw](semiorepo://definition/semio-repo/cli/main.go/InTemplateRaw)

InTemplateRaw MUST operate on the CommentScanState receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️newcodebasecontext](semiorepo://definition/semio-repo/cli/main.go/NewCodebaseContext)

NewCodebaseContext MUST initialize all required fields and return a valid CodebaseContext.

## [🧰semiorepo⌨️cli💻maingo🛠️loadbundles](semiorepo://definition/semio-repo/cli/main.go/LoadBundles)

LoadBundles MUST read from the configured storage path.

## [🧰semiorepo⌨️cli💻maingo🛠️loadfiles](semiorepo://definition/semio-repo/cli/main.go/LoadFiles)

LoadFiles MUST read from the configured storage path.

## [🧰semiorepo⌨️cli💻maingo🛠️loadbreachs](semiorepo://definition/semio-repo/cli/main.go/LoadBreachs)

LoadBreachs MUST read from the configured storage path.

## [🧰semiorepo⌨️cli💻maingo🛠️loadtickets](semiorepo://definition/semio-repo/cli/main.go/LoadTickets)

LoadTickets MUST read from the configured storage path.

## [🧰semiorepo⌨️cli💻maingo🛠️loadpolicies](semiorepo://definition/semio-repo/cli/main.go/LoadPolicies)

LoadPolicies MUST read from the configured storage path.

## [🧰semiorepo⌨️cli💻maingo🛠️getbundleforfile](semiorepo://definition/semio-repo/cli/main.go/GetBundleForFile)

GetBundleForFile MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️getbundleinfo](semiorepo://definition/semio-repo/cli/main.go/GetBundleInfo)

GetBundleInfo MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️getfileid](semiorepo://definition/semio-repo/cli/main.go/GetFileID)

GetFileID MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️getfolderid](semiorepo://definition/semio-repo/cli/main.go/GetFolderID)

GetFolderID MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️fileuri](semiorepo://definition/semio-repo/cli/main.go/FileURI)

FileURI MUST operate on the CodebaseContext receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️folderuri](semiorepo://definition/semio-repo/cli/main.go/FolderURI)

FolderURI MUST operate on the CodebaseContext receiver and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️buildcodebasebundles](semiorepo://definition/semio-repo/cli/main.go/BuildCodebaseBundles)

BuildCodebaseBundles MUST assemble the codebase bundles from the available context data.

## [🧰semiorepo⌨️cli💻maingo🛠️buildcodebasefolders](semiorepo://definition/semio-repo/cli/main.go/BuildCodebaseFolders)

BuildCodebaseFolders MUST assemble the codebase folders from the available context data.

## [🧰semiorepo⌨️cli💻maingo🛠️buildcodebasefiles](semiorepo://definition/semio-repo/cli/main.go/BuildCodebaseFiles)

BuildCodebaseFiles MUST assemble the codebase files from the available context data.

## [🧰semiorepo⌨️cli💻maingo🛠️buildcodebasesections](semiorepo://definition/semio-repo/cli/main.go/BuildCodebaseSections)

BuildCodebaseSections MUST assemble the codebase sections from the available context data.

## [🧰semiorepo⌨️cli💻maingo🛠️buildcodebasedefinitions](semiorepo://definition/semio-repo/cli/main.go/BuildCodebaseDefinitions)

BuildCodebaseDefinitions MUST assemble the codebase definitions from the available context data.

## [🧰semiorepo⌨️cli💻maingo🛠️buildcodebasecontributors](semiorepo://definition/semio-repo/cli/main.go/BuildCodebaseContributors)

BuildCodebaseContributors MUST assemble the codebase contributors from the available context data.

## [🧰semiorepo⌨️cli💻maingo🛠️buildcodebasetickets](semiorepo://definition/semio-repo/cli/main.go/BuildCodebaseTickets)

BuildCodebaseTickets MUST assemble the codebase tickets from the available context data.

## [🧰semiorepo⌨️cli💻maingo🛠️buildcodebasepolicies](semiorepo://definition/semio-repo/cli/main.go/BuildCodebasePolicies)

BuildCodebasePolicies MUST assemble the codebase policies from the available context data.

## [🧰semiorepo⌨️cli💻maingo🛠️buildcodebasebreachs](semiorepo://definition/semio-repo/cli/main.go/BuildCodebaseBreachs)

BuildCodebaseBreachs MUST assemble the codebase breachs from the available context data.

## [🧰semiorepo⌨️cli💻maingo🛠️buildcodebasetree](semiorepo://definition/semio-repo/cli/main.go/BuildCodebaseTree)

BuildCodebaseTree MUST assemble the codebase tree from the available context data.

## [🧰semiorepo⌨️cli💻maingo🛠️buildcodebase](semiorepo://definition/semio-repo/cli/main.go/BuildCodebase)

BuildCodebase MUST assemble the codebase from the available context data.

## [🧰semiorepo⌨️cli💻maingo🛠️buildcodebasesnapshot](semiorepo://definition/semio-repo/cli/main.go/BuildCodebaseSnapshot)

BuildCodebaseSnapshot MUST assemble the codebase snapshot from the available context data.

## [🧰semiorepo⌨️cli💻maingo🛠️buildcodebasebundlesforfiles](semiorepo://definition/semio-repo/cli/main.go/BuildCodebaseBundlesForFiles)

BuildCodebaseBundlesForFiles MUST assemble the codebase bundles for files from the available context data.

## [🧰semiorepo⌨️cli💻maingo🛠️buildcodebasefoldersforfiles](semiorepo://definition/semio-repo/cli/main.go/BuildCodebaseFoldersForFiles)

BuildCodebaseFoldersForFiles MUST assemble the codebase folders for files from the available context data.

## [🧰semiorepo⌨️cli💻maingo🛠️buildcodebasefilesforfiles](semiorepo://definition/semio-repo/cli/main.go/BuildCodebaseFilesForFiles)

BuildCodebaseFilesForFiles MUST assemble the codebase files for files from the available context data.

## [🧰semiorepo⌨️cli💻maingo🛠️buildcodebasesectionsforfiles](semiorepo://definition/semio-repo/cli/main.go/BuildCodebaseSectionsForFiles)

BuildCodebaseSectionsForFiles MUST assemble the codebase sections for files from the available context data.

## [🧰semiorepo⌨️cli💻maingo🛠️buildcodebasedefinitionsforfiles](semiorepo://definition/semio-repo/cli/main.go/BuildCodebaseDefinitionsForFiles)

BuildCodebaseDefinitionsForFiles MUST assemble the codebase definitions for files from the available context data.

## [🧰semiorepo⌨️cli💻maingo🛠️toolcodebase](semiorepo://definition/semio-repo/cli/main.go/ToolCodebase)

ToolCodebase MUST complete the operation and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️getticketsdir](semiorepo://definition/semio-repo/cli/main.go/GetTicketsDir)

GetTicketsDir MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️getticketpath](semiorepo://definition/semio-repo/cli/main.go/GetTicketPath)

GetTicketPath MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️getticketfilepath](semiorepo://definition/semio-repo/cli/main.go/GetTicketFilePath)

GetTicketFilePath MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️getimportantfilepath](semiorepo://definition/semio-repo/cli/main.go/GetImportantFilePath)

GetImportantFilePath MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️getticketjsonpath](semiorepo://definition/semio-repo/cli/main.go/GetTicketJsonPath)

GetTicketJsonPath MUST return the stored value without modification.

## [🧰semiorepo⌨️cli💻maingo🛠️findticketbyslug](semiorepo://definition/semio-repo/cli/main.go/FindTicketBySlug)

FindTicketBySlug MUST return nil when no match is found.

## [🧰semiorepo⌨️cli💻maingo🛠️latestticket](semiorepo://definition/semio-repo/cli/main.go/LatestTicket)

LatestTicket MUST complete the operation and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️openticket](semiorepo://definition/semio-repo/cli/main.go/OpenTicket)

OpenTicket MUST complete the operation and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️opengoal](semiorepo://definition/semio-repo/cli/main.go/OpenGoal)

OpenGoal MUST complete the operation and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️updatetickettitle](semiorepo://definition/semio-repo/cli/main.go/UpdateTicketTitle)

UpdateTicketTitle MUST complete the operation and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️createticket](semiorepo://definition/semio-repo/cli/main.go/CreateTicket)

CreateTicket MUST persist the new entity and return a reference to it.

## [🧰semiorepo⌨️cli💻maingo🛠️countlines](semiorepo://definition/semio-repo/cli/main.go/CountLines)

CountLines MUST complete the operation and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️countlinesinfile](semiorepo://definition/semio-repo/cli/main.go/CountLinesInFile)

CountLinesInFile MUST complete the operation and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️countlinesatcommit](semiorepo://definition/semio-repo/cli/main.go/CountLinesAtCommit)

CountLinesAtCommit MUST complete the operation and return consistent results.

## [🧰semiorepo⌨️cli💻maingo🛠️readtextfileatcommit](semiorepo://definition/semio-repo/cli/main.go/ReadTextFileAtCommit)

ReadTextFileAtCommit MUST return the text file at commit content or an error if unavailable.

## [🧰semiorepo⌨️cli💻maingo🛠️listfilesatcommit](semiorepo://definition/semio-repo/cli/main.go/ListFilesAtCommit)

ListFilesAtCommit MUST return all available files at commit entries.

## [🧰semiorepo⌨️cli💻maingo🛠️filterticketworkspacefiles](semiorepo://definition/semio-repo/cli/main.go/FilterTicketWorkspaceFiles)

FilterTicketWorkspaceFiles MUST return only entries that match the filter criteria.

## [🧰semiorepo⌨️cli💻maingo🛠️saveticket](semiorepo://definition/semio-repo/cli/main.go/SaveTicket)

SaveTicket MUST persist the ticket atomically to the data store.

## [🧰semiorepo⌨️cli💻maingo🛠️readticket](semiorepo://definition/semio-repo/cli/main.go/ReadTicket)

ReadTicket MUST return the ticket content or an error if unavailable.

## [🧰semiorepo⌨️cli💻maingo🛠️listtickets](semiorepo://definition/semio-repo/cli/main.go/ListTickets)

ListTickets MUST return all available tickets entries.

## [🧰semiorepo⌨️cli💻maingo🛠️streamtickets](semiorepo://definition/semio-repo/cli/main.go/StreamTickets)

StreamTickets MUST invoke the callback for each matching tickets entry.

## [🧰semiorepo⌨️cli💻maingo🛠️invalidateprojectcache](semiorepo://definition/semio-repo/cli/main.go/InvalidateProjectCache)

InvalidateProjectCache MUST clear the cached state to force a reload.

## [🧰semiorepo⌨️cli💻maingo🛠️loadprojects](semiorepo://definition/semio-repo/cli/main.go/LoadProjects)

LoadProjects MUST return all matching projects from the data source.

## [🧰semiorepo⌨️cli💻maingo🛠️loadcommits](semiorepo://definition/semio-repo/cli/main.go/LoadCommits)

LoadCommits MUST return all matching commits from the data source.

## [🧰semiorepo⌨️cli💻maingo🛠️loadbundles](semiorepo://definition/semio-repo/cli/main.go/LoadBundles)

LoadBundles MUST return all matching bundles from the data source.

## [🧰semiorepo⌨️cli💻maingo🛠️getprojects](semiorepo://definition/semio-repo/cli/main.go/GetProjects)

GetProjects MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️streambundles](semiorepo://definition/semio-repo/cli/main.go/StreamBundles)

StreamBundles MUST invoke the callback for each matching bundles entry.

## [🧰semiorepo⌨️cli💻maingo🛠️streamprojects](semiorepo://definition/semio-repo/cli/main.go/StreamProjects)

StreamProjects MUST invoke the callback for each matching projects entry.

## [🧰semiorepo⌨️cli💻maingo🛠️streamfolders](semiorepo://definition/semio-repo/cli/main.go/StreamFolders)

StreamFolders MUST invoke the callback for each matching folders entry.

## [🧰semiorepo⌨️cli💻maingo🛠️streamfiles](semiorepo://definition/semio-repo/cli/main.go/StreamFiles)

StreamFiles MUST invoke the callback for each matching files entry.

## [🧰semiorepo⌨️cli💻maingo🛠️streamsections](semiorepo://definition/semio-repo/cli/main.go/StreamSections)

StreamSections MUST invoke the callback for each matching sections entry.

## [🧰semiorepo⌨️cli💻maingo🛠️streamdefinitions](semiorepo://definition/semio-repo/cli/main.go/StreamDefinitions)

StreamDefinitions MUST invoke the callback for each matching definitions entry.

## [🧰semiorepo⌨️cli💻maingo🛠️resolvebundleforpath](semiorepo://definition/semio-repo/cli/main.go/ResolveBundleForPath)

ResolveBundleForPath MUST return the resolved value or an error if unresolvable.

## [🧰semiorepo⌨️cli💻maingo🛠️progressticket](semiorepo://definition/semio-repo/cli/main.go/ProgressTicket)

ProgressTicket MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️finishticket](semiorepo://definition/semio-repo/cli/main.go/FinishTicket)

FinishTicket MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️reopenticket](semiorepo://definition/semio-repo/cli/main.go/ReopenTicket)

ReopenTicket MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️toolticketopen](semiorepo://definition/semio-repo/cli/main.go/ToolTicketOpen)

ToolTicketOpen MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️toolticketlist](semiorepo://definition/semio-repo/cli/main.go/ToolTicketList)

ToolTicketList MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️toolticketread](semiorepo://definition/semio-repo/cli/main.go/ToolTicketRead)

ToolTicketRead MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️toolticketclose](semiorepo://definition/semio-repo/cli/main.go/ToolTicketClose)

ToolTicketClose MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️toolticketreopen](semiorepo://definition/semio-repo/cli/main.go/ToolTicketReopen)

ToolTicketReopen MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️tooldraftcreate](semiorepo://definition/semio-repo/cli/main.go/ToolDraftCreate)

ToolDraftCreate MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️tooldraftlist](semiorepo://definition/semio-repo/cli/main.go/ToolDraftList)

ToolDraftList MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️tooldraftdelete](semiorepo://definition/semio-repo/cli/main.go/ToolDraftDelete)

ToolDraftDelete MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️toolgoalcreate](semiorepo://definition/semio-repo/cli/main.go/ToolGoalCreate)

ToolGoalCreate MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️toolgoallist](semiorepo://definition/semio-repo/cli/main.go/ToolGoalList)

ToolGoalList MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️toolgoalclose](semiorepo://definition/semio-repo/cli/main.go/ToolGoalClose)

ToolGoalClose MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️toolgoalreopen](semiorepo://definition/semio-repo/cli/main.go/ToolGoalReopen)

ToolGoalReopen MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️toolcontributoradd](semiorepo://definition/semio-repo/cli/main.go/ToolContributorAdd)

ToolContributorAdd MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️toolcontributorlist](semiorepo://definition/semio-repo/cli/main.go/ToolContributorList)

ToolContributorList MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️toolcontributorremove](semiorepo://definition/semio-repo/cli/main.go/ToolContributorRemove)

ToolContributorRemove MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️toolprojectlist](semiorepo://definition/semio-repo/cli/main.go/ToolProjectList)

ToolProjectList MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️toolbundlelist](semiorepo://definition/semio-repo/cli/main.go/ToolBundleList)

ToolBundleList MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️toolprojecttree](semiorepo://definition/semio-repo/cli/main.go/ToolProjectTree)

ToolProjectTree MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️toolfoldercreate](semiorepo://definition/semio-repo/cli/main.go/ToolFolderCreate)

ToolFolderCreate MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️toolfoldermove](semiorepo://definition/semio-repo/cli/main.go/ToolFolderMove)

ToolFolderMove MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️toolfolderdelete](semiorepo://definition/semio-repo/cli/main.go/ToolFolderDelete)

ToolFolderDelete MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️toolfolderlist](semiorepo://definition/semio-repo/cli/main.go/ToolFolderList)

ToolFolderList MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️toolfoldertree](semiorepo://definition/semio-repo/cli/main.go/ToolFolderTree)

ToolFolderTree MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️toolfilecreate](semiorepo://definition/semio-repo/cli/main.go/ToolFileCreate)

ToolFileCreate MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️fileheaderid](semiorepo://definition/semio-repo/cli/main.go/FileHeaderId)

FileHeaderId MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️agpllicensetext](semiorepo://definition/semio-repo/cli/main.go/AGPLLicenseText)

AGPLLicenseText MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️fileheaderuri](semiorepo://definition/semio-repo/cli/main.go/FileHeaderUri)

FileHeaderUri MUST return the semiorepo URI for a file path.

## [🧰semiorepo⌨️cli💻maingo🛠️sectionheaderid](semiorepo://definition/semio-repo/cli/main.go/SectionHeaderId)

SectionHeaderId MUST return the section artifact ID for a file path and section path.

## [🧰semiorepo⌨️cli💻maingo🛠️sectionheaderuri](semiorepo://definition/semio-repo/cli/main.go/SectionHeaderUri)

SectionHeaderUri MUST return the semiorepo URI for a section.

## [🧰semiorepo⌨️cli💻maingo🛠️definitionheaderid](semiorepo://definition/semio-repo/cli/main.go/DefinitionHeaderId)

DefinitionHeaderId MUST return the definition artifact ID for a file path, section path, and definition name.

## [🧰semiorepo⌨️cli💻maingo🛠️definitionheaderuri](semiorepo://definition/semio-repo/cli/main.go/DefinitionHeaderUri)

DefinitionHeaderUri MUST return the semiorepo URI for a definition.

## [🧰semiorepo⌨️cli💻maingo🛠️toolfilemove](semiorepo://definition/semio-repo/cli/main.go/ToolFileMove)

ToolFileMove MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️toolfiledelete](semiorepo://definition/semio-repo/cli/main.go/ToolFileDelete)

ToolFileDelete MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️toolfilelist](semiorepo://definition/semio-repo/cli/main.go/ToolFileList)

ToolFileList MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️toolfiletree](semiorepo://definition/semio-repo/cli/main.go/ToolFileTree)

ToolFileTree MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️toolsectioncreate](semiorepo://definition/semio-repo/cli/main.go/ToolSectionCreate)

ToolSectionCreate MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️toolsectionmove](semiorepo://definition/semio-repo/cli/main.go/ToolSectionMove)

ToolSectionMove MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️toolintegrate](semiorepo://definition/semio-repo/cli/main.go/ToolIntegrate)

ToolIntegrate MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️toolextract](semiorepo://definition/semio-repo/cli/main.go/ToolExtract)

ToolExtract MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️updateagentsdocspath](semiorepo://definition/semio-repo/cli/main.go/UpdateAgentsDocsPath)

UpdateAgentsDocsPath MUST apply the update and return an error if the target is missing.

## [🧰semiorepo⌨️cli💻maingo🛠️removeagentsdocsentry](semiorepo://definition/semio-repo/cli/main.go/RemoveAgentsDocsEntry)

RemoveAgentsDocsEntry MUST remove the target and return an error on failure.

## [🧰semiorepo⌨️cli💻maingo🛠️splitheader](semiorepo://definition/semio-repo/cli/main.go/SplitHeader)

SplitHeader MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️mergeheaders](semiorepo://definition/semio-repo/cli/main.go/MergeHeaders)

MergeHeaders MUST combine the inputs and return the merged result.

## [🧰semiorepo⌨️cli💻maingo🛠️uniquestrings](semiorepo://definition/semio-repo/cli/main.go/UniqueStrings)

UniqueStrings MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️toolsectiondelete](semiorepo://definition/semio-repo/cli/main.go/ToolSectionDelete)

ToolSectionDelete MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️toolsectionlist](semiorepo://definition/semio-repo/cli/main.go/ToolSectionList)

ToolSectionList MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️toolsectiontree](semiorepo://definition/semio-repo/cli/main.go/ToolSectionTree)

ToolSectionTree MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️tooldefinitionlist](semiorepo://definition/semio-repo/cli/main.go/ToolDefinitionList)

ToolDefinitionList MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️tooldefinitiontree](semiorepo://definition/semio-repo/cli/main.go/ToolDefinitionTree)

ToolDefinitionTree MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️toolupdatemetabolism](semiorepo://definition/semio-repo/cli/main.go/ToolUpdateMetabolism)

ToolUpdateMetabolism MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️exporttosqlite](semiorepo://definition/semio-repo/cli/main.go/ExportToSQLite)

ExportToSQLite MUST write the complete output to the target.

## [🧰semiorepo⌨️cli💻maingo🛠️toolexport](semiorepo://definition/semio-repo/cli/main.go/ToolExport)

ToolExport MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️newresolver](semiorepo://definition/semio-repo/cli/main.go/NewResolver)

NewResolver MUST initialize all required fields and return a valid resolver.

## [🧰semiorepo⌨️cli💻maingo🛠️newresolverwithcontext](semiorepo://definition/semio-repo/cli/main.go/NewResolverWithContext)

NewResolverWithContext MUST initialize all required fields and return a valid resolver with context.

## [🧰semiorepo⌨️cli💻maingo🛠️newdefaultcontext](semiorepo://definition/semio-repo/cli/main.go/NewDefaultContext)

NewDefaultContext MUST initialize all required fields and return a valid default context.

## [🧰semiorepo⌨️cli💻maingo🛠️newrepocontext](semiorepo://definition/semio-repo/cli/main.go/NewRepoContext)

NewRepoContext MUST initialize all required fields and return a valid repo context.

## [🧰semiorepo⌨️cli💻maingo🛠️getrootdir](semiorepo://definition/semio-repo/cli/main.go/GetRootDir)

GetRootDir MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️getfileid](semiorepo://definition/semio-repo/cli/main.go/GetFileID)

GetFileID MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️getfolderid](semiorepo://definition/semio-repo/cli/main.go/GetFolderID)

GetFolderID MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️getbundles](semiorepo://definition/semio-repo/cli/main.go/GetBundles)

GetBundles MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️getprojects](semiorepo://definition/semio-repo/cli/main.go/GetProjects)

GetProjects MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️getcommits](semiorepo://definition/semio-repo/cli/main.go/GetCommits)

GetCommits MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️getfolders](semiorepo://definition/semio-repo/cli/main.go/GetFolders)

GetFolders MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️getfiles](semiorepo://definition/semio-repo/cli/main.go/GetFiles)

GetFiles MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️getdefinitions](semiorepo://definition/semio-repo/cli/main.go/GetDefinitions)

GetDefinitions MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️getsections](semiorepo://definition/semio-repo/cli/main.go/GetSections)

GetSections MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️getcontributors](semiorepo://definition/semio-repo/cli/main.go/GetContributors)

GetContributors MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️gettickets](semiorepo://definition/semio-repo/cli/main.go/GetTickets)

GetTickets MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️getgoals](semiorepo://definition/semio-repo/cli/main.go/GetGoals)

GetGoals MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️goalcreate](semiorepo://definition/semio-repo/cli/main.go/GoalCreate)

GoalCreate MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️updategoaltitle](semiorepo://definition/semio-repo/cli/main.go/UpdateGoalTitle)

UpdateGoalTitle MUST apply the update and return an error if the target is missing.

## [🧰semiorepo⌨️cli💻maingo🛠️goalchange](semiorepo://definition/semio-repo/cli/main.go/GoalChange)

GoalChange MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️goalclose](semiorepo://definition/semio-repo/cli/main.go/GoalClose)

GoalClose MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️goalreopen](semiorepo://definition/semio-repo/cli/main.go/GoalReopen)

GoalReopen MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️ticketchange](semiorepo://definition/semio-repo/cli/main.go/TicketChange)

TicketChange MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️goaldelete](semiorepo://definition/semio-repo/cli/main.go/GoalDelete)

GoalDelete MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️ticketdelete](semiorepo://definition/semio-repo/cli/main.go/TicketDelete)

TicketDelete MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️getdrafts](semiorepo://definition/semio-repo/cli/main.go/GetDrafts)

GetDrafts MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️draftcreate](semiorepo://definition/semio-repo/cli/main.go/DraftCreate)

DraftCreate MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️draftdelete](semiorepo://definition/semio-repo/cli/main.go/DraftDelete)

DraftDelete MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️getpolicies](semiorepo://definition/semio-repo/cli/main.go/GetPolicies)

GetPolicies MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️getstatutes](semiorepo://definition/semio-repo/cli/main.go/GetStatutes)

GetStatutes MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️analyze](semiorepo://definition/semio-repo/cli/main.go/Analyze)

Analyze MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️fix](semiorepo://definition/semio-repo/cli/main.go/Fix)

Fix MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️ticketopen](semiorepo://definition/semio-repo/cli/main.go/TicketOpen)

TicketOpen MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️ticketprogress](semiorepo://definition/semio-repo/cli/main.go/TicketProgress)

TicketProgress MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️ticketclose](semiorepo://definition/semio-repo/cli/main.go/TicketClose)

TicketClose MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️ticketreopen](semiorepo://definition/semio-repo/cli/main.go/TicketReopen)

TicketReopen MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️foldercreate](semiorepo://definition/semio-repo/cli/main.go/FolderCreate)

FolderCreate MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️foldermove](semiorepo://definition/semio-repo/cli/main.go/FolderMove)

FolderMove MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️folderdelete](semiorepo://definition/semio-repo/cli/main.go/FolderDelete)

FolderDelete MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️filecreate](semiorepo://definition/semio-repo/cli/main.go/FileCreate)

FileCreate MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️filemove](semiorepo://definition/semio-repo/cli/main.go/FileMove)

FileMove MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️filedelete](semiorepo://definition/semio-repo/cli/main.go/FileDelete)

FileDelete MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️sectioncreate](semiorepo://definition/semio-repo/cli/main.go/SectionCreate)

SectionCreate MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️sectionmove](semiorepo://definition/semio-repo/cli/main.go/SectionMove)

SectionMove MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️sectiondelete](semiorepo://definition/semio-repo/cli/main.go/SectionDelete)

SectionDelete MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️integrate](semiorepo://definition/semio-repo/cli/main.go/Integrate)

Integrate MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️extract](semiorepo://definition/semio-repo/cli/main.go/Extract)

Extract MUST return the extracted component from the input.

## [🧰semiorepo⌨️cli💻maingo🛠️contributoradd](semiorepo://definition/semio-repo/cli/main.go/ContributorAdd)

ContributorAdd MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️contributorremove](semiorepo://definition/semio-repo/cli/main.go/ContributorRemove)

ContributorRemove MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️syncmanagement](semiorepo://definition/semio-repo/cli/main.go/SyncManagement)

SyncManagement MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️getrootdir](semiorepo://definition/semio-repo/cli/main.go/GetRootDir)

GetRootDir MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️getbundles](semiorepo://definition/semio-repo/cli/main.go/GetBundles)

GetBundles MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️getprojects](semiorepo://definition/semio-repo/cli/main.go/GetProjects)

GetProjects MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️getcommits](semiorepo://definition/semio-repo/cli/main.go/GetCommits)

GetCommits MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️getfolders](semiorepo://definition/semio-repo/cli/main.go/GetFolders)

GetFolders MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️getfiles](semiorepo://definition/semio-repo/cli/main.go/GetFiles)

GetFiles MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️getdefinitions](semiorepo://definition/semio-repo/cli/main.go/GetDefinitions)

GetDefinitions MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️getsections](semiorepo://definition/semio-repo/cli/main.go/GetSections)

GetSections MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️getcontributors](semiorepo://definition/semio-repo/cli/main.go/GetContributors)

GetContributors MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️gettickets](semiorepo://definition/semio-repo/cli/main.go/GetTickets)

GetTickets MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️getpolicies](semiorepo://definition/semio-repo/cli/main.go/GetPolicies)

GetPolicies MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️getstatutes](semiorepo://definition/semio-repo/cli/main.go/GetStatutes)

GetStatutes MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️analyze](semiorepo://definition/semio-repo/cli/main.go/Analyze)

Analyze MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️fix](semiorepo://definition/semio-repo/cli/main.go/Fix)

Fix MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️ticketopen](semiorepo://definition/semio-repo/cli/main.go/TicketOpen)

TicketOpen MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️ticketprogress](semiorepo://definition/semio-repo/cli/main.go/TicketProgress)

TicketProgress MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️ticketclose](semiorepo://definition/semio-repo/cli/main.go/TicketClose)

TicketClose MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️ticketreopen](semiorepo://definition/semio-repo/cli/main.go/TicketReopen)

TicketReopen MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️ticketchange](semiorepo://definition/semio-repo/cli/main.go/TicketChange)

TicketChange MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️foldercreate](semiorepo://definition/semio-repo/cli/main.go/FolderCreate)

FolderCreate MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️foldermove](semiorepo://definition/semio-repo/cli/main.go/FolderMove)

FolderMove MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️folderdelete](semiorepo://definition/semio-repo/cli/main.go/FolderDelete)

FolderDelete MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️filecreate](semiorepo://definition/semio-repo/cli/main.go/FileCreate)

FileCreate MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️filemove](semiorepo://definition/semio-repo/cli/main.go/FileMove)

FileMove MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️filedelete](semiorepo://definition/semio-repo/cli/main.go/FileDelete)

FileDelete MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️sectioncreate](semiorepo://definition/semio-repo/cli/main.go/SectionCreate)

SectionCreate MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️sectionmove](semiorepo://definition/semio-repo/cli/main.go/SectionMove)

SectionMove MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️sectiondelete](semiorepo://definition/semio-repo/cli/main.go/SectionDelete)

SectionDelete MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️integrate](semiorepo://definition/semio-repo/cli/main.go/Integrate)

Integrate MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️extract](semiorepo://definition/semio-repo/cli/main.go/Extract)

Extract MUST return the extracted component from the input.

## [🧰semiorepo⌨️cli💻maingo🛠️contributoradd](semiorepo://definition/semio-repo/cli/main.go/ContributorAdd)

ContributorAdd MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️contributorremove](semiorepo://definition/semio-repo/cli/main.go/ContributorRemove)

ContributorRemove MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️syncmanagement](semiorepo://definition/semio-repo/cli/main.go/SyncManagement)

SyncManagement MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️getgoals](semiorepo://definition/semio-repo/cli/main.go/GetGoals)

GetGoals MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️goalcreate](semiorepo://definition/semio-repo/cli/main.go/GoalCreate)

GoalCreate MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️goalchange](semiorepo://definition/semio-repo/cli/main.go/GoalChange)

GoalChange MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️goalclose](semiorepo://definition/semio-repo/cli/main.go/GoalClose)

GoalClose MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️goalreopen](semiorepo://definition/semio-repo/cli/main.go/GoalReopen)

GoalReopen MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️goaldelete](semiorepo://definition/semio-repo/cli/main.go/GoalDelete)

GoalDelete MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️ticketdelete](semiorepo://definition/semio-repo/cli/main.go/TicketDelete)

TicketDelete MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️getdrafts](semiorepo://definition/semio-repo/cli/main.go/GetDrafts)

GetDrafts MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️draftcreate](semiorepo://definition/semio-repo/cli/main.go/DraftCreate)

DraftCreate MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️draftdelete](semiorepo://definition/semio-repo/cli/main.go/DraftDelete)

DraftDelete MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️gettodos](semiorepo://definition/semio-repo/cli/main.go/GetTodos)

GetTodos MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️todocreate](semiorepo://definition/semio-repo/cli/main.go/TodoCreate)

TodoCreate MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️todochange](semiorepo://definition/semio-repo/cli/main.go/TodoChange)

TodoChange MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️tododelete](semiorepo://definition/semio-repo/cli/main.go/TodoDelete)

TodoDelete MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️newexecutor](semiorepo://definition/semio-repo/cli/main.go/NewExecutor)

NewExecutor MUST initialize all required fields and return a valid executor.

## [🧰semiorepo⌨️cli💻maingo🛠️newexecutorwithcontext](semiorepo://definition/semio-repo/cli/main.go/NewExecutorWithContext)

NewExecutorWithContext MUST initialize all required fields and return a valid executor with context.

## [🧰semiorepo⌨️cli💻maingo🛠️execute](semiorepo://definition/semio-repo/cli/main.go/Execute)

Execute MUST execute the operation to completion and report any errors.

## [🧰semiorepo⌨️cli💻maingo🛠️executejson](semiorepo://definition/semio-repo/cli/main.go/ExecuteJSON)

ExecuteJSON MUST execute the operation to completion and report any errors.

## [🧰semiorepo⌨️cli💻maingo🛠️validatequery](semiorepo://definition/semio-repo/cli/main.go/ValidateQuery)

ValidateQuery MUST return nil when valid and a descriptive error otherwise.

## [🧰semiorepo⌨️cli💻maingo🛠️getoperationtype](semiorepo://definition/semio-repo/cli/main.go/GetOperationType)

GetOperationType MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️query](semiorepo://definition/semio-repo/cli/main.go/Query)

Query MUST execute the query and return matching results.

## [🧰semiorepo⌨️cli💻maingo🛠️drafts](semiorepo://definition/semio-repo/cli/main.go/Drafts)

Drafts MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️node](semiorepo://definition/semio-repo/cli/main.go/Node)

Node MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️repo](semiorepo://definition/semio-repo/cli/main.go/Repo)

Repo MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️projects](semiorepo://definition/semio-repo/cli/main.go/Projects)

Projects MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️project](semiorepo://definition/semio-repo/cli/main.go/Project)

Project MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️bundles](semiorepo://definition/semio-repo/cli/main.go/Bundles)

Bundles MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️folders](semiorepo://definition/semio-repo/cli/main.go/Folders)

Folders MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️files](semiorepo://definition/semio-repo/cli/main.go/Files)

Files MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️sections](semiorepo://definition/semio-repo/cli/main.go/Sections)

Sections MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️definitions](semiorepo://definition/semio-repo/cli/main.go/Definitions)

Definitions MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️contributors](semiorepo://definition/semio-repo/cli/main.go/Contributors)

Contributors MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️todos](semiorepo://definition/semio-repo/cli/main.go/Todos)

Todos MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️tickets](semiorepo://definition/semio-repo/cli/main.go/Tickets)

Tickets MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️interactions](semiorepo://definition/semio-repo/cli/main.go/Interactions)

Interactions MUST perform the Interactions operation.

## [🧰semiorepo⌨️cli💻maingo🛠️policies](semiorepo://definition/semio-repo/cli/main.go/Policies)

Policies MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️statutes](semiorepo://definition/semio-repo/cli/main.go/Statutes)

Statutes MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️breachs](semiorepo://definition/semio-repo/cli/main.go/Breachs)

Breachs MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️bundle](semiorepo://definition/semio-repo/cli/main.go/Bundle)

Bundle MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️folder](semiorepo://definition/semio-repo/cli/main.go/Folder)

Folder MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️file](semiorepo://definition/semio-repo/cli/main.go/File)

File MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️section](semiorepo://definition/semio-repo/cli/main.go/Section)

Section MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️definition](semiorepo://definition/semio-repo/cli/main.go/Definition)

Definition MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️contributor](semiorepo://definition/semio-repo/cli/main.go/Contributor)

Contributor MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️ticket](semiorepo://definition/semio-repo/cli/main.go/Ticket)

Ticket MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️policy](semiorepo://definition/semio-repo/cli/main.go/Policy)

Policy MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️statute](semiorepo://definition/semio-repo/cli/main.go/Statute)

Statute MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️analyze](semiorepo://definition/semio-repo/cli/main.go/Analyze)

Analyze MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️mutation](semiorepo://definition/semio-repo/cli/main.go/Mutation)

Mutation MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️syncmanagement](semiorepo://definition/semio-repo/cli/main.go/SyncManagement)

SyncManagement MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️fix](semiorepo://definition/semio-repo/cli/main.go/Fix)

Fix MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️draftcreate](semiorepo://definition/semio-repo/cli/main.go/DraftCreate)

DraftCreate MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️draftdelete](semiorepo://definition/semio-repo/cli/main.go/DraftDelete)

DraftDelete MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️ticketopen](semiorepo://definition/semio-repo/cli/main.go/TicketOpen)

TicketOpen MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️ticketclose](semiorepo://definition/semio-repo/cli/main.go/TicketClose)

TicketClose MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️ticketreopen](semiorepo://definition/semio-repo/cli/main.go/TicketReopen)

TicketReopen MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️ticketchange](semiorepo://definition/semio-repo/cli/main.go/TicketChange)

TicketChange MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️goalcreate](semiorepo://definition/semio-repo/cli/main.go/GoalCreate)

GoalCreate MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️goalchange](semiorepo://definition/semio-repo/cli/main.go/GoalChange)

GoalChange MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️goalclose](semiorepo://definition/semio-repo/cli/main.go/GoalClose)

GoalClose MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️todocreate](semiorepo://definition/semio-repo/cli/main.go/TodoCreate)

TodoCreate MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️todochange](semiorepo://definition/semio-repo/cli/main.go/TodoChange)

TodoChange MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️tododelete](semiorepo://definition/semio-repo/cli/main.go/TodoDelete)

TodoDelete MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️goalreopen](semiorepo://definition/semio-repo/cli/main.go/GoalReopen)

GoalReopen MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️ticketprogress](semiorepo://definition/semio-repo/cli/main.go/TicketProgress)

TicketProgress MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️goaldelete](semiorepo://definition/semio-repo/cli/main.go/GoalDelete)

GoalDelete MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️ticketdelete](semiorepo://definition/semio-repo/cli/main.go/TicketDelete)

TicketDelete MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️contributoradd](semiorepo://definition/semio-repo/cli/main.go/ContributorAdd)

ContributorAdd MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️contributorremove](semiorepo://definition/semio-repo/cli/main.go/ContributorRemove)

ContributorRemove MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️foldercreate](semiorepo://definition/semio-repo/cli/main.go/FolderCreate)

FolderCreate MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️foldermove](semiorepo://definition/semio-repo/cli/main.go/FolderMove)

FolderMove MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️folderdelete](semiorepo://definition/semio-repo/cli/main.go/FolderDelete)

FolderDelete MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️filecreate](semiorepo://definition/semio-repo/cli/main.go/FileCreate)

FileCreate MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️filemove](semiorepo://definition/semio-repo/cli/main.go/FileMove)

FileMove MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️filedelete](semiorepo://definition/semio-repo/cli/main.go/FileDelete)

FileDelete MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️sectioncreate](semiorepo://definition/semio-repo/cli/main.go/SectionCreate)

SectionCreate MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️sectionmove](semiorepo://definition/semio-repo/cli/main.go/SectionMove)

SectionMove MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️sectiondelete](semiorepo://definition/semio-repo/cli/main.go/SectionDelete)

SectionDelete MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️integrate](semiorepo://definition/semio-repo/cli/main.go/Integrate)

Integrate MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️extract](semiorepo://definition/semio-repo/cli/main.go/Extract)

Extract MUST return the extracted component from the input.

## [🧰semiorepo⌨️cli💻maingo🛠️repo](semiorepo://definition/semio-repo/cli/main.go/Repo_)

Repo_ MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️bundles](semiorepo://definition/semio-repo/cli/main.go/Bundles)

Bundles MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️folders](semiorepo://definition/semio-repo/cli/main.go/Folders)

Folders MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️files](semiorepo://definition/semio-repo/cli/main.go/Files)

Files MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️sections](semiorepo://definition/semio-repo/cli/main.go/Sections)

Sections MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️definitions](semiorepo://definition/semio-repo/cli/main.go/Definitions)

Definitions MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️contributors](semiorepo://definition/semio-repo/cli/main.go/Contributors)

Contributors MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️todos](semiorepo://definition/semio-repo/cli/main.go/Todos)

Todos MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️tickets](semiorepo://definition/semio-repo/cli/main.go/Tickets)

Tickets MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️policies](semiorepo://definition/semio-repo/cli/main.go/Policies)

Policies MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️statutes](semiorepo://definition/semio-repo/cli/main.go/Statutes)

Statutes MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️breachs](semiorepo://definition/semio-repo/cli/main.go/Breachs)

Breachs MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️scopetofiles](semiorepo://definition/semio-repo/cli/main.go/ScopeToFiles)

ScopeToFiles MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️computeticketfiles](semiorepo://definition/semio-repo/cli/main.go/ComputeTicketFiles)

ComputeTicketFiles MUST return the computed result deterministically.

## [🧰semiorepo⌨️cli💻maingo🛠️getgitdifflines](semiorepo://definition/semio-repo/cli/main.go/GetGitDiffLines)

GetGitDiffLines MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️cancloseticket](semiorepo://definition/semio-repo/cli/main.go/CanCloseTicket)

CanCloseTicket MUST return a deterministic boolean result.

## [🧰semiorepo⌨️cli💻maingo🛠️getbundlebypath](semiorepo://definition/semio-repo/cli/main.go/GetBundleByPath)

GetBundleByPath MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️guesssectionname](semiorepo://definition/semio-repo/cli/main.go/GuessSectionName)

GuessSectionName MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️getgitdiffsectionlinemetrics](semiorepo://definition/semio-repo/cli/main.go/GetGitDiffSectionLineMetrics)

GetGitDiffSectionLineMetrics MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️flattensections](semiorepo://definition/semio-repo/cli/main.go/FlattenSections)

FlattenSections MUST return a single-level collection with all nested items.

## [🧰semiorepo⌨️cli💻maingo🛠️buildgitdiffargs](semiorepo://definition/semio-repo/cli/main.go/BuildGitDiffArgs)

BuildGitDiffArgs MUST construct and return the fully initialized result.

## [🧰semiorepo⌨️cli💻maingo🛠️getgitdiffstatus](semiorepo://definition/semio-repo/cli/main.go/GetGitDiffStatus)

GetGitDiffStatus MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️getfolderchildren](semiorepo://definition/semio-repo/cli/main.go/GetFolderChildren)

GetFolderChildren MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️getfolderfiles](semiorepo://definition/semio-repo/cli/main.go/GetFolderFiles)

GetFolderFiles MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️analyzefile](semiorepo://definition/semio-repo/cli/main.go/AnalyzeFile)

AnalyzeFile MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️parsecontributoridentity](semiorepo://definition/semio-repo/cli/main.go/ParseContributorIdentity)

ParseContributorIdentity MUST return the parsed result or an error for invalid input.

## [🧰semiorepo⌨️cli💻maingo🛠️listcontributors](semiorepo://definition/semio-repo/cli/main.go/ListContributors)

ListContributors MUST return all available contributors entries.

## [🧰semiorepo⌨️cli💻maingo🛠️streamcontributors](semiorepo://definition/semio-repo/cli/main.go/StreamContributors)

StreamContributors MUST invoke the callback for each matching contributors entry.

## [🧰semiorepo⌨️cli💻maingo🛠️getcontributoravatarpath](semiorepo://definition/semio-repo/cli/main.go/GetContributorAvatarPath)

GetContributorAvatarPath MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️getcontributoravatarroundpath](semiorepo://definition/semio-repo/cli/main.go/GetContributorAvatarRoundPath)

GetContributorAvatarRoundPath MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️getcontributorpath](semiorepo://definition/semio-repo/cli/main.go/GetContributorPath)

GetContributorPath MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️createcontributor](semiorepo://definition/semio-repo/cli/main.go/CreateContributor)

CreateContributor MUST create a new entry and return an error on conflict.

## [🧰semiorepo⌨️cli💻maingo🛠️loadcontributor](semiorepo://definition/semio-repo/cli/main.go/LoadContributor)

LoadContributor MUST return all matching contributor from the data source.

## [🧰semiorepo⌨️cli💻maingo🛠️savecontributor](semiorepo://definition/semio-repo/cli/main.go/SaveContributor)

SaveContributor MUST persist the contributor atomically to the data store.

## [🧰semiorepo⌨️cli💻maingo🛠️removecontributor](semiorepo://definition/semio-repo/cli/main.go/RemoveContributor)

RemoveContributor MUST remove the target and return an error on failure.

## [🧰semiorepo⌨️cli💻maingo🛠️getregisteredpolicies](semiorepo://definition/semio-repo/cli/main.go/GetRegisteredPolicies)

GetRegisteredPolicies MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️bundles](semiorepo://definition/semio-repo/cli/main.go/Bundles)

Bundles MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️folders](semiorepo://definition/semio-repo/cli/main.go/Folders)

Folders MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️files](semiorepo://definition/semio-repo/cli/main.go/Files)

Files MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️sections](semiorepo://definition/semio-repo/cli/main.go/Sections)

Sections MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️definitions](semiorepo://definition/semio-repo/cli/main.go/Definitions)

Definitions MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️contributors](semiorepo://definition/semio-repo/cli/main.go/Contributors)

Contributors MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️policies](semiorepo://definition/semio-repo/cli/main.go/Policies)

Policies MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️statutes](semiorepo://definition/semio-repo/cli/main.go/Statutes)

Statutes MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️breachs](semiorepo://definition/semio-repo/cli/main.go/Breachs)

Breachs MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️toolanalyze](semiorepo://definition/semio-repo/cli/main.go/ToolAnalyze)

ToolAnalyze MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️toolfix](semiorepo://definition/semio-repo/cli/main.go/ToolFix)

ToolFix MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️toolpolicylist](semiorepo://definition/semio-repo/cli/main.go/ToolPolicyList)

ToolPolicyList MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️toolpolicytree](semiorepo://definition/semio-repo/cli/main.go/ToolPolicyTree)

ToolPolicyTree MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️toolpolicycheck](semiorepo://definition/semio-repo/cli/main.go/ToolPolicyCheck)

ToolPolicyCheck MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️toolpolicybreachlist](semiorepo://definition/semio-repo/cli/main.go/ToolPolicyBreachList)

ToolPolicyBreachList MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🪨blockedtoolpatterns](semiorepo://definition/semio-repo/cli/main.go/BlockedToolPatterns)

BlockedToolPatterns lists shell command patterns that MUST always be denied.

## [🧰semiorepo⌨️cli💻maingo🛠️movefile](semiorepo://definition/semio-repo/cli/main.go/MoveFile)

MoveFile MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️copyfile](semiorepo://definition/semio-repo/cli/main.go/CopyFile)

CopyFile MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️getrepogoalsdir](semiorepo://definition/semio-repo/cli/main.go/GetRepoGoalsDir)

GetRepoGoalsDir MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️listgoals](semiorepo://definition/semio-repo/cli/main.go/ListGoals)

ListGoals MUST return all available goals entries.

## [🧰semiorepo⌨️cli💻maingo🛠️readgoal](semiorepo://definition/semio-repo/cli/main.go/ReadGoal)

ReadGoal MUST return the goal content or an error if unavailable.

## [🧰semiorepo⌨️cli💻maingo🛠️streamgoals](semiorepo://definition/semio-repo/cli/main.go/StreamGoals)

StreamGoals MUST invoke the callback for each matching goals entry.

## [🧰semiorepo⌨️cli💻maingo🛠️streamstatutes](semiorepo://definition/semio-repo/cli/main.go/StreamStatutes)

StreamStatutes MUST invoke the callback for each matching statutes entry.

## [🧰semiorepo⌨️cli💻maingo🛠️streamcommits](semiorepo://definition/semio-repo/cli/main.go/StreamCommits)

StreamCommits MUST invoke the callback for each matching commits entry.

## [🧰semiorepo⌨️cli💻maingo🛠️savegoal](semiorepo://definition/semio-repo/cli/main.go/SaveGoal)

SaveGoal MUST persist the goal atomically to the data store.

## [🧰semiorepo⌨️cli💻maingo🛠️resolvecontributorcontributions](semiorepo://definition/semio-repo/cli/main.go/ResolveContributorContributions)

ResolveContributorContributions MUST return the resolved value or an error if unresolvable.

## [🧰semiorepo⌨️cli💻maingo🛠️gettodos](semiorepo://definition/semio-repo/cli/main.go/GetTodos)

GetTodos MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️scantodos](semiorepo://definition/semio-repo/cli/main.go/ScanTodos)

ScanTodos MUST scan the input completely and collect all matches.

## [🧰semiorepo⌨️cli💻maingo🛠️parsetodomarkdown](semiorepo://definition/semio-repo/cli/main.go/ParseTodoMarkdown)

ParseTodoMarkdown MUST return the parsed result or an error for invalid input.

## [🧰semiorepo⌨️cli💻maingo🛠️parsetodocomments](semiorepo://definition/semio-repo/cli/main.go/ParseTodoComments)

ParseTodoComments MUST return the parsed result or an error for invalid input.

## [🧰semiorepo⌨️cli💻maingo🛠️todocreate](semiorepo://definition/semio-repo/cli/main.go/TodoCreate)

TodoCreate MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️todochange](semiorepo://definition/semio-repo/cli/main.go/TodoChange)

TodoChange MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️tododelete](semiorepo://definition/semio-repo/cli/main.go/TodoDelete)

TodoDelete MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️todototicket](semiorepo://definition/semio-repo/cli/main.go/TodoToTicket)

TodoToTicket MUST return a non-nil error when the operation fails.

## [🧰semiorepo⌨️cli💻maingo🛠️string](semiorepo://definition/semio-repo/cli/main.go/String)

String MUST return a non-empty string representation.

## [🧰semiorepo⌨️cli💻maingo🛠️parseartifactref](semiorepo://definition/semio-repo/cli/main.go/ParseArtifactRef)

ParseArtifactRef MUST return the parsed result or an error for invalid input.

## [🧰semiorepo⌨️cli💻maingo🛠️unslugify](semiorepo://definition/semio-repo/cli/main.go/UnSlugify)

UnSlugify MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️findsectionbyslug](semiorepo://definition/semio-repo/cli/main.go/FindSectionBySlug)

FindSectionBySlug MUST return the matching result or an error if not found.

## [🧰semiorepo⌨️cli💻maingo🛠️resolvesectionname](semiorepo://definition/semio-repo/cli/main.go/ResolveSectionName)

ResolveSectionName MUST return the resolved value or an error if unresolvable.

## [🧰semiorepo⌨️cli💻maingo🛠️sectionidvaluetouripath](semiorepo://definition/semio-repo/cli/main.go/SectionIdValueToUriPath)

SectionIdValueToUriPath MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️definitionidvaluetouripath](semiorepo://definition/semio-repo/cli/main.go/DefinitionIdValueToUriPath)

DefinitionIdValueToUriPath MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️parsesectionuripath](semiorepo://definition/semio-repo/cli/main.go/ParseSectionUriPath)

ParseSectionUriPath MUST return the parsed result or an error for invalid input.

## [🧰semiorepo⌨️cli💻maingo🛠️statuteidtouripath](semiorepo://definition/semio-repo/cli/main.go/StatuteIdToUriPath)

StatuteIdToUriPath MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️statuteuripathtoid](semiorepo://definition/semio-repo/cli/main.go/StatuteUriPathToId)

StatuteUriPathToId MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️getartifactid](semiorepo://definition/semio-repo/cli/main.go/GetArtifactID)

GetArtifactID MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️getartifacturi](semiorepo://definition/semio-repo/cli/main.go/GetArtifactURI)

GetArtifactURI MUST retrieve the requested value or return an error.

## [🧰semiorepo⌨️cli💻maingo🛠️idtouri](semiorepo://definition/semio-repo/cli/main.go/IdToUri)

IdToUri MUST complete the operation successfully.

## [🧰semiorepo⌨️cli💻maingo🛠️uritoid](semiorepo://definition/semio-repo/cli/main.go/UriToId)

UriToId MUST complete the operation successfully.

## [🧰semiorepo📚go💻emitgo🛠️emit](semiorepo://definition/semio-repo/go/emit.go/Emit)

Emit MUST perform the Emit operation.

## [🧰semiorepo⌨️server💻maingo🔖package](semiorepo://section/Package)

Package declaration for the semio repo server binary. MUST be package main.

## [🧰semiorepo⌨️server💻maingo🔖imports](semiorepo://section/Imports)

Standard library and third-party imports MUST be grouped by origin.

## [🧰semiorepo⌨️server💻maingo🔖config](semiorepo://section/Config)

Server configuration loading from environment variables. MUST provide sensible defaults.

## [🧰semiorepo⌨️server💻maingo🔖models](semiorepo://section/Models)

Data model types for tickets, scopes, warnings, breachs, events, and API request/response payloads. MUST mirror the server SQLite schema.

## [🧰semiorepo⌨️server💻maingo🔖database](semiorepo://section/Database)

SQLite database layer for persistent storage of tickets, scopes, claims, warnings, breachs, and events. MUST use WAL journal mode.

## [🧰semiorepo⌨️server💻maingo🔖eventbus](semiorepo://section/EventBus)

Asynchronous in-process event bus for decoupled event publishing and subscription. MUST persist events to the database before dispatching.

## [🧰semiorepo⌨️server💻maingo🔖diffparsing](semiorepo://section/DiffParsing)

Unified diff parser that extracts file paths and hunk line ranges from patch text. MUST handle standard git diff output format.

## [🧰semiorepo⌨️server💻maingo🔖indexing](semiorepo://section/Indexing)

Source code indexer that parses files into scopes covering files, sections, and definitions. MUST support region-marker-based sections and language-specific definition patterns.

## [🧰semiorepo⌨️server💻maingo🔖claims](semiorepo://section/Claims)

Scope claim mapping logic that associates diff hunks with overlapping scopes. MUST detect multi-ticket conflicts.

## [🧰semiorepo⌨️server💻maingo🔖warnings](semiorepo://section/Warnings)

Conflict warning generation from multi-ticket scope overlaps. MUST produce error-severity warnings for blocking conflicts.

## [🧰semiorepo⌨️server💻maingo🔖server](semiorepo://section/Server)

HTTP server with ticket lifecycle, diff ingestion, pre-commit checks, indexing, and webhook endpoints. MUST enforce authentication on mutating routes.

## [🧰semiorepo⌨️server💻maingo🔖processing](semiorepo://section/Processing)

Diff processing pipeline that indexes changed files, maps claims, detects conflicts, and produces warnings. MUST be transactional per request.

## [🧰semiorepo⌨️server💻maingo🔖webhooks](semiorepo://section/Webhooks)

GitHub webhook handlers for issue comment caching and issue event processing. MUST verify HMAC signatures when a secret is configured.

## [🧰semiorepo⌨️server💻maingo🔖discord](semiorepo://section/Discord)

Discord notification integration for ticket lifecycle events. MUST silently skip when no webhook URL is configured.

## [🧰semiorepo⌨️server💻maingo🔖utilities](semiorepo://section/Utilities)

Shared utility functions used across the server. MUST produce unique identifiers.

## [🧰semiorepo⌨️server💻maingo🔖main](semiorepo://section/Main)

Application entry point that initializes the database, event bus, server, and HTTP routes. MUST register all handlers before listening.
MUST open the database, start the event bus, register all routes, and block on ListenAndServe.

## [🧰semiorepo⌨️server💻maingo🛠️opendatabase](semiorepo://definition/semio-repo/server/main.go/openDatabase)

MUST enable WAL journal mode and foreign keys.

## [🧰semiorepo⌨️server💻maingo🛠️close](semiorepo://definition/semio-repo/server/main.go/Close)

MUST release all database resources.

## [🧰semiorepo⌨️server💻maingo🛠️neweventbus](semiorepo://definition/semio-repo/server/main.go/NewEventBus)

MUST initialize the channel buffer to 256 and create a cancellable context.

## [🧰semiorepo⌨️server💻maingo🛠️subscribe](semiorepo://definition/semio-repo/server/main.go/Subscribe)

MUST append the handler to the handlers map.

## [🧰semiorepo⌨️server💻maingo🛠️publish](semiorepo://definition/semio-repo/server/main.go/Publish)

MUST store the event in the database before sending to the channel.

## [🧰semiorepo⌨️server💻maingo🛠️start](semiorepo://definition/semio-repo/server/main.go/Start)

MUST consume events from the channel and invoke registered handlers.

## [🧰semiorepo⌨️server💻maingo🛠️stop](semiorepo://definition/semio-repo/server/main.go/Stop)

MUST block until the goroutine exits.

## [🧰semiorepo⌨️server💻maingo🛠️newserver](semiorepo://definition/semio-repo/server/main.go/NewServer)

MUST initialize the index cache and GitHub comment cache.

## [🧰semiorepo⌨️server💻maingo🛠️processdiff](semiorepo://definition/semio-repo/server/main.go/processDiff)

MUST return warnings and breachs alongside the processing result.

## [🧰semiorepo⌨️server💻maingo🛠️main](semiorepo://definition/semio-repo/server/main.go/main)

MUST open the database, start the event bus, register all routes, and block on ListenAndServe.

## [🧰semiorepo🖱️vscode💻codegents🔖configuration](semiorepo://section/Configuration)

Configuration MUST generate typed client code from the GraphQL schema.

## [🧰semiorepo🖱️vscode💻codegents🪨config](semiorepo://definition/semio-repo/vscode/codegen.ts/config)

Config MUST reference the GraphQL schema and generate client preset output.

## [🧰semiorepo🖱️vscode💻extensionts🔖imports](semiorepo://section/Imports)

Imports MUST include VS Code API, Node.js utilities, and semio validation.

## [🧰semiorepo🖱️vscode💻extensionts🔖constants](semiorepo://section/Constants)

Constants MUST define static configuration for diagnostics and UI strings.

## [🧰semiorepo🖱️vscode💻extensionts🔖types](semiorepo://section/Types)

Types MUST define interfaces for repo events, tool results, and data models.

## [🧰semiorepo🖱️vscode💻extensionts🔖globals](semiorepo://section/Globals)

Globals MUST hold module-level state for output channel, diagnostics, caches, and providers.

## [🧰semiorepo🖱️vscode💻extensionts🔖utilities](semiorepo://section/Utilities)

Utilities MUST provide shared functions for logging, shell execution, and binary resolution.

## [🧰semiorepo🖱️vscode💻extensionts🔖uriresolution](semiorepo://section/URI%20Resolution)

URI Resolution MUST handle parsing, tree node caching, and semiorepo URI navigation.

## [🧰semiorepo🖱️vscode💻extensionts🔖helpers](semiorepo://section/Helpers)

Helpers MUST provide file path extraction, ticket path resolution, and editor navigation.

## [🧰semiorepo🖱️vscode💻extensionts🔖fileanalysisdiagnostics](semiorepo://section/File%20Analysis%20&%20Diagnostics)

File Analysis & Diagnostics MUST handle analysis, breach diagnostics, bundle caching, and kit validation.

## [🧰semiorepo🖱️vscode💻extensionts🔖providers](semiorepo://section/Providers)

Providers MUST implement VS Code tree data providers for filter, monorepo, and sections views.

## [🧰semiorepo🖱️vscode💻extensionts🔖activation](semiorepo://section/Activation)

Activation MUST handle extension activation, command registration, and lifecycle management.

## [🧰semiorepo🖱️vscode💻queriests🔖queries](semiorepo://section/Queries)

Typed GraphQL document constants MUST use generated graphql tag functions.
