# Summary - Fix VS Code Extension Activation and Tree Providers

The VS Code extension now correctly activates and registers all TreeView providers at the beginning of the activation sequence. This resolves the intermediate "No data provider registered" error and infinite loading spinners. GraphQL data fetching has been unified and hardened against shell escaping issues on Linux, with detailed logging in a dedicated "semio" output channel.

## Changes
-   **VS Code Manifest**: Updated engine version and added view-specific activation events.
-   **Extension Logic**: Reordered registration, improved URQL fetch robustness, and unified codebase loading.
-   **Packaging**: Fixed VSIX creation requirements and optimized file exclusions.
-   **Container Integration**: Updated `post-attach.sh` to correctly build the repo binary.

## Continued: "No sections found" Investigation

Investigated the sections tree view showing "No sections found" for all files. Added comprehensive logging throughout the section fetching pipeline to diagnose the issue.

### Key Findings
-   **CLI Works Correctly**: `repo section list` command returns sections for all supported languages.
-   **Code Path Verified**: Section fetching flow (`SectionsProvider.getChildren()` → `getSectionListForFile()` → `runRepoCommandJson()` → `extractSections()`) is logically correct.
-   **Added Logging**: Added logging at each step to help identify where issues may occur in specific environments.

### Supported Languages for Sections
TypeScript, Python, Go, Rust, Markdown (headings), Shell, C#, Ruby, JSON, TOML, GraphQL.

### Tests Added
-   Section view registration
-   Section commands (sectionTree, sectionList, sectionCreate, sectionMove, sectionDelete, sectionOpen, sectionRename, sectionIntegrate)
