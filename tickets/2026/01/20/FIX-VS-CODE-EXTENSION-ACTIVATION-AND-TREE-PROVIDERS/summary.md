# Summary - Fix VS Code Extension Activation and Tree Providers

The VS Code extension now correctly activates and registers all TreeView providers at the beginning of the activation sequence. This resolves the intermediate "No data provider registered" error and infinite loading spinners. GraphQL data fetching has been unified and hardened against shell escaping issues on Linux, with detailed logging in a dedicated "semio" output channel.

## Changes
-   **VS Code Manifest**: Updated engine version and added view-specific activation events.
-   **Extension Logic**: Reordered registration, improved URQL fetch robustness, and unified codebase loading.
-   **Packaging**: Fixed VSIX creation requirements and optimized file exclusions.
-   **Container Integration**: Updated `post-attach.sh` to correctly build the repo binary.
