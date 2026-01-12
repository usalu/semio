# Summary - Fix npm Invalid Version error

Fixed `npm install` failure by adding missing `version` fields to workspace member `package.json` files and resolving `vitest` dependency version conflicts. Removed conflicting lockfiles to ensure a clean dependency tree.
