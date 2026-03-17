---
name: studio
kind: bundle
emoji: 👥
summary: Y.js CRDT runtime and kit persistence providers for semio sketchpad.
---

### Summary
Y.js CRDT runtime and kit persistence providers for semio sketchpad.

### Specs
- Re-exports Y and IndexeddbPersistence from yjs/y-indexeddb
- Provides PersistenceProvider interface and PersistenceFactory type
- Implements IndexedDB, JSON file, and SQLite folder persistence factories
- YDocBinaryPersistenceProvider uses Y.encodeStateAsUpdate/applyUpdate for CRDT round-tripping
