---
name: semio
kind: user
emoji: 🧩
summary: ✏️ Design-Information-Representationing for Kit-of-Parts 🧩
---

# 📑 Overview

1. [🛍 Products](#%EF%B8%8F-products-)
   - [✏ sketchpad](#%EF%B8%8F-sketchpad-)
   - [👥 studio](#-studio-)
   - [☁ cloud](#%EF%B8%8F-cloud-)
   - [🤖 assistant](#-assistant-)
   - [🦗 semio.gh](#-semiogh-)
   - [🦏 semio.3dm](#-semio3dm-)
   - [🐝 semio.wasp](#-semiowasp-)
   - [🦌 semio.monoceros](#-semiomonoceros-)
   - [🐞 semio.ladybug](#-semioladybug-)

# 🧾 Specification

## 🕸️ Systems

### Kit, Design, Types, Ports

### Types, Shapes, Connectors

### Designs, Pieces, Connections, Layers, Groups

### Stats, Attributes,

## 🛠️ Mechanisms

## 📛 Concepts

### 📦 Kit [↑](#-concepts-)

A [`kit`](#-kit-) is a collection of [`types`](#-type-), [`designs`](#%EF%B8%8F-design-), [`authors`](#-author-), [`qualities`](#-quality-), [`attributes`](#%EF%B8%8F-attribute-), and [`concepts`](#%EF%B8%8F-concept-) 📦

The SQL-schema of `kit.db` is found in [`./semio/sqlite/schema.sql`](./semio/sqlite/schema.sql) 📄

For Inter-Process-Communication (IPC) the JSON-schema in [`./semio/jsonschema/kit.json`](./semio/jsonschema/kit.json) is used 📄

####

A [`kit`](#-kit-) is either _static_ (a special `.zip` file) or _dynamic_ (bound to a runtime) 📦

A _static_ [`kit`](#-kit-) contains a reserved `.semio` folder that contains a `kit.db` sqlite file 💾

### 🏘 Design [↑](#-concepts-)

A [`design`](#%EF%B8%8F-design-) is an undirected graph of [`pieces`](#-piece-) (nodes) and [`connections`](#-connection-) (edges) with organizational [`layers`](#-layer-), [`groups`](#-group-), [`stats`](#-stat-), [`attributes`](#%EF%B8%8F-attribute-), and [`concepts`](#%EF%B8%8F-concept-) 📐

A [`design`](#-design-) is _proto_ (a _protodesign_) when it has no _parent_.

_Children_ of a _parent_ are \_subdesigns.

A _flat_ [`design`](#%EF%B8%8F-design-) has no [`connections`](#-connection-) and all [`pieces`](#-piece-) are _fixed_ ◳

The [`pieces`](#-piece-) are _placed_ _hierarchically_ ([breadth-first](https://en.wikipedia.org/wiki/Breadth-first_search)) for every _component_ 🌿

Additional [`connections`](#-connection-) which where not used in the _placement_ can be used to validate the computed [`planes`](#-plane-) 🛂

### 🏠 Type [↑](#-concepts-)

A [`type`](#-type-) is a reusable component with different [`representations`](#-representation-), [`connectors`](#-port-), [`attributes`](#%EF%B8%8F-attribute-), [`concepts`](#%EF%B8%8F-concept-), and [`authors`](#-author-) 🧱

A [`type`](#-type-) is _proto_ (a _prototype_) when it has no _parent_.

_Children_ of a _parent_ are \_subtypes.

A [`type`](#-type-) can be **virtual** (intermediate type requiring other virtual types to form a physical type), **scalable**, and **mirrorable** with **stock** quantity, **unit**, and optional **location** 📍

### 🔗 Connection [↑](#-concepts-)

A [`connection`](#-connection-) is a 3D-Link between two [`pieces`](#-piece-) with the _translation_ parameters **gap** (offset in y-direction), **shift** (offset in x-direction) and **rise** (offset in z-direction), and the _rotation_ parameters **rotation** (rotation around y-axis), **turn** (rotation around z-axis) and **tilt** (rotation around x-axis) 🪢

The _translation_ is applied first, then the _rotation_ 🥈

The two [`pieces`](#-piece-) are called **_connected_** and **_connecting_** but there is no difference between them 🔄

The _direction_ of a [`connection`](#-connection-) goes from the lower _hierarchy_ to the higher _hierarchy_ of the [`pieces`](#-piece-) ➡

A [`connection`](#-connection-) can have [`attributes`](#%EF%B8%8F-attribute-) and diagram positioning with **x** and **y** offsets 📍

### ⭕ Piece [↑](#-concepts-)

A [`piece`](#-piece-) is an instance of either a [`type`](#-type-) or a [`design`](#%EF%B8%8F-design-) with **id**, optional **description**, optional **plane**, **center** position, **scale**, optional **mirror plane**, **hidden** and **locked** states, **color**, and [`attributes`](#%EF%B8%8F-attribute-) 📐

A [`piece`](#-piece-) is either _fixed_ (with a [`plane`](#-plane-)) or _linked_ (with a [`connection`](#-connection-)) 📐

A group of _connected_ [`pieces`](#-piece-) is called a _component_ 🌿

The _hierarchy_ of a [`piece`](#-piece-) is the length of the shortest path to the next _fixed_ [`piece`](#-piece-) 👣

### ⚓ Connector [↑](#-concepts-)

A [`connector`](#-port-) is a conceptual connection **point** with an outwards **direction**, **id**, optional **description**, and **t** value for diagram ring positioning 🤝

A [`connector`](#-port-) can be marked as **mandatory** in which case it is required to be connected to a [`piece`](#-piece-) 💯

A [`connector`](#-port-) can have a connector **port** and a list of **compatible ports** for explicit compatibility control 👨‍👩‍👧‍👦

No **port** means the _default_ port and no **compatible ports** means the connector is compatible with all other connectors 🔑

It is enough for one [`connector`](#-port-) to be compatible with another [`connector`](#-port-) to be compatible with each other ↔

A [`connector`](#-port-) can have [`props`](#-prop-) that define measurable characteristics and [`attributes`](#%EF%B8%8F-attribute-) for additional metadata 📏

### 💾 Representation [↑](#-concepts-)

A [`representation`](#-representation-) is a **[`tagged`](#%EF%B8%8F-tag-)** **[`url`](#-url-)** to a resource with an optional **description** 📄

No **[`tags`](#%EF%B8%8F-tag-)** means the _default_ representation 🔑

The similarity of [`representations`](#-representation-) is determined by the [jaccard index](https://en.wikipedia.org/wiki/Jaccard_index) of their **[`tags`](#%EF%B8%8F-tag-)** 🔄

### 🏷️ Attribute [↑](#-concepts-)

A [`attribute`](#%EF%B8%8F-attribute-) is metadata with a unique **name**, an optional **value**, an optional **unit** and an optional **definition** ([`url`](#-url-) or text) 🔤

The **name** is[kebab-cased](https://en.wikipedia.org/wiki/Kebab_case) and with `.`-separated string similar to [toml keys](https://toml.io/en/v1.0.0#keys) 🔑

No **value** is equivalent to the boolean _true_ where the **name** is the category of the attribute 🔑

The **unit** is a [unit identifier](https://en.wikipedia.org/wiki/Unit_of_measurement) 🔢

- `mm` for millimeter, `cm` for centimeter, `dm` for decimeter, `m` for meter, `km` for kilometer
- `m²` for square meter, `m³` for cubic meter, `m⁴` for quartic meter
- `°` for degree, `rad` for radian
- `N` for newton, `kN` for kilonewton, `MN` for meganewton
- `°C` for degree Celsius, `°F` for degree Fahrenheit
- `W` for watt, `kW` for kilowatt, `MW` for megawatt, `GW` for gigawatt
- `Wh` for watt-hour, `kWh` for kilowatt-hour, `MWh` for megawatt-hour, `GWh` for gigawatt-hour
- `J` for joule, `kJ` for kilojoule, `kcal` for kilocalorie
- `kWh/m²a` for kilowatt-hour per square meter per year
- `m/s` for meter per second, `m²/s` for square meter per second, `m³/s` for cubic meter per second
- `Pa` for pascal, `kPa` for kilopascal, `MPa` for megapascal
- …

A list of [attributes](#%EF%B8%8F-attribute-) is semantically equivalent to nested dictionaries where the key is the **name** and the value is the **value** ↔

### 🏷️ Tag [↑](#-concepts-)

A [`tag`](#%EF%B8%8F-tag-) is a [kebab-cased](https://en.wikipedia.org/wiki/Kebab_case) **name** 🔤

### ◳ Plane [↑](#-concepts-)

A [`plane`](#-plane-) is a location (**origin**) and orientation (**x-axis**, **y-axis** and derived z-axis) in 3D space ✈

The coordinate system is left-handed where the thumb points up into the direction of the z-axis, the index-finger forwards into the direction of the y-axis and the middle-finger points to the right into the direction of the x-axis 👈

### 🔗 Url [↑](#-concepts-)

A [`url`](#-url-) is either _relative_ (to the root of the `.zip` file) or _remote_ (http, https, ftp, …) string🌐

A _relative_ [`url`](#-url-) is a `/`-normalized path to a file in the `.zip` file and is not prefixed with with `.`, `./`, `/`, …

### 🔢 Quality [↑](#-concepts-)

A [`quality`](#-quality-) is a measurement definition with a **key**, **name**, **description**, **kind** (General, Design, Type, Piece, Connection, Connector), **unit information** (SI and Imperial), **range constraints** (min/max with exclusion flags), **default value**, and optional **formula** 📏

A [`quality`](#-quality-) can be **scalable** (adjusts with piece scaling) and have multiple **benchmarks** for performance evaluation 🎯

The **kind** determines which entities the quality can be applied to using a bitwise enum system 🔢

### 📊 Benchmark [↑](#-concepts-)

A [`benchmark`](#-benchmark-) is a performance standard within a [`quality`](#-quality-) with a **name**, optional **icon**, and **range** (min/max with exclusion flags) 🏆

Benchmarks provide reference points for evaluating quality measurements against industry or design standards 📈

### 🏷️ Concept [↑](#-concepts-)

A [`concept`](#%EF%B8%8F-concept-) is a **name** and **order** pair that provides semantic grouping for [`kits`](#-kit-), [`types`](#-type-), or [`designs`](#%EF%B8%8F-design-) 🧠

Concepts enable hierarchical organization and categorization of design elements beyond simple naming 📂

### 👤 Author [↑](#-concepts-)

An [`author`](#-author-) has a **name** and **email** and can be associated with [`kits`](#-kit-), [`types`](#-type-), or [`designs`](#%EF%B8%8F-design-) with a **rank** indicating contribution level 👨‍💻

Authors provide attribution and contact information for design ownership and collaboration 🤝

### 📋 Layer [↑](#-concepts-)

A [`layer`](#-layer-) is an organizational grouping within a [`design`](#%EF%B8%8F-design-) with a **name**, optional **description**, and **color** for visual organization 🎨

Layers provide a way to group and manage pieces logically within complex designs 📑

### 👥 Group [↑](#-concepts-)

A [`group`](#-group-) is a collection of [`pieces`](#-piece-) within a [`design`](#%EF%B8%8F-design-) with optional **name**, **description**, **color**, and **attributes** 👥

Groups enable semantic clustering of pieces that belong together functionally or conceptually 🔗

### ⚙️️ Prop [↑](#-concepts-)

A [`prop`](#-prop-) is a **key-value** pair on a [`connector`](#-port-) that references a [`quality`](#-quality-) with a specific **value** and optional **unit** 🔧

Props define measurable characteristics of connectors using the quality system for standardized measurement 📐

### 📈 Stat [↑](#-concepts-)

A [`stat`](#-stat-) is a statistical measurement on a [`design`](#%EF%B8%8F-design-) that references a [`quality`](#-quality-) with **range** (min/max) and optional **unit** 📊
Stats provide computed or measured performance data for entire designs using the quality framework 📈

# 📦 Bundles [↑](#-overview)

- [algorithms](algorithms/README.md) – Graph matching and placement algorithms
- [antlr](antlr/README.md) – ANTLR lexer and parser definitions
- [assets](assets/README.md) – Visual assets, logotypes, icons
- [desktop](desktop/README.md) – The semio desktop Electron application
- [docs](docs/README.md) – Monorepo documentation site
- [engine](engine/README.md) – Core engine for design resolution
- [examples](examples/README.md) – Example scripts and projects
- [gh](gh/README.md) – Grasshopper plugin integration
- [go](go/README.md) – Go implementation libraries
- [graphql](graphql/README.md) – GraphQL schema extensions
- [js](js/README.md) – TypeScript/Javascript implementations
- [jsonschema](jsonschema/README.md) – JSON Schema specifications
- [liveblocks](liveblocks/README.md) – Liveblocks real-time multiplayer configuration
- [net](net/README.md) – .NET implementations and bridges
- [openapi](openapi/README.md) – OpenAPI REST specification
- [peg](peg/README.md) – PEG parsing definitions
- [play](play/README.md) – semio play environment
- [py](py/README.md) – Python implementations
- [rb](rb/README.md) – Ruby implementations
- [rdf](rdf/README.md) – Semantic web RDF descriptions
- [reports](reports/README.md) – Reporting and statistics outputs
- [rs](rs/README.md) – Rust implementations
- [sites](sites/README.md) – Assorted web deployment packages
- [sketchpad](sketchpad/README.md) – Browser-based sketching application
- [sqlite](sqlite/README.md) – Database schema
- [studio](studio/README.md) – Studio collaboration environment
- [ui](ui/README.md) – Additional UI packages
- [vscode](vscode/README.md) – Language intelligence and extension files
