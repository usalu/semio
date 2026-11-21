# GIS Map Child Stores

The registered GIS component exceeded the WebAssembly linker limit of one million functions while the map editor and viewer instantiated the full Semio child-store roster. The GIS Map document contract declares Drawing, Image and Value children. The map composition now owns the three-member roster used by the editor, viewer, hub and their test fixtures.

The three stores retain their canonical Semio kind, standard, subset, schema, store producer, native genesis and history limits. Image remains supported as an optional child. The existing native GIS plugin registration and the independent Terrain apps continue to use their existing paths.

The existing document contract test now compares the roster with all three child coordinates in its JSON fixture. A static check matched the map roster to both that fixture and the canonical Semio roster. The scoped whitespace check passed. Component linking and runtime checks are tracked by the parent ticket.
