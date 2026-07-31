# Weak entity TypeScript mirrors

`🪶️WeakEntities` in `compose/js/index.ts` declares minimal `*Wire` interfaces for host-side typing. They are **not** a byte-for-byte mirror of every field in `target.schema.graphql` (Place/Location/Camera/Side may differ); tighten them when sketchpad consumes those shapes.
