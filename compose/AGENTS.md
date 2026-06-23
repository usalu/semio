---
emoji: 🏘️
---

# 🧾 Specification

## Strict layering (compose wasm host)

Dependency direction is **one step down only** (no skipping, no upward imports):

`compose/sketchpad` → `compose/react` → `compose/js` → **GraphQL** → `compose/rs`

- **`compose/rs`**: sole owner of domain logic, kit caches, semantic change semantics, and diff algebra. One logical process (WASM worker or OS native); async, non-blocking toward clients.
- **`compose/js`**: thin GraphQL client + `KitStore` (typed methods, subscription callbacks). No kit authority, no domain caches.
- **`compose/react`**: thin adapter to `@semio-tech/compose-js` stores; hooks use `useSyncExternalStore` (or equivalent) on those subscriptions for external reads.
- **`compose/sketchpad`**: UI + wiring only; kit I/O exclusively through `@semio-tech/compose-react` exports (never `@semio-tech/compose-js`).

## 🕸️ Systems

### Kits, Families, Designs, Types

### Types, Representations, Ports, Connectors

### Designs, Pieces, Connections, Layers, Groups

### Stats, Attributes,

## 🛠️ Mechanisms

### InMemory

**Layout:** In `compose/rs`, the live in-memory `KitGraph` is updated on the DTO-mutation path: `ChangeKitCommand` (and graph helpers that delegate to it) compute the next `KitFullDto` on an isolated `KitGraph` clone, then `KitGraph::apply_kit_diff` merges a `KitDiff` and runs `apply_kit_state` (full re-layout) plus `emit_kit_dto_reconcile_events`. WIP and other callers that refresh the live `KitRef` with `from_full` / `from_full_dto` must not bypass that contract.

## 📛 Entities

$$
\Sigma := \text{finite strings},
\qquad
\mathbb{B} := \{\mathrm{true},\mathrm{false}\},
\qquad
\mathbb{R} := \text{real numbers}.
$$

$$
X \rightharpoonup Y := \text{partial functions},
\qquad
\bot := \text{unspecified optional value},
\qquad
\top := \text{present without explicit value}.
$$

Every `###` entity in this section carries an explicit **rank** (non-negative integer, lower first). **Rules:** if A’s definition materially uses B as a constituent (not only as an opaque forward reference), then `rank(B) < rank(A)`; among independent peers, lower structural complexity first; on cycles, the lower-complexity entity gets the lower rank; numeric spaces over $\mathbb{R}$ rank before geometric composites built from them.

### Id · rank 0

A id is an immutable uuid-v7 string of the creation timestamp.

### Coordinate · rank 1

$$
\operatorname{Point} := \mathbb{R}^2
$$

### Offset · rank 2

$$
\operatorname{Point} := \mathbb{R}^2
$$

### Point · rank 3

$$
\operatorname{Point} := \mathbb{R}^3
$$

### Vector · rank 4

$$
\operatorname{Vector} := \mathbb{R}^3
$$

### ◳ Plane · rank 5

$$
\operatorname{Plane}
:=
\left\{
(o,x,y) \in \operatorname{Point} \times \operatorname{Vector} \times \operatorname{Vector}
\;\middle|\;
 x \neq 0,\; y \neq 0,\; x \not\parallel y
\right\}.
$$

### 🔗 Url · rank 6

$$
\operatorname{Url} := \Sigma,
\qquad
\operatorname{Url} = \operatorname{RelativeUrl} \sqcup \operatorname{RemoteUrl}.
$$

### 🏷️ Attribute · rank 7

$$
a = (key, value, unit, definition),
$$

$$
key \in \Sigma,
\qquad
value \in \Sigma \cup \{\top,\bot\},
\qquad
unit \in \Sigma \cup \{\bot\},
\qquad
definition \in \Sigma \cup \{\bot\}.
$$

### 🏷️ Tag · rank 8

$$
t = (id, name, description, icon, attributes).
$$

### 📈 Stat · rank 9

$$
s = (key, unit, min, minExcluded, max, maxExcluded).
$$

### ⚙️ Prop · rank 10

$$
\pi = (key, value, unit, attributes).
$$

### 👤 Author · rank 11

$$
u = (name, email, attributes).
$$

### 🏷️ Concept · rank 12

$$
c = (id, name, description, icon, attributes).
$$

### 📊 Benchmark · rank 13

$$
b = (name, icon, min, minExcluded, max, maxExcluded, definition, attributes).
$$

### 💾 Representation · rank 14

$$
m = (id, name, tags, file, description, attributes).
$$

$$
\operatorname{sim}(m_1,m_2)
=
\frac{|tags(m_1) \cap tags(m_2)|}{|tags(m_1) \cup tags(m_2)|},
\qquad
|tags(m_1) \cup tags(m_2)| \neq 0.
$$

### ⚓ Connector · rank 15

$$
\kappa =
(
id,
point,
direction,
t,
mandatory,
port,
compatiblePorts,
props,
description,
attributes
),
$$

$$
point \in \operatorname{Point},
\qquad
direction \in \operatorname{Vector},
\qquad
t \in [0,1).
$$

$$
\operatorname{port}^{\ast}(\kappa)
=
\begin{cases}
port(\kappa), & port(\kappa) \neq \bot,\\
\bot, & \text{otherwise.}
\end{cases}
$$

$$
\operatorname{compatible}(\kappa_1,\kappa_2)
\iff
\bigl(compatiblePorts(\kappa_1)=\varnothing\bigr)
\lor
\bigl(compatiblePorts(\kappa_2)=\varnothing\bigr)
\lor
\bigl(\operatorname{port}^{\ast}(\kappa_2) \in compatiblePorts(\kappa_1)\bigr)
\lor
\bigl(\operatorname{port}^{\ast}(\kappa_1) \in compatiblePorts(\kappa_2)\bigr).
$$

### 🏠 Type · rank 16

$$
\tau =
(
name,
representations,
connectors,
props,
isVirtual,
canScale,
canMirror,
unit,
availableCount,
location,
authors,
concepts,
icon,
image,
description,
attributes,
created,
updated
).
$$

$$
\operatorname{parent}^{\mathrm{type}} : \mathcal{T} \rightharpoonup \mathcal{T},
\qquad
\operatorname{proto}(\tau) \iff \operatorname{parent}^{\mathrm{type}}(\tau) = \bot.
$$

### ⭕ Piece · rank 17

$$
p =
(
id,
ref,
plane,
center,
scale,
mirrorPlane,
props,
hidden,
locked,
color,
description,
attributes
),
\qquad
ref(p) \in \mathcal{T} \sqcup \mathcal{D}.
$$

$$
\operatorname{fixed}(p) \iff plane(p) \neq \bot \land center(p) \neq \bot.
$$

$$
\operatorname{linked}(p) \iff plane(p) = \bot \land center(p) = \bot.
$$

$$
F_d := \{p \in P_d \mid \operatorname{fixed}(p)\}.
$$

$$
\operatorname{hierarchy}_d(p)
:=
\min_{f \in F_d} \operatorname{dist}_{\mathcal{G}(d)}(f,p).
$$

$$
\mathcal{F}_d := \text{a breadth-first spanning forest of } \mathcal{G}(d) \text{ rooted at } F_d.
$$

$$
\operatorname{parent}_d : P_d \rightharpoonup P_d.
$$

$$
\operatorname{path}_d(p)
=
\begin{cases}
[], & \operatorname{parent}_d(p) = \bot,\\[4pt]
\operatorname{path}_d(\operatorname{parent}_d(p)) \mathbin{+\!\!+} [\operatorname{parent}_d(p)], & \text{otherwise.}
\end{cases}
$$

$$
\operatorname{ancestor}_d(x,y) \iff x \in \operatorname{path}_d(y).
$$

$$
\operatorname{descendant}_d(y,x) \iff x \in \operatorname{path}_d(y).
$$

$$
\operatorname{child}_d(c,p) \iff \operatorname{parent}_d(c) = p.
$$

$$
\operatorname{grandchild}_d(g,p)
\iff
\exists c \in P_d:
\operatorname{child}_d(c,p) \land \operatorname{child}_d(g,c).
$$

$$
\operatorname{root}_d(p) \iff \operatorname{parent}_d(p) = \bot.
$$

$$
\operatorname{leaf}_d(p) \iff \neg \exists c \in P_d : \operatorname{child}_d(c,p).
$$

$$
\operatorname{sibling}_d(p,q)
\iff
p \neq q \land \operatorname{parent}_d(p) = \operatorname{parent}_d(q) \neq \bot.
$$

### 🔗 Connection · rank 18

$$
\sigma = (piece, connector, designPiece^{\ast}),
\qquad
designPiece^{\ast} \in P_d \cup \{\bot\}.
$$

$$
e =
(
\sigma_c,
\sigma_g,
gap,
shift,
rise,
rotation,
turn,
tilt,
x,
y,
description,
attributes
).
$$

$$
\operatorname{ends}(e) = \{\sigma_c.piece, \sigma_g.piece\}.
$$

$$
\operatorname{lower}(e)
:=
\arg\min_{p \in \operatorname{ends}(e)} \operatorname{hierarchy}_d(p),
\qquad
\operatorname{higher}(e)
:=
\arg\max_{p \in \operatorname{ends}(e)} \operatorname{hierarchy}_d(p).
$$

### 📋 Layer · rank 19

$$
\lambda = (path, isHidden, isLocked, color, description, attributes).
$$

### 👥 Group · rank 20

$$
g = (pieces, color, name, description, attributes).
$$

### 🔢 Quality · rank 21

$$
\operatorname{QualityKind}
\subseteq
\{\mathrm{General},\mathrm{Design},\mathrm{Type},\mathrm{Piece},\mathrm{Connection},\mathrm{Connector}\}.
$$

$$
q =
(
key,
name,
kind,
default,
formula,
defaultSiUnit,
defaultImperialUnit,
min,
minExcluded,
max,
maxExcluded,
canScale,
benchmarks,
definition,
attributes
).
$$

### 🏘 Design · rank 22

$$
d =
(
name,
P_d,
E_d,
S_d,
\Pi_d,
L_d,
G_d,
canScale,
canMirror,
unit,
location,
authors,
concepts,
icon,
image,
description,
attributes,
created,
updated
).
$$

$$
\mathcal{G}(d) = (P_d, \sim_d),
\qquad
p \sim_d q
\iff
\exists e \in E_d \text{ joining } p \text{ and } q.
$$

$$
\operatorname{directlyConnected}_d(p,q) \iff p \sim_d q.
$$

$$
\operatorname{connected}_d(p,q)
\iff
\exists n \ge 0,\; \exists p_0,\dots,p_n \in P_d:
\; p_0 = p,\; p_n = q,\; p_i \sim_d p_{i+1}.
$$

$$
\operatorname{component}_d(p) = [p]_{\operatorname{connected}_d}.
$$

$$
\operatorname{parent}^{\mathrm{design}} : \mathcal{D} \rightharpoonup \mathcal{D},
\qquad
\operatorname{proto}(d) \iff \operatorname{parent}^{\mathrm{design}}(d) = \bot.
$$

### 📦 Kit · rank 23

$$
K = (T_K, D_K, Q_K, F_K, A_K, C_K, \Gamma_K, Attr_K, description, metadata).
$$

$$
T_K \subseteq \mathcal{T},
\qquad
D_K \subseteq \mathcal{D},
\qquad
Q_K = \text{qualities},
\qquad
F_K = \text{files},
$$

$$
A_K = \text{authors},
\qquad
C_K = \text{concepts},
\qquad
\Gamma_K = \text{tags},
\qquad
Attr_K = \text{kit-level attributes}.
$$

- `kit store` is the master process and is full control plane to do everything. It has three concurrent tasks: wip kit, backbone kit stub and kit coordinator. It has a kit conflict registry to manage conflicts between the wip kit and the backbone kit.
- `wip kit` is an async task that is a replica of the kit graph.
- `backbone kit stub` an async task kit graph stub to an authorative persisted out-of-process kit graph. **Backbone kinds** (attach at runtime via `compose-store` JSON-RPC `backbone.attach`): **Dev** — single JSON file; **Local** — folder with `.compose/kit.db` (and file blobs); **Remote** — hub session (pull; propose may require owner credentials). Related RPC: `backbone.detach`, `backbone.status`, `backbone.setActiveCheckpoint`, `conflicts.list`, `conflicts.resolve`, `coordinator.syncNow`.
- `kit graph` is a complete in-memory kit graph (including history, sessions, drafts, transactions, etc)
- `kit coordinator` is an asnyc task to coordinate the wip kit process and the backbone kit graph process.
- `kit history` is the complete history of a kit (initial kit, checkpoints, alternatives)
- `kit checkpoint tree` is the tree of all checkpoints.
- `initial kit` is a kit snapshot.
- `kit checkpoint` is a compressed list of kit changes with an optional message, timestamp and authors.
- `kit change` is a forward list of kit change commands and a backward list of kit change commands.
- `kit session` is a stateful session that a client can open (e.g. when sketchpad opens a kit for the first time a kit session is opened).
- `kit draft` is a draft is a stack of kit transactions for a checkpoint within a session. Undo/redo support. A draft is only allowed on the last checkpoint of an alternative or the last checkpoint of `the kit`.
- `kit transaction` is a raw list of kit changes for a draft. Undo/redo support.
- `kit alternative` is a named list of checkpoints (starting from `the kit` and then more linear checkpoints). Multiple alternatives can shared checkpoints. Checkpoints are stored individually.
- `kit diff` is a diff to a kit snapshot.
- `kit command` is a command to a `kit store`
- `kit read command` is a read-only command to a `kit store`
- `kit change command` is a command that changes part of the kit within a `kit transaction`
- `kit snapshot` is a point-in-time representation of a kit.
- `materialized kit` is a computed kit snapshot that is computed from an initial kit
- `the kit` means the the last materlialized from non-alternative
- `kit release` is checkpoint that is marked for released and is additionally stored as materialized kit.
