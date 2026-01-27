# The Programming & Systems Mind Atlas

## How to Use This Manual

This manual teaches programming and systems through the lens of **semio**—a real-world, multi-language monorepo for Design-Information-Modeling in kit-of-parts architecture. Every concept is grounded in actual code, files, and workflows from semio.

**What is semio?**

semio is a software system that helps architects and designers work with modular building components (called a "kit-of-parts"). Instead of designing buildings from scratch, you define reusable **types** (like wall panels, columns, connectors) and assemble them into **designs**. Think of it like LEGO for architecture—but with real engineering data, 3D models, and the ability to generate building plans.

The semio codebase spans multiple programming languages:
- **TypeScript/JavaScript** (`js/`) — The visual editor (Sketchpad), documentation website, VS Code extension
- **Python** (`py/`) — The computational engine for processing kits
- **C#** (`net/`) — Integration with Rhino/Grasshopper (3D modeling software)
- **Go** (`go/`) — Command-line tools and MCP server for AI assistants

This manual uses semio code as examples for every programming concept.

**The Five-Layer Explanation System**

Every concept in this manual is explained using five layers:

1. **Plain**: What this means in everyday life, using analogies you already understand
2. **Technical**: What this actually means in computing, with precise terminology
3. **Why**: Why this was invented—the problem it solves
4. **What it enables**: What becomes possible because this exists
5. **What it limits**: What becomes harder or impossible because of how this works

**Reading Strategies**

- **First read**: Read all five layers for each concept
- **Quick reference**: Read only Plain and Technical
- **Deep understanding**: Focus on Why and What it enables/limits
- **Problem-solving**: Start with What it limits to understand constraints

**Prerequisites**

None. This manual assumes you have never programmed and have only basic familiarity with using a computer (clicking, typing, opening programs).

---

## Part 1: The Foundations

### Chapter 1: What Computers Actually Are

#### 1.1 The Machine That Follows Instructions

**Plain explanation**

Imagine you have an incredibly fast, incredibly obedient, but incredibly literal assistant. This assistant can do exactly what you tell them, millions of times per second, without getting tired. But they have no common sense whatsoever. If you say "design a building," they'll stare blankly. If you say "create a piece of type 'Wall', place it at coordinates (0, 0, 0), then create another piece, connect them at connector 'Left' to connector 'Right' with a gap of 100mm," they'll do it perfectly, forever.

A computer is that assistant. It's a machine that executes instructions exactly as given, at extraordinary speed, with no understanding of what those instructions mean.

**Technical explanation**

A computer is an electronic device that processes data according to a set of instructions called a program. At its core, a computer consists of:

- A processor (CPU) that executes instructions
- Memory (RAM) that holds data and instructions temporarily
- Storage (SSD/HDD) that holds data permanently
- Input/output devices for communication with humans and other systems

The computer operates by fetching an instruction from memory, decoding what that instruction means, executing the instruction, and then moving to the next instruction. This cycle—fetch, decode, execute—happens billions of times per second.

**semio in context**: When you open the semio Sketchpad in your browser, your computer is executing millions of instructions per second—rendering 3D models, tracking mouse movements, calculating where pieces connect, and updating the display 60 times per second. The semio Python engine (`py/engine/engine.py`) processes your kit files, computes piece placements, and validates connections—all through precise instruction sequences.

**Why it was invented**

Before computers, humans performed calculations by hand or with mechanical devices. This was slow, error-prone, and exhausting. During World War II, the need to calculate artillery trajectories, break codes, and process massive amounts of data drove the invention of electronic computers. The fundamental insight was: if we can represent instructions and data as electrical signals, we can process them at the speed of electricity.

**What it enables**

- Processing billions of calculations per second
- Perfect repeatability—the same kit definition always produces the same 3D model
- Tireless operation—semio can process thousands of pieces without fatigue
- Automation of any task that can be described precisely
- Storage and retrieval of vast amounts of design information
- Real-time collaboration across networks (via Liveblocks in semio)

**What it limits**

- Computers can only do what they're instructed to do—semio can't "design something nice"
- Instructions must be completely unambiguous—a connector needs exact coordinates
- Computers don't understand context—they don't know a "wall" is meant to stand upright
- They cannot handle truly novel situations without specific programming
- Physical limitations of electricity, heat, and materials constrain speed
- Any error in instructions propagates perfectly—a wrong connection formula applies everywhere

---

#### 1.2 Memory: The Computer's Scratchpad

**Plain explanation**

Imagine you're designing a building with hundreds of modular parts. You need scratch paper to track which pieces are placed, where each connector is located, and which connections have been made. Without scratch paper, you'd lose track immediately.

Memory is the computer's scratch paper. It's where the computer keeps the information it's currently working with. In semio, when you're editing a design in the Sketchpad, all the pieces, connections, and their properties are held in memory—instantly accessible for display and modification.

**Technical explanation**

Computer memory (RAM—Random Access Memory) is a collection of electronic circuits that can store data temporarily. Memory is organized as a sequence of bytes (each byte is 8 bits, where a bit is a single 0 or 1). Each byte has a unique address—a number that identifies its location.

Key characteristics:

- **Volatile**: Memory contents are lost when power is removed
- **Random access**: Any location can be read or written in approximately the same time
- **Finite**: Modern computers have gigabytes of RAM (billions of bytes)
- **Fast**: Memory access takes nanoseconds (billionths of a second)

**semio in context**: When the semio Sketchpad loads a kit, it reads the kit from storage (a `.zip` file or SQLite database) and creates JavaScript objects in memory. The Y.js library (`KitStore` in `Sketchpad.tsx`) keeps a synchronized copy of all types, designs, pieces, and connections in RAM. When you drag a piece, the position updates in memory instantly—you see the change immediately because memory access is measured in nanoseconds.

```typescript
// From js/semio/sketchpad/Sketchpad.tsx - data held in memory
export class KitStore extends Store<Kit> {
  public readonly yDoc: Y.Doc;  // Y.js document in memory
  private readonly yTypes: Y.Array<Y.Map<unknown>>;  // Types array
  private readonly yDesigns: Y.Array<Y.Map<unknown>>; // Designs array
  // ... all kit data lives here in RAM
}
```

**Why it was invented**

Early computers stored data on slow media like punch cards or magnetic drums. The CPU would have to wait while data was retrieved, wasting its processing power. RAM was invented to provide fast, temporary storage that could keep up with the CPU.

**What it enables**

- Programs can work with large amounts of data simultaneously
- Fast switching between tasks (kit data stays in memory)
- Complex algorithms like piece placement that require storing intermediate results
- Running multiple programs at once (Sketchpad, VS Code, Engine)
- Instant access to any piece or connection currently being edited
- Interactive programs that respond immediately to user input

**What it limits**

- Memory is expensive—large kits with many 3D models may exceed available RAM
- Memory is volatile—if you close the browser without saving, changes are lost
- Programs must fit their working data in available memory
- Memory access is still slower than CPU operations (creating bottlenecks)
- Sharing memory between programs creates complexity and security issues
- Physical limits on how much memory can fit in a computer

---

#### 1.3 The CPU: The Brain

**Plain explanation**

If memory is scratch paper, the CPU (Central Processing Unit) is the architect using that scratch paper. It's the part that actually does the work—calculating where pieces go, checking if connectors are compatible, transforming 3D coordinates, and processing every click and drag.

The CPU is incredibly fast but surprisingly simple. It can only do basic operations: add numbers, compare values, move data from one place to another. The magic is that it does these simple operations billions of times per second, and complex behavior—like rendering a 3D design with hundreds of connected pieces—emerges from combining simple operations.

**Technical explanation**

The CPU is an integrated circuit (chip) that executes instructions. A modern CPU contains:

- **ALU (Arithmetic Logic Unit)**: Performs math and logic operations
- **Registers**: Tiny, extremely fast storage locations inside the CPU
- **Control Unit**: Fetches and decodes instructions
- **Cache**: Small amounts of very fast memory built into the CPU

The CPU operates on a clock cycle. Each cycle, it can perform one or more operations. Modern CPUs run at gigahertz speeds—billions of cycles per second. A 3 GHz CPU performs 3 billion cycles per second.

**semio in context**: When you connect two pieces in semio, the CPU executes millions of simple operations:

1. Read connector position from piece A
2. Read connector direction from piece A  
3. Apply gap/shift/rise transformations
4. Apply rotation/turn/tilt transformations
5. Calculate resulting plane for piece B
6. Store new position in memory
7. Trigger re-render of the 3D scene

Each step breaks down into even simpler operations—loading values, multiplying matrices, storing results. The Three.js library used by semio for 3D rendering executes thousands of these per frame.

**Why it was invented**

The CPU represents the idea of a "universal machine"—a single device that can perform any computation by following different instructions. Before CPUs, machines were built for specific purposes. The insight was: instead of building specialized hardware for each task, build one general-purpose processor and change what it does by changing the instructions.

**What it enables**

- One physical machine can run semio, Rhino, your browser, and any other software
- Software can be updated without changing hardware
- Computers can switch between tasks instantly
- General-purpose computing for any problem that can be described algorithmically
- Economies of scale—mass-produce one type of chip for all applications
- Innovation in software without requiring hardware changes

**What it limits**

- CPUs are sequential—they do one thing at a time (per core)
- Some problems don't map well to the CPU's instruction set (graphics use GPU)
- Heat limits how fast CPUs can run
- Instructions must be fetched from memory, creating delays
- Complex operations like matrix multiplication require many simple instructions
- Power consumption is proportional to speed

---

#### 1.4 Electricity, Bits, and Binary

**Plain explanation**

Computers speak a very simple language: on or off. Every piece of information in a computer is ultimately represented as billions of tiny switches, each either on or off. We call "off" a 0 and "on" a 1. These are bits—binary digits.

It's like Morse code, but faster and with only two symbols. Just as Morse code can represent any message using dots and dashes, binary can represent any information using 0s and 1s. A piece's position (X=1500, Y=2000, Z=0), a connector's direction vector, a type's name ("Wall Panel")—everything becomes patterns of bits.

**Technical explanation**

Binary is a base-2 number system using only 0 and 1. Computers use binary because electronic circuits have two stable states: high voltage (typically representing 1) and low voltage (representing 0). This is much more reliable than trying to distinguish between many voltage levels.

Key concepts:

- **Bit**: A single binary digit (0 or 1)
- **Byte**: 8 bits, can represent 256 different values (2^8)
- **Word**: The CPU's natural data size, typically 32 or 64 bits

**semio in context**: Everything in a semio kit is binary:

```typescript
// A Point in semio (from js/semio/semio.ts)
export const PointSchema = z.object({
  x: z.number(),  // 64-bit floating point = 64 bits
  y: z.number(),  // 64-bit floating point = 64 bits  
  z: z.number(),  // 64-bit floating point = 64 bits
});
// Total: 192 bits = 24 bytes per 3D point

// A GUID (globally unique identifier) for each entity
export type Guid = string;  // 128 bits stored as 36 character string
// Example: "0193f8a2-7b4c-7d8e-9f01-234567890abc"
```

When you save a kit, the JSON is converted to bytes: each character becomes a number (UTF-8 encoding), and those numbers become binary patterns stored on disk.

**Why it was invented**

Binary wasn't invented for computers—it's ancient mathematics. But it was adopted for computers because electronic circuits can reliably distinguish two states. Early computers experimented with decimal (10 states), but noise and variability made it unreliable. Binary is robust: if voltage is above a threshold, it's 1; below, it's 0. Small fluctuations don't cause errors.

**What it enables**

- Extreme reliability in storing and transmitting kit data
- Simple, small circuits (transistors are binary switches)
- Perfect copies—digital kit files can be copied without degradation
- Error detection and correction (using redundant bits)
- Boolean logic maps directly to circuit design
- Massive miniaturization (billions of transistors on a chip)

**What it limits**

- Representing continuous values requires approximation (semio uses floating-point with limited precision)
- Binary is not human-readable—kit files need JSON or SQLite formats
- Some operations need many binary steps
- Floating-point math has precision limits (semio uses `TOLERANCE = 1e-5` to handle rounding)
- Everything must be digitized (3D models become mesh vertices)
- File sizes can be large for high-fidelity 3D model representations

---

#### 1.5 Storage: Permanent Memory

**Plain explanation**

Memory (RAM) forgets everything when you turn off the computer. Storage is like a filing cabinet—it keeps your kits even when the power is off. When you save a design in semio, it moves from the computer's scratch paper (memory) to the filing cabinet (storage).

Storage is slower than memory but permanent and much larger. Your computer might have 16 GB of memory but 1000 GB (1 TB) of storage. That's enough for thousands of semio kits, each with detailed 3D models.

**Technical explanation**

Storage devices retain data without power. Common types:

**Hard Disk Drives (HDD)**:
- Spinning magnetic platters
- Mechanical read/write heads
- Cheap, high capacity
- Slow (milliseconds to access data)
- Moving parts can fail

**Solid State Drives (SSD)**:
- No moving parts
- Flash memory cells trap electrons
- Faster (microseconds)
- More expensive per GB
- Limited write cycles (cells wear out)

**semio in context**: semio uses multiple storage formats:

```
# Static kit storage (a .zip file containing):
kit.zip/
├── .semio/
│   └── kit.db          # SQLite database with kit metadata
├── models/
│   ├── wall-panel.glb  # 3D model files (GLTF binary)
│   └── column.glb
└── images/
    └── thumbnail.png

# The SQLite schema is defined in:
# sql/sqlite/schema.sql
```

When you open a kit in Sketchpad, the browser reads from IndexedDB (browser storage) or fetches from a server. The Python engine (`py/engine/engine.py`) reads `.zip` files and SQLite databases to process kit definitions.

**Why it was invented**

Volatile memory couldn't preserve work between sessions. Early storage was paper tape and punch cards. Magnetic storage (tape, then disks) provided rewritable, permanent storage. The invention of flash memory removed mechanical limitations, making storage fast enough to reduce the gap with RAM.

**What it enables**

- Preserving kits across power cycles
- Storing vastly more data than fits in memory
- Sharing kits between computers via `.zip` files
- Operating systems and programs persist on disk
- SQLite databases can hold complex relational kit data
- Kits can be distributed via npm, PyPI, or direct download

**What it limits**

- Storage is orders of magnitude slower than memory
- Programs must explicitly save data (semio transactions handle this)
- Storage devices can fail, losing data (backups are essential)
- Write operations wear out SSDs
- Large 3D model files take time to read/write
- Fragmentation can slow access

---

#### 1.6 How All These Pieces Talk to Each Other

**Plain explanation**

Imagine an architectural office with different departments (design, engineering, fabrication). These departments need communication channels—email, phone, meetings. The speed of these channels determines how fast the whole office can work.

In a computer, different components (CPU, memory, storage, graphics card) need pathways for information to travel. These pathways are called buses. Some are highways (fast, for critical traffic like memory access), others are local streets (slower, for USB devices).

**Technical explanation**

Computer components communicate through buses—sets of parallel wires that carry data. Key buses include:

**Memory Bus**: Connects CPU to RAM
- Very wide (64-bit or more)
- Very fast (hundreds of millions of transfers per second)

**PCIe (Peripheral Component Interconnect Express)**: Connects expansion cards
- Point-to-point lanes
- High bandwidth for demanding devices (graphics cards, NVMe SSDs)

**USB (Universal Serial Bus)**: Connects external devices
- Standardized connector
- Slower than internal buses
- Supports hot-plugging (connect/disconnect while running)

**semio in context**: When you use the semio Sketchpad:

1. **Memory bus**: The browser's JavaScript engine reads kit data from RAM
2. **PCIe**: The GPU renders 3D models using Three.js
3. **Network**: HTTP requests fetch kit files from servers
4. **Storage bus**: IndexedDB persists changes to SSD

In the development workflow:
- **VS Code extension** (`js/vscode/`) communicates with the Go repo tool via stdin/stdout
- **Python engine** (`py/engine/`) serves a REST API over HTTP (port 2507)
- **Grasshopper plugin** (`net/Semio.Grasshopper/`) communicates with Rhino through the .NET runtime

**Why it was invented**

As computers evolved, specialized components emerged. Rather than integrate everything into one chip, modularity allowed: upgrading components independently, standardization across manufacturers, and competition driving innovation. Buses standardize how components communicate, enabling an ecosystem of compatible parts.

**What it enables**

- Building computers from interchangeable parts
- Upgrading components without replacing everything
- Third-party manufacturers can create compatible devices
- Standardized interfaces (USB, HTTP) work across brands and platforms
- Specialized components for specialized tasks (GPU for 3D rendering)
- Flexible system configuration

**What it limits**

- Communication overhead—data must travel across buses
- Bandwidth bottlenecks—buses have maximum throughput
- Latency—signals take time to travel (network latency for remote kits)
- Power consumption from driving signals
- Physical constraints on connector size and placement
- Compatibility issues when standards change

---

### Chapter 2: What Programming Really Is

#### 2.1 Code: Instructions in Human-Readable Form

**Plain explanation**

Writing instructions for a computer directly in binary (1s and 0s) would be like designing a building by specifying the exact position of every atom. It's technically possible but practically impossible.

Code is a translation layer. You write in something resembling English, and tools translate it into the binary the computer actually understands. The semio codebase is written in human-readable languages (TypeScript, Python, C#, Go), and each gets translated into machine instructions.

**Technical explanation**

Code is text written in a programming language that specifies computations. Code consists of:

- **Statements**: Individual instructions (do this, then that)
- **Expressions**: Calculations that produce values (`gap + shift`)
- **Declarations**: Naming things (`const piece: Piece = ...`)
- **Control structures**: Decisions and repetition (if, while, for)

Code is stored in plain text files with specific extensions (`.ts`, `.py`, `.cs`, `.go`). These files are processed by compilers or interpreters that convert human-readable code into machine code.

**semio in context**: Here's real code from `js/semio/semio.ts`:

```typescript
// Human-readable: create a unique identifier for any entity
export const guid = () => uuidv7();

// Human-readable: check if two pieces refer to the same entity
export const areSamePieceId = (a: PieceId, b: PieceId): boolean => 
  a.guid === b.guid;

// Human-readable: apply a diff to update an attribute
export const applyAttributeDiff = (base: Attribute, diff: AttributeDiff): Attribute => {
  return { ...base, ...diff };
};
```

This TypeScript compiles to JavaScript, which the browser's engine then compiles to machine code. The same logic in Python (`py/engine/engine.py`) and C# (`net/Semio/Semio.cs`) follows similar patterns but with different syntax.

**Why it was invented**

The earliest programmers wrote machine code directly—numeric operation codes. This was error-prone and slow. Assembly language added symbolic names (ADD instead of numeric codes). Higher-level languages abstracted further, letting programmers think in terms of problems (pieces, connections, types) rather than machine operations.

**What it enables**

- Humans can read and understand semio's logic
- The codebase can be thousands of files (semio has ~30,000+ lines)
- Collaboration—multiple developers understand and contribute
- Maintenance—code can be updated years later
- Documentation lives alongside code (AGENTS.md, README.md)
- Patterns and best practices can be taught and shared

**What it limits**

- Translation adds overhead (TypeScript → JavaScript → machine code)
- Abstraction can hide performance implications
- Programmers need to learn language syntax
- Different languages have different capabilities
- Code must be exact—computers are literal
- Understanding what code actually does requires understanding the language

---

#### 2.2 Programming Languages: The Bridge Between Human and Machine

**Plain explanation**

Just as human languages (English, German, Japanese) have different vocabularies and grammars, programming languages (TypeScript, Python, C#) have different keywords and rules. Each language has strengths for certain tasks.

semio uses four main languages, each chosen for where it excels:
- **TypeScript** for web interfaces (Sketchpad in browsers)
- **Python** for computational processing (the engine)
- **C#** for integration with Rhino/Grasshopper (3D modeling)
- **Go** for fast command-line tools and AI integration

**Technical explanation**

A programming language defines:

- **Syntax**: The grammar—how to write valid statements
- **Semantics**: The meaning—what statements do when executed
- **Type system**: How data is categorized and validated
- **Standard library**: Built-in functionality
- **Execution model**: How programs run (compiled, interpreted, etc.)

**semio's language choices**:

| Language | Location | Why Chosen | Execution |
|----------|----------|------------|-----------|
| TypeScript | `js/semio/` | Type-safe JavaScript, runs in browsers | Compiled to JS, interpreted |
| Python | `py/engine/` | Rich data science libraries, readable | Interpreted |
| C# | `net/Semio/` | Required by Rhino/Grasshopper | Compiled to IL, JIT compiled |
| Go | `go/repo/` | Fast startup, single binary, great for CLI | Compiled to native |

```typescript
// TypeScript (js/semio/semio.ts) - static types, compiles to JavaScript
export type Guid = string;
export const areSameTypeId = (a: TypeId, b: TypeId): boolean => a.guid === b.guid;
```

```python
# Python (py/engine/engine.py) - dynamic, interpreted
def validate_kit(kit: Kit) -> ValidationResult:
    problems = []
    # ... validation logic
    return ValidationResult(problems=problems)
```

```csharp
// C# (net/Semio/Semio.cs) - compiled, runs on .NET
public static class Constants {
    public const string Name = "semio";
    public const float Tolerance = 1e-5f;
}
```

```go
// Go (go/repo/main.go) - compiled, single binary
func main() {
    rootCmd.Execute()
}
```

**Why different languages exist**

Different problems favor different tools. semio needs:
- Browser execution → TypeScript/JavaScript (only language browsers run)
- Rhino integration → C# (Rhino's plugin API is .NET)
- Fast CLI → Go (compiles to single fast binary)
- Data processing → Python (numpy, pandas, graphene libraries)

**What it enables**

- Choose the right tool for each platform
- Leverage existing ecosystems (React, FastAPI, .NET, Cobra)
- Performance optimization where needed (Go for CLI speed)
- Type safety where beneficial (TypeScript catches errors at compile time)
- Rich libraries for each domain

**What it limits**

- Developers must learn multiple languages
- Same logic duplicated across languages (Kit model exists in TS, Python, C#, Go)
- Schema synchronization is complex (JSON Schema as source of truth)
- Different testing frameworks per language
- Build systems must coordinate across ecosystems

---

#### 2.3 Instructions: Telling the Computer What to Do

**Plain explanation**

An instruction is a single step: "add these numbers," "find this piece," "connect these connectors." Programs are built from thousands or millions of these tiny steps.

Think of instructions like steps in an architectural drawing. "Draw a building" is too vague. "Draw a line from point (0,0) to point (100,0), then from (100,0) to (100,50)..." is precise. The computer needs each step spelled out. Programming is the art of breaking big tasks into small, precise steps.

**Technical explanation**

At the machine level, instructions are extremely simple operations:

- **Arithmetic**: ADD, SUB, MUL, DIV
- **Logic**: AND, OR, NOT, XOR
- **Memory**: LOAD, STORE
- **Control**: JUMP, BRANCH, CALL, RETURN
- **Comparison**: COMPARE, TEST

**semio in context**: High-level semio code compiles to many machine instructions:

```typescript
// One line of semio TypeScript (from semio.ts)
const plane = applyPlaneDiff(basePlane, diff);
```

This single line becomes something like:
1. Load basePlane.origin.x from memory
2. Load diff.origin.x from memory (if present)
3. Add them together
4. Store result in new plane.origin.x
5. Repeat for origin.y, origin.z
6. Load basePlane.xAxis.x, xAxis.y, xAxis.z
7. ... and so on for all 9 values in a Plane

A simple connection operation in semio involves:
```typescript
// Conceptually: "connect pieceA to pieceB"
// Actually requires dozens of steps:
1. Find pieceA's connector by ID
2. Get connector's point in local coordinates
3. Get connector's direction vector
4. Transform by pieceA's plane
5. Apply gap offset along direction
6. Apply shift offset perpendicular to direction
7. Apply rise offset along Z
8. Apply rotation around direction axis
9. Apply turn around Z axis
10. Apply tilt around perpendicular axis
11. Compute final plane for pieceB
12. Store pieceB's new plane
13. Update the connection record
14. Trigger UI re-render
```

**Why systematic instructions matter**

Computers have no intuition. "Connect nicely" means nothing without specifying:
- Which connector on which piece
- Exact gap, shift, rise values
- Exact rotation, turn, tilt angles

Instructions must be:
- **Unambiguous**: Only one possible interpretation
- **Complete**: Nothing left unspecified
- **Ordered**: The sequence matters (can't apply rotation before knowing the connector)

**What it enables**

- Precise control over piece placement
- Predictable, reproducible designs
- Complex algorithms from simple building blocks
- Optimization of critical operations
- Debugging—trace exactly what happened step by step

**What it limits**

- Every case must be anticipated (what if connector doesn't exist?)
- Verbosity—simple tasks require many instructions
- No implicit understanding of design intent
- Errors in instructions execute faithfully
- Learning curve for precise thinking

---

#### 2.4 Variables: Named Storage Locations

**Plain explanation**

Imagine you're tracking building components and the specification says "use the floor panel" but you have three floor panel types. Which one? Variables solve this by naming things: "groundFloorPanel," "upperFloorPanel," "roofPanel."

A variable is a name that refers to a piece of data. Instead of saying "the value stored in memory address 0x7fff5fbff8ac," you say "piece.plane.origin.x." The computer translates the name to the actual memory location.

**Technical explanation**

A variable binds a name to a storage location. When you write:

```typescript
const gap = 100;
```

The computer:
1. Allocates memory to hold the value 100
2. Associates the name "gap" with that memory location
3. Stores 100 in that location

**semio in context**: Variables throughout the semio codebase:

```typescript
// From js/semio/semio.ts - naming entities
export type Guid = string;  // Type alias: Guid IS a string

// Variables holding kit data
const kit: Kit = loadKit("metabolism.zip");
const types: Type[] = kit.types;  // Array of all types
const firstType: Type = types[0];  // Single type
const connectors: Connector[] = firstType.connectors;  // That type's connectors

// Variables in connection calculations
const connectorPoint: Point = connector.point;  // {x: 0, y: 100, z: 0}
const connectorDirection: Vector = connector.direction;  // {x: 0, y: 1, z: 0}
const gapOffset: number = connection.gap ?? 0;  // Gap value or default 0
```

Variables have:
- **Name**: The identifier you use in code (`kit`, `types`, `gapOffset`)
- **Value**: The data currently stored (the actual kit object, the array, the number)
- **Type**: The kind of data (`Kit`, `Type[]`, `number`)
- **Scope**: Where in the program the name is valid (inside a function, globally)
- **Lifetime**: How long the storage exists (until function returns, until program ends)

**Why variables were invented**

Without names, programmers would reference raw memory addresses. semio would look like:
```
// Nightmare version without variables
LOAD [0x7fff5fbff8ac]  // What is this? Who knows!
ADD [0x7fff5fbff8b4]
STORE [0x7fff5fbff8bc]
```

With variables:
```typescript
// Clear, understandable version
const totalHeight = baseHeight + extensionHeight;
```

**What it enables**

- Readable, maintainable code—`piece.plane.origin.x` is self-documenting
- Symbolic computation (work with concepts like `Type`, not addresses)
- Automatic memory management in TypeScript/Python
- Compiler catches undefined names (`tyep` instead of `type` → error)
- Self-documenting code through good naming
- Abstraction—hide implementation details

**What it limits**

- Names can be misleading (nothing enforces that `currentDesign` is actually current)
- Name collisions in large programs (solved by modules)
- Memory still has limits regardless of naming
- Understanding scope and lifetime requires learning
- Performance implications of variable lookup (usually negligible)

---

#### 2.5 Types: Categories of Data

**Plain explanation**

In semio, you treat a Point differently from a Vector. You don't connect a Point to another Point—you connect Connectors. You don't place a Connector in space—you place a Piece. Types are the computer's way of categorizing data so it knows what operations make sense.

A `Piece` type can be placed, connected, scaled. A `Type` type defines connectors and models. A `Connection` type links two pieces with gap/shift/rise parameters. Types define what you can do with data.

**Technical explanation**

A type specifies:
- **Representation**: How data is stored in memory
- **Operations**: What can be done with values of this type
- **Constraints**: What values are valid

**semio's type hierarchy** (from `js/semio/semio.ts`):

```typescript
// Primitive types (built into the language)
number    // 64-bit floating point: 3.14159, -100, 0
string    // Text: "Wall Panel", "Left Connector"
boolean   // true or false

// semio's domain types (defined by semio)
export type Guid = string;  // Type alias

// Value types (small, immutable data)
export type Point = { x: number; y: number; z: number };
export type Vector = { x: number; y: number; z: number };
export type Plane = { origin: Point; xAxis: Vector; yAxis: Vector };

// Entity types (complex objects with identity)
export type Connector = {
  guid: Guid;
  id: string;
  name?: string;
  point: Point;
  direction: Vector;
  t?: number;
  mandatory?: boolean;
  interface?: InterfaceId;
  description?: string;
  attributes?: Attribute[];
};

export type Type = {
  guid: Guid;
  name: string;
  variant?: string;
  models?: Model[];
  connectors?: Connector[];
  // ... many more fields
};

export type Piece = {
  guid: Guid;
  id: string;
  name?: string;
  type?: TypeId;      // References a Type
  design?: DesignId;  // Or references a Design (sub-design)
  plane?: Plane;
  center?: Point;
  scale?: number;
  // ... more fields
};
```

Type systems vary:
- **Static typing** (TypeScript, C#): Types checked at compile time—errors caught before running
- **Dynamic typing** (Python, JavaScript): Types checked at runtime—more flexible, later errors

semio uses TypeScript's static typing to catch errors early:
```typescript
// TypeScript catches this at compile time:
const piece: Piece = { guid: "123", id: "p1" };
piece.plane.origin.x = 100;  // ERROR: piece.plane might be undefined!

// Correct version:
if (piece.plane) {
  piece.plane.origin.x = 100;  // OK: we checked first
}
```

**Why types exist**

Without types, all data is just bytes. Adding a `Point` to a `Connection` produces nonsense. Types:
- Prevent meaningless operations (can't add Piece + Number)
- Document what data represents (function takes a `Kit`, returns a `Design`)
- Enable optimization (knowing size and operations)
- Catch errors before runtime

**What it enables**

- Early detection of bugs (TypeScript catches "undefined" errors)
- Better documentation (function signatures show types)
- IDE autocomplete (VS Code knows `piece.` has `.plane`, `.type`, etc.)
- Compiler optimization (knowing types enables efficient code)
- Clear contracts between parts of code

**What it limits**

- Type annotations add verbosity (`const kit: Kit = ...` vs just `const kit = ...`)
- Some valid programs are rejected by type checkers
- Type systems add language complexity (generics, unions, intersections)
- Converting between types requires explicit code
- Dynamic programs may need type workarounds

---

#### 2.6 Functions: Reusable Blocks of Logic

**Plain explanation**

Imagine calculating piece placement every time two components connect. The math is complex—coordinate transformations, rotations, offsets. Instead of writing this calculation every time, you create a function called `computeConnectedPlane` and use it wherever needed.

Functions are named shortcuts for blocks of code. You define the code once, give it a name, and then "call" that name whenever you want to execute that code. semio has hundreds of functions for diffing, applying changes, validating kits, and transforming geometry.

**Technical explanation**

A function is a reusable block of code with:
- **Name**: How you refer to it
- **Parameters**: Input values (optional)
- **Body**: The code that executes
- **Return value**: Output produced (optional)

**semio in context**: Functions from `js/semio/semio.ts`:

```typescript
// FUNCTION 1: Generate a unique identifier
// =========================================
// Name: guid
// Parameters: none
// Returns: a unique string like "01961c12-f8c1-7a5a-b5c8-..."
// Purpose: Every entity in semio (types, pieces, connections) needs a unique ID
export const guid = () => uuidv7();

// FUNCTION 2: Check if two values are deeply equal
// =================================================
// Name: deepEqual
// Parameters: a (any value), b (any value)
// Returns: true if a equals b, false otherwise
// Purpose: Diffing needs to know if two objects are the same
export const deepEqual = (a: any, b: any): boolean => {
  // Quick check: if they're the exact same reference, they're equal
  if (a === b) return true;
  
  // Handle null/undefined cases
  if (a == null && b == null) return true;
  if (a == null || b == null) return false;
  
  // Different types can't be equal
  if (typeof a !== typeof b) return false;
  
  // ... more comparison logic for objects and arrays
  return false;
};

// FUNCTION 3: Compute the difference between two 3D points
// =========================================================
// Name: getPointDiff
// Parameters: before (Point), after (Point)
// Returns: PointDiff - how much each coordinate changed
// Purpose: Track what changed when a piece moves
export const getPointDiff = (before: Point, after: Point): PointDiff => {
  return {
    x: after.x - before.x,  // How much X changed
    y: after.y - before.y,  // How much Y changed
    z: after.z - before.z,  // How much Z changed
  };
};

// FUNCTION 4: Apply a diff to update a point
// ==========================================
// Name: applyPointDiff
// Parameters: base (Point to update), diff (changes to apply)
// Returns: new Point with changes applied
// Purpose: Undo/redo and sync use diffs to update positions
export const applyPointDiff = (base: Point, diff: PointDiff): Point => {
  // Use ?? 0 to default to 0 if diff value is undefined
  const x = diff.x ?? 0;
  const y = diff.y ?? 0;
  const z = diff.z ?? 0;
  
  // Return a NEW point (don't modify the original - immutability!)
  return {
    x: base.x + x,  // Add the X change to base X
    y: base.y + y,  // Add the Y change to base Y
    z: base.z + z,  // Add the Z change to base Z
  };
};

// FUNCTION 5: Generate unique names like "Wall", "Wall 2", "Wall 3"
// =================================================================
// Name: generateUniqueName
// Parameters: baseName, existingNames array, separator (default " ")
// Returns: a name that doesn't exist in existingNames
// Purpose: When creating new types/pieces, avoid duplicate names
export const generateUniqueName = (
  baseName: string,           // The name we want to use: "Wall"
  existingNames: string[],    // Names already taken: ["Wall", "Column"]
  separator: string = " "     // What goes before number: " " → "Wall 2"
): string => {
  // If baseName isn't taken, just use it
  if (!existingNames.includes(baseName)) return baseName;
  
  // Otherwise, try "Wall 2", "Wall 3", etc.
  let counter = 2;
  while (existingNames.includes(`${baseName}${separator}${counter}`)) {
    counter++;  // Keep incrementing until we find an unused name
  }
  return `${baseName}${separator}${counter}`;
};
```

When a function is called:
1. Arguments are passed (`baseName="Wall"`, `existingNames=["Wall"]`)
2. A new scope is created for the function
3. The body executes
4. The return value is sent back (`"Wall 2"`)
5. Execution continues after the call

**Why functions were invented**

Early programs were long sequences of instructions with jumps (GOTOs). This created "spaghetti code"—impossible to follow. Functions introduced:
- **Modularity**: Break semio into small pieces (diffing, validation, transformation)
- **Reuse**: `applyPointDiff` works for any Point
- **Abstraction**: Hide complex diff logic behind simple names
- **Testing**: Test `getPointDiff` independently

**What it enables**

- DRY (Don't Repeat Yourself)—diff logic written once, used everywhere
- Understandable structure (`validateKit` clearly validates a kit)
- Independent development (different developers work on different functions)
- Testing in isolation (unit tests for each function)
- Libraries of pre-written functions (Three.js, Zod, Y.js)
- Recursive solutions (functions calling themselves for tree traversal)

**What it limits**

- Function call overhead (negligible for most uses)
- Need to design good interfaces (what parameters, what return value?)
- Side effects can make functions unpredictable
- Deep call stacks use memory (recursion on deep hierarchies)
- Debugging through many function layers

---

#### 2.7 Control Flow: Making Decisions and Loops

**Plain explanation**

A program that just executes line by line would be useless for semio. We need to make decisions ("if this connector is mandatory and not connected, report an error") and repeat actions ("for each piece in the design, compute its position").

Control flow statements let programs branch (go one way or another) and loop (repeat until done).

**Technical explanation**

**semio in context**: Control flow throughout the codebase:

```typescript
// EXAMPLE 1: IF STATEMENTS - Making Decisions
// ============================================
// When computing a diff, we only record values that actually changed
export const getPointDiff = (before: Point, after: Point): PointDiff => {
  const diff: PointDiff = {};  // Start with empty diff
  
  // IF the X coordinate changed, record the difference
  if (before.x !== after.x) diff.x = after.x - before.x;
  
  // IF the Y coordinate changed, record the difference  
  if (before.y !== after.y) diff.y = after.y - before.y;
  
  // IF the Z coordinate changed, record the difference
  if (before.z !== after.z) diff.z = after.z - before.z;
  
  // If nothing changed, diff will be {} (empty object)
  // This saves space - we don't store unchanged values
  return diff;
};

// EXAMPLE 2: LOOPS - Processing Collections
// =========================================
// When comparing two lists of attributes, we need to check every item
const getAttributesDiff = (before: Attribute[], after: Attribute[]): AttributesDiff => {
  // Create Sets for fast lookup (O(1) instead of O(n) for each check)
  const beforeGuids = new Set(before.map((a) => a.guid));
  const afterGuids = new Set(after.map((a) => a.guid));
  
  // LOOP through "before" to find REMOVED items
  // (items that existed before but don't exist after)
  const removed = before
    .filter((a) => !afterGuids.has(a.guid))  // Keep if NOT in after
    .map((a) => ({ guid: a.guid }));         // Just need the ID
  
  // LOOP through "after" to find ADDED items
  // (items that exist now but didn't exist before)
  const added = after.filter((a) => !beforeGuids.has(a.guid));
  
  // LOOP through "after" to find UPDATED items
  // (items that exist in both, but with different values)
  const updated = after
    .filter((a) => beforeGuids.has(a.guid))  // Must exist in both
    .map((a) => ({
      attribute: { guid: a.guid },
      // Compute what changed for this specific attribute
      diff: getAttributeDiff(
        before.find((b) => b.guid === a.guid)!,  // Find matching "before"
        a  // Current "after" value
      )
    }))
    .filter((u) => Object.keys(u.diff).length > 0);  // Only if something changed
    
  // Return all three categories: what was removed, added, and updated
  return { removed, added, updated };
};
```

**Control flow keywords**:
- `if/else if/else`: Conditional execution (run this code only IF condition is true)
- `while`: Loop while condition is true (keep doing this WHILE something is true)
- `for`: Iterate over a sequence (do this FOR each item in a list)
- `break`: Exit loop early (stop the loop NOW)
- `continue`: Skip to next iteration (skip this item, continue with next)
- `return`: Exit function and return value (we're done, here's the answer)

At the machine level, control flow uses comparison instructions and jumps to change which instruction executes next.

**Why control flow exists**

Static sequences can only solve trivial problems. semio needs:
- Reacting to conditions (is this piece placed? is this connector mandatory?)
- Processing collections (all pieces, all connections, all types)
- Repeating until conditions are met (keep generating unique names until one is unique)
- Handling multiple cases (different validation for different entity types)

**What it enables**

- Programs that respond to input
- Processing any number of pieces and connections
- Complex algorithms with branching logic
- Interactive applications (Sketchpad responds to every click)
- Error handling and recovery
- Real-world problem solving

**What it limits**

- Complex control flow is hard to follow
- Deeply nested conditionals become unreadable
- Loops can run forever if conditions are wrong (infinite loops)
- Testing all branches is difficult
- Performance depends on which branches execute

---

#### 2.8 Errors: When Things Go Wrong

**Plain explanation**

Mistakes happen. You might try to load a kit file that doesn't exist, reference a type that was deleted, or connect two incompatible connectors. These are errors—situations where the program can't do what you asked.

Some errors are caught before the program runs (syntax errors, type errors). Others happen while running (runtime errors—like trying to load a corrupted kit file). semio anticipates errors through validation systems and handles them gracefully.

**Technical explanation**

**semio in context**: Error categories across the codebase:

**1. Schema validation errors (Zod)**:
```typescript
// js/semio/semio.ts uses Zod for runtime validation
const PointSchema = z.object({
  x: z.number(),
  y: z.number(), 
  z: z.number(),
});

// If you pass invalid data:
const invalid = { x: "not a number", y: 0, z: 0 };
const result = PointSchema.safeParse(invalid);
// result.success === false
// result.error contains detailed validation errors
```

**2. Domain validation errors (validateKit)**:
```typescript
// Validation produces Problem objects
interface Problem {
  constraintId: string;       // e.g., "type-name-unique"
  severity: "error" | "warning";
  message: string;            // Human-readable description
  location: SemioDomainLocation;
  relatedGuids?: Guid[];
  fixes: Fix[];               // Suggested fixes as KitDiffs
}

// Usage
const result = validateKit(kit);
if (result.problems.some(p => p.severity === "error")) {
  // Handle validation errors
}
```

**3. Runtime errors with try/catch**:
```typescript
// Python engine handles file operations safely
try:
    with open(kit_path, "r") as f:
        kit_data = json.load(f)
except FileNotFoundError:
    raise KitNotFoundError(f"Kit file not found: {kit_path}")
except json.JSONDecodeError as e:
    raise KitParseError(f"Invalid JSON in kit file: {e}")
```

**4. Type errors caught by TypeScript/C# compilers**:
```typescript
// TypeScript catches type errors at compile time
const piece: Piece = { id: "piece-1", type: { guid: "..." } };
piece.id = 42; // Error: Type 'number' is not assignable to type 'string'
```

**Why error handling matters**

semio interacts with unpredictable inputs:
- Kit files may be corrupted or from old versions
- Users may delete types that pieces reference
- Networks may fail during kit sync
- Rhino/Grasshopper may pass invalid geometry
- Other systems may send malformed JSON

Without error handling, any problem crashes the entire application. With error handling, semio can recover, suggest fixes, or fail gracefully.

**What it enables**

- Robust applications that don't crash on invalid kits
- Meaningful error messages ("Type 'Wall' not found in kit")
- Quick Fixes in VS Code that apply `KitDiff` repairs
- Validation reports before saving kits
- Graceful degradation (display what's valid, highlight errors)
- Defensive programming against malformed external data

**What it limits**

- Error handling adds code complexity (try/catch everywhere)
- Errors can be swallowed accidentally (empty catch blocks)
- Knowing which errors to catch requires domain experience
- Some errors can't be recovered from (corrupted binary data)
- Error handling paths are often under-tested

---

### Chapter 3: How Data Actually Works

#### 3.1 What Data Is

**Plain explanation**

Data is information—anything a computer works with. In semio, data includes architectural designs, type definitions, piece placements, connector positions, and connection parameters. Every kit you create is data. Every piece, every connection, every attribute—all data.

To a computer, all data is ultimately numbers. The Point (100, 50, 0) is three numbers. The Guid "01961c12-f..." is a number encoded as text. The color "red" becomes (255, 0, 0). The computer doesn't know that (100, 50, 0) represents a connector position; it just processes the numbers.

**Technical explanation**

**semio in context**: Core data types from `js/semio/semio.ts`:

**Primitive data** (single values):
```typescript
// Guid: Universally unique identifier
type Guid = string;  // "01961c12-f8c1-7a5a-b5c8-a12b3c4d5e6f"

// Numbers for geometry
const point: Point = { x: 100, y: 50, z: 0 };

// Booleans for flags
interface Piece {
  isHidden?: boolean;  // true or false
  isLocked?: boolean;
}

// Strings for names
interface Type {
  name: string;       // "Wall", "Column", "Beam"
  description?: string;
}
```

**Composite data** (combinations):
```typescript
// Objects - named fields of different types
interface Connector {
  guid: Guid;
  name?: string;
  point: Point;       // Nested object
  direction: Vector;  // Nested object
  t?: number;
  mandatory?: boolean;
}

// Arrays - ordered sequences
interface Type {
  connectors?: Connector[];  // Array of objects
  models?: Model[];
  props?: Prop[];
}

// Maps - key-value pairs (in Y.js stores)
const typesByGuid: Map<Guid, Type>;
const piecesByGuid: Map<Guid, Piece>;
```

**Complex structures**:
```typescript
// Tree structures (types can have parent types)
interface Type {
  name: string;
  parent?: TypeId;    // Reference to parent type
  // Children derived by querying parent reference
}

// Graph structures (pieces connected via connections)
interface Connection {
  connected: Side;    // One piece-connector pair
  connecting: Side;   // Another piece-connector pair
}
```

Data has:
- **Representation**: How bits encode meaning (Point uses IEEE 754 floats)
- **Type**: What operations are valid (can add Points, can't add Guids)
- **Structure**: How parts relate (Piece contains Plane, Plane contains Point)
- **Size**: How many bytes
- **Interpretation**: What the bits mean (same bits can be int, float, or garbage)

**Why we care about data**

Programs exist to transform data. Input data → processing → output data. Understanding data representation helps you:

- Choose appropriate types
- Avoid precision errors
- Optimize memory usage
- Design good data models
- Debug mysterious behaviors

**What it enables**

- Modeling any information digitally
- Perfect copies and transmission
- Automated processing at scale
- Searchable, sortable, filterable records
- Persistence across time
- Sharing across networks

**What it limits**

- Continuous values must be approximated
- Encoding/decoding adds complexity
- Interpretation requires context (what do these bytes mean?)
- Size limits constrain what can be stored
- Data without structure is hard to use
- Privacy and security concerns for personal data

---

#### 3.2 Data Structures: Organizing Information

**Plain explanation**

Data structures are ways of organizing data. A list of pieces is different from a map of pieces by GUID. A hierarchy of types (parent → children) is different from a graph of connections. Each structure is suited to different tasks.

Choosing the right structure is like choosing the right container: an array for ordered sequences, a Map for fast lookups, a tree for hierarchies. semio uses all of these: arrays for connectors within a type, maps for quick GUID lookups, trees for type hierarchies, and graphs for piece connections.

**Technical explanation**

**semio in context**: Data structures throughout the codebase:

**Array/List**: Ordered sequence, indexed by position

```typescript
// ============================================================
// EXAMPLE 1: ARRAYS - Ordered Collections of Items
// ============================================================
// An array is like a numbered list where each item has a position.
// Position 0 is first, position 1 is second, etc.
// Arrays are perfect when ORDER matters (first connector, second connector...)

// TypeScript arrays for collections within entities
interface Kit {
  types?: Type[];        // Array of types - each type at position 0, 1, 2...
  designs?: Design[];    // Array of designs - maintains insertion order
  qualities?: Quality[]; // Array of qualities - can be empty []
  files?: File[];       // The "?" means optional - might not exist
}

// Python uses "list" for the same concept
class Kit(BaseModel):
    types: list[Type] = []    # Empty list is the default value
    designs: list[Design] = []  # list[Type] means "a list containing Type items"
```

**Map/Dictionary**: Key-value pairs for fast lookup

```typescript
// ============================================================
// EXAMPLE 2: MAPS - Find Items Instantly by Key
// ============================================================
// A Map is like a dictionary: look up a word (key), get its definition (value).
// Instead of searching through 1000 types, you can find any type INSTANTLY by its GUID.
// Use Maps when you need to look up items by a unique identifier.

// TypeScript Map for GUID-based lookups
const typesByGuid: Map<Guid, Type> = new Map();  // Create empty Map (key=Guid, value=Type)
kit.types?.forEach(t => typesByGuid.set(t.guid, t));  // Add each type with its GUID as key

// To find a type: typesByGuid.get("abc-123") → returns the Type or undefined

// Python uses "dict" (dictionary) for the same concept
types_by_guid: dict[str, Type] = {t.guid: t for t in kit.types}
# This is "dict comprehension" - creates key:value pairs from a loop

// Y.js Map for reactive state (automatically syncs between users!)
const yTypes = yDoc.getMap<YType>("types");  // Get or create a Map named "types"
yTypes.set(type.guid, yType);  // Add a type - all connected users see this change
```

**Set**: Unique values, fast membership check

```typescript
// ============================================================
// EXAMPLE 3: SETS - Track Unique Items (No Duplicates!)
// ============================================================
// A Set is like a guest list: you're either on it or not.
// Adding the same person twice doesn't create a duplicate.
// Use Sets when you need to track "which items" without caring about order.

// Set for tracking selected items in the UI
const selection = new Set<Guid>();  // Create empty Set of GUIDs
selection.add(piece.guid);          // Add a piece to selection
selection.add(piece.guid);          // Adding same piece again - no effect (already in Set)
if (selection.has(piece.guid)) {    // Check if piece is selected - INSTANT (O(1))
  /* piece is selected */
}

// Set for computing diffs (what changed between before and after?)
const beforeGuids = new Set(before.map(a => a.guid));  // Set of all "before" GUIDs
const afterGuids = new Set(after.map(a => a.guid));    // Set of all "after" GUIDs
const removed = before.filter(a => !afterGuids.has(a.guid));  // Items in before but NOT in after
// The "!" means NOT - so we keep items where afterGuids does NOT have their GUID
```

**Tree**: Hierarchical structure

```typescript
// ============================================================
// EXAMPLE 4: TREES - Parent-Child Hierarchies
// ============================================================
// A tree is like a family tree or folder structure.
// Each item has ONE parent (except the root) and can have MANY children.
// Use trees when data has natural "contains" or "is-a-kind-of" relationships.

// Type hierarchy (parent-child via reference)
interface Type {
  guid: Guid;           // Unique identifier for this type
  name: string;         // Name like "Capsule" or "Wall"
  parent?: TypeId;      // OPTIONAL reference to parent type
  // If parent exists, this type INHERITS from its parent
  // Example: "External Wall" parent is "Wall", "Wall" parent is "Structure"
}

// To get all subtypes of a type:
const getSubtypes = (types: Type[], parentGuid: Guid): Type[] => {
  // filter() keeps only items where the condition is true
  return types.filter(t => t.parent?.guid === parentGuid);
  // "t.parent?.guid" safely accesses guid - returns undefined if parent doesn't exist
  // "===" checks if the GUID matches exactly
};

// Layer hierarchy uses path (implicit tree - no parent reference needed!)
interface Layer {
  path: string;  // The "/" creates the hierarchy implicitly
  // "Structure"                → Top level layer
  // "Structure/Walls"          → Child of "Structure"
  // "Structure/Walls/External" → Child of "Structure/Walls"
}
```

**Graph**: Pieces connected by connections

```typescript
// ============================================================
// EXAMPLE 5: GRAPHS - Networks of Connected Things
// ============================================================
// A graph is like a social network: nodes (people) connected by edges (friendships).
// Unlike trees, graphs can have CYCLES (A connects to B connects to C connects to A).
// semio designs ARE graphs: pieces (nodes) connected by connections (edges).

// Design is an undirected graph (connections work both ways)
interface Design {
  pieces?: Piece[];         // Nodes of the graph
  connections?: Connection[]; // Edges connecting pieces
}

// Connection links two pieces (undirected means order doesn't matter)
interface Connection {
  connected: Side;   // { piece: PieceId, connector: ConnectorId } - one end
  connecting: Side;  // { piece: PieceId, connector: ConnectorId } - other end
  // The connection joins the "connected" connector to the "connecting" connector
}

// Graph traversal to find all pieces connected together (a "component")
// This uses BFS (Breadth-First Search) - explores neighbors before going deeper
const findConnectedPieces = (startGuid: Guid, connections: Connection[]): Set<Guid> => {
  const visited = new Set<Guid>();  // Track which pieces we've already seen
  const queue = [startGuid];        // Queue of pieces to explore (FIFO: first in, first out)
  
  while (queue.length > 0) {        // Keep going until queue is empty
    const current = queue.shift()!; // Remove and get first item ("!" asserts it exists)
    
    if (visited.has(current)) continue;  // Skip if already visited (prevents infinite loops!)
    visited.add(current);           // Mark as visited
    
    // Find all adjacent pieces (connected to current piece)
    connections.forEach(c => {
      // Check both directions since graph is undirected
      if (c.connected.piece.guid === current) 
        queue.push(c.connecting.piece.guid);  // Add neighbor to explore later
      if (c.connecting.piece.guid === current)
        queue.push(c.connected.piece.guid);   // Add other neighbor too
    });
  }
  return visited;  // All pieces reachable from startGuid
};
```
```

**Why structures matter**

Different operations have different costs:
- Array search: O(n) - check each piece
- Map lookup: O(1) - instant by GUID
- Tree traversal: O(n) - visit all nodes
- Graph pathfinding: O(V + E) - depends on vertices and edges

Big-O notation describes how operations scale. When a kit has 1000 types, finding one by GUID via Map is instant; searching an array checks up to 1000 items.

**What it enables**

- Fast GUID lookups via Map (essential for real-time UI)
- Efficient diff computation via Set operations
- Natural hierarchy representation for types and layers
- Graph algorithms for placement computation
- Optimized rendering (only visible pieces)

**What it limits**

- Wrong structure choice causes poor performance
- Memory overhead for maintaining indexes
- Keeping structures in sync (Map must update when Array changes)
- Trade-offs between operations (arrays maintain order, maps don't)

---

#### 3.3 Objects: Grouping Data and Behavior

**Plain explanation**

In semio, things have properties and capabilities. A Type has a name, connectors, and models (properties) and can be validated, diffed, and rendered (capabilities). An object bundles data (properties) with functions that operate on that data (methods).

Objects let you think about your program in terms of things that interact: Pieces connect via Connectors, Designs contain Pieces, Kits bundle Types and Designs. Each is a coherent unit with its own data and operations.

**Technical explanation**

**semio in context**: Object patterns across languages:

**TypeScript (functional style with interfaces)**:

```typescript
// ============================================================
// EXAMPLE 1: OBJECTS IN TYPESCRIPT - Data + Functions Separated
// ============================================================
// TypeScript uses a "functional" style: data is described by interfaces,
// and functions are written separately to operate on that data.
// This makes code easy to test and reason about.

// js/semio/semio.ts - Data as interfaces (describes SHAPE of data)
interface Piece {
  guid: Guid;           // Every piece has a unique identifier
  name?: string;        // Optional name (? means might not exist)
  type?: TypeId;        // Reference to which Type this piece uses
  plane?: Plane;        // Position and orientation in 3D space
  center?: Point;       // Center point for UI display
  isHidden?: boolean;   // Is this piece hidden from view?
  isLocked?: boolean;   // Is this piece locked from editing?
}

// Behavior as standalone functions (NOT attached to the data)
const getPieceDiff = (before: Piece, after: Piece): PieceDiff => { 
  // Compares two pieces and returns what changed
  // ... implementation ...
};
const applyPieceDiff = (base: Piece, diff: PieceDiff): Piece => { 
  // Takes a piece and changes, returns new piece with changes applied
  // ... implementation ...
};
const areSamePieceId = (a: PieceId, b: PieceId): boolean => a.guid === b.guid;
// Simple comparison: are these the same piece? Just check if GUIDs match
```

**Python (class-based with Pydantic)**:

```python
# ============================================================
# EXAMPLE 2: OBJECTS IN PYTHON - Data + Methods Together
# ============================================================
# Python uses classes that combine data (fields) and behavior (methods).
# Pydantic adds automatic validation - if you give wrong data types, it errors!

# py/engine/engine.py - Classes with validation
class Piece(BaseModel):  # Inherits from Pydantic's BaseModel
    # These are the data fields with type annotations
    guid: str = Field(default_factory=lambda: str(uuid7()))  # Auto-generates GUID if not provided
    name: str | None = None        # String or None (Python's optional syntax)
    type: TypeId | None = None     # Reference to type definition
    plane: Plane | None = None     # 3D position/orientation
    center: Point | None = None    # Center point
    is_hidden: bool | None = None  # Python uses snake_case (is_hidden not isHidden)
    is_locked: bool | None = None
    
    # This is a METHOD - a function that belongs to the class
    def to_placed(self, kit: "Kit") -> "PlacedPiece":
        """Compute the placed representation of this piece."""
        # 'self' refers to THIS piece instance
        # The method can access self.guid, self.plane, etc.
        ...
```

**C# (traditional OOP)**:

```csharp
// ============================================================
// EXAMPLE 3: OBJECTS IN C# - Full Object-Oriented Programming
// ============================================================
// C# uses traditional OOP with properties (data) and methods (behavior).
// Properties have get/set accessors for controlled access.

// net/Semio/Semio.cs - Full OOP with properties
public class Piece : ISerializable  // Piece implements ISerializable interface
{
    // Properties with get/set accessors
    public Guid Guid { get; set; }      // Can be read and written
    public string? Name { get; set; }    // "?" means nullable (might be null)
    public TypeId? Type { get; set; }
    public Plane? Plane { get; set; }
    public Point? Center { get; set; }
    public bool? IsHidden { get; set; }  // C# uses PascalCase (IsHidden not isHidden)
    public bool? IsLocked { get; set; }
    
    // Method that computes something using this object's data
    public Plane ComputePlane(Kit kit, Design design)
    {
        // 'this' refers to the current Piece instance
        // Method can access this.Guid, this.Plane, etc.
        ...
    }
}
```

An object combines:
- **State**: Data stored in fields (guid, name, plane)
- **Behavior**: Methods that operate on state (ComputePlane, to_placed)
- **Identity**: A way to distinguish instances (guid serves this purpose)

**Object-oriented principles in semio**:

```typescript
// ============================================================
// EXAMPLE 4: THREE KEY OOP PRINCIPLES
// ============================================================
// Object-Oriented Programming has three core ideas:
// 1. INHERITANCE: Child objects get features from parent objects
// 2. POLYMORPHISM: Same function works on different types
// 3. ENCAPSULATION: Hide complex internals, show simple interface

// ----- INHERITANCE -----
// A Type can extend (inherit from) a parent Type
interface Type {
  parent?: TypeId;  // Reference to parent type (optional)
  // If a type has a parent, it INHERITS:
  //   - Connectors from parent
  //   - Props from parent
  //   - Other characteristics
  // Example: "External Wall" inherits from "Wall" inherits from "Structure"
}

// ----- POLYMORPHISM -----
// "Poly" = many, "morph" = forms
// The SAME function (validateKit) works on DIFFERENT entity types
const validateKit = (kit: Kit): ValidationResult => {
  const problems: Problem[] = [];  // Collect all problems found
  
  // validateType and validateDesign return the SAME type of result
  // even though they validate completely different things
  kit.types?.forEach(t => problems.push(...validateType(t)));
  // ...validateType(t) spreads the returned array into problems
  kit.designs?.forEach(d => problems.push(...validateDesign(d)));
  
  return { problems };  // Return all problems in one result
};

// ----- ENCAPSULATION -----
// Hide COMPLEX internals (Y.js), expose SIMPLE interface
class KitStore {
  // "private" means ONLY this class can access these
  private readonly yDoc: Y.Doc;           // Complex Y.js document
  private readonly yTypes: Y.Array<YType>; // Complex Y.js array
  
  // "readonly" means these can't be reassigned after construction
  
  // PUBLIC API - what other code sees and uses
  // Users don't need to know about Y.js!
  addType(type: Type): void { ... }    // Add a type to the kit
  removeType(guid: Guid): void { ... } // Remove a type by GUID
  snapshot(): Kit { ... }              // Get current kit state
  
  // The complex Y.js sync, persistence, conflict resolution
  // is all HIDDEN inside these simple methods
}
```

**Why objects were invented**

As programs grew larger, managing global functions and data became chaotic. Objects provide:

- **Organization**: Piece data and piece operations together
- **Encapsulation**: Hide Y.js complexity behind KitStore API
- **Modeling**: Type, Piece, Connection map to real-world concepts
- **Reuse**: Base stores extended by app-specific stores

**What it enables**

- Intuitive modeling of architectural domains
- Clean separation (Store handles persistence, App handles UI)
- Reusable validation across entity types
- Extensible systems (KitDiffAppStore extends AppStore)
- GUI systems (React components as objects with props/state)

**What it limits**

- Overhead for object creation (mitigated by caching)
- Class hierarchies can become complex
- Tight coupling if not designed carefully
- Not all problems fit the object paradigm
- Can be over-engineered for simple transforms

---

#### 3.4 State: Things Change

**Plain explanation**

State is the current condition of your program—all the values at a moment in time. In semio, state includes which kit is open, which pieces are selected, what's being dragged, undo history, camera position. When you select a piece, you change the state.

Managing state is one of programming's hardest problems. When many things can change (Sketchpad UI, Y.js sync, Grasshopper updates), and changes affect other things, bugs emerge from unexpected interactions.

**Technical explanation**

**semio in context**: State at multiple levels:

**Local state**: Variables within a function

```typescript
// ============================================================
// EXAMPLE 1: LOCAL STATE - Safe and Temporary
// ============================================================
// Local state exists ONLY while a function runs.
// When the function ends, the variable disappears.
// This is the SAFEST type of state - no one else can mess with it.

const computePiecePlane = (piece: Piece, connections: Connection[]): Plane => {
  let currentPlane = piece.plane;  // Create LOCAL variable
  // "let" means the value CAN change during the function
  
  // We can safely modify currentPlane here
  // because ONLY this function can see it
  // ... computation logic ...
  
  return currentPlane;  // Return the result, variable is gone after this
};
```

**Object/Store state**: Fields within stores

```typescript
// ============================================================
// EXAMPLE 2: STORE STATE - Persisted and Shared
// ============================================================
// Store state lives in a class and persists across function calls.
// Multiple parts of the app can read (and potentially write) this state.
// This needs careful management to avoid conflicts!

// KitStore - persists with Y.js document
class KitStore {
  private readonly yTypes: Y.Array<YType>;  // REACTIVE state
  // "Reactive" means when this changes, the UI automatically updates!
  // Y.js handles syncing this across all connected users
  
  private snapshot: Kit | null = null;      // CACHED state
  // "Cached" means we store a computed result to avoid recalculating
  // "null" means it might not exist yet
}

// Design app state - persists in XState machine
interface DesignAppState {
  selection: { pieces: Guid[]; connections: Guid[] };  // What's currently selected
  hover: { pieces: Guid[]; connectors: Guid[] };       // What's under the mouse
  camera: Camera;      // Where the 3D view is looking
  activeTool: ToolKind;  // Which tool is active (select, connect, etc.)
}
```

**Global/Application state**: XState machine context

```typescript
// ============================================================
// EXAMPLE 3: GLOBAL STATE - Application-Wide Truth
// ============================================================
// Global state is the "single source of truth" for the entire app.
// XState is a state machine library that makes state changes PREDICTABLE:
// - State can only change through defined EVENTS
// - Each event triggers specific ACTIONS
// - The current state determines what events are valid

// Sketchpad.tsx - centralized state machine
const sketchpadMachine = createMachine({
  // CONTEXT holds all the actual data values
  context: {
    theme: "system",      // "system", "light", or "dark"
    language: "en",       // "en", "de", etc.
    expertise: "normal",  // "beginner", "normal", "expert"
    kits: [],             // Array of all loaded kits
    homeApp: { ... },     // State for the Home screen
    kitApp: { ... },      // State for the Kit editor
    designApp: { ... },   // State for the Design editor
    typeApp: { ... },     // State for the Type editor
    // ...
  },
  
  // EVENT HANDLERS - what happens when events occur
  on: {
    // When "DESIGN.SELECT_PIECE" event arrives, run "selectPiece" action
    "DESIGN.SELECT_PIECE": { actions: "selectPiece" },
    // When "SET_THEME" event arrives, run "setTheme" action
    "SET_THEME": { actions: "setTheme" },
    // This makes state changes PREDICTABLE and TRACEABLE
  }
});
```

**State management patterns in semio**:

```typescript
// ============================================================
// EXAMPLE 4: STATE MANAGEMENT PATTERNS
// ============================================================
// semio uses different systems for different types of state:
// - XState: UI state (selection, hover, tools) - predictable transitions
// - Y.js: Collaborative data (kits) - automatic sync across users
// - Diffs: Undo/redo - minimal change tracking

// ----- XSTATE FOR UI STATE -----
const actor = useSketchpadActor();  // Get the state machine instance
actor.send({ type: "DESIGN.SELECT_PIECE", guid: piece.guid });
// "send" dispatches an event to the state machine
// The machine handles the event and updates state accordingly

// ----- Y.JS FOR COLLABORATIVE DATA -----
yTypes.observe((event) => {
  // This function runs whenever yTypes changes
  // Changes can come from:
  //   - This user's actions
  //   - Another user on another device
  //   - Server-side updates
  invalidateSnapshot();  // Clear cached data so we recalculate
});

// ----- DIFFS FOR PERSISTENCE AND UNDO -----
const diff = getKitDiff(beforeKit, afterKit);  // What changed?
undoStack.push(diff);  // Save the diff for undo
// Diffs can be INVERTED: if diff adds a piece, inverseDiff removes it
// This enables efficient undo/redo without storing full copies
```

**Why state matters**

Programs do useful work by changing state. But:
- Multiple sources modifying state cause bugs (user action + sync + undo)
- State makes testing harder (must set up correct kit state)
- Distributed state (Y.js across devices) is complex
- Debugging "why is this piece in wrong position" is difficult

**What it enables**

- Interactive applications (Sketchpad responds to every action)
- Collaborative editing (Y.js syncs state across users)
- Undo/redo functionality (diff-based state history)
- Session persistence (state saved to IndexedDB)
- Real-time validation (state changes trigger validation)

**What it limits**

- Concurrent access causes race conditions (mitigated by Y.js CRDT)
- Global state creates hidden dependencies
- Stale state bugs (using outdated snapshot)
- Memory usage for state storage

---

#### 3.5 Immutability vs Mutability

**Plain explanation**

Mutable data can be changed in place. If you add a piece to a design, the design now has more pieces. Immutable data cannot be changed—instead, you create a new design with the additional piece. The original remains unchanged.

semio uses both: Y.js internally mutates documents for efficiency, but the diff system treats data as immutable transformations. Each diff describes how to transform one immutable snapshot into another.

**Technical explanation**

**semio in context**: Mutability and immutability patterns:

**Mutable (Y.js documents)**:
```typescript
// Y.js mutates in place for CRDT sync
const yPieces = yDoc.getArray<YPiece>("pieces");
yPieces.push([newPiece]);  // Mutates the array
// Change propagates to all connected clients

// Mutable observation
yPieces.observe((event) => {
  // React to mutations from any source
});
```

**Immutable (Diff system)**:
```typescript
// Diffs treat data as immutable snapshots
const beforeKit: Kit = { ... };
const afterKit: Kit = { ...beforeKit, types: [...beforeKit.types, newType] };

// Compute diff between immutable snapshots
const diff = getKitDiff(beforeKit, afterKit);
// diff = { types: { added: [newType] } }

// Apply diff to create new immutable snapshot
const restoredKit = applyKitDiff(beforeKit, diff);
```

**Immutable patterns for React state**:
```typescript
// XState context is treated immutably
actor.send({
  type: "DESIGN.SELECT_PIECE",
  guid: piece.guid
});

// The action creates new context, doesn't mutate
actions: {
  selectPiece: assign({
    designApp: (context, event) => ({
      ...context.designApp,
      selection: {
        ...context.designApp.selection,
        pieces: [...context.designApp.selection.pieces, event.guid]
      }
    })
  })
}
```

**Immutability benefits for semio**:
- **Undo/Redo**: Store diffs, invert to undo
- **Collaboration**: Y.js CRDTs merge concurrent changes
- **Change detection**: React re-renders on new reference
- **Testing**: Snapshots are predictable

**Immutability costs**:
- **Memory**: Creating copies uses more memory
- **Performance**: Copying is slower than modifying
- **Y.js bridge**: Must sync immutable snapshots with mutable Y.js

**Why the distinction matters**

Bugs from unexpected mutation are common:
```typescript
// BAD: Mutating shared array
const deletePiece = (pieces: Piece[], guid: Guid) => {
  const index = pieces.findIndex(p => p.guid === guid);
  pieces.splice(index, 1);  // Mutates caller's array!
  return pieces;
};

// GOOD: Return new array
const deletePiece = (pieces: Piece[], guid: Guid): Piece[] => {
  return pieces.filter(p => p.guid !== guid);  // New array
};
```

**What it enables**

- Diff-based undo/redo (just store and invert diffs)
- Safe concurrent programming (Y.js CRDTs)
- Simpler debugging (each snapshot is complete)
- Efficient React change detection
- Time-travel debugging in dev tools

**What it limits**

- More memory for copies
- Performance overhead for frequent updates
- Bridge code between immutable API and mutable Y.js

---

## Part 2: Building Systems

### Chapter 4: How Software Is Organized

#### 4.1 Files: The Basic Container

**Plain explanation**

A file is a named container for data stored on disk. In semio, files hold everything: TypeScript source code, Python engine code, C# integration code, kit definitions (`.json`), 3D models (`.glb`), SQL schemas, and configuration.

Every kit is a collection of files—a `.zip` archive containing `kit.db` (SQLite database) and model files. Every component of semio lives in files organized by language and purpose.

**Technical explanation**

**semio in context**: File types throughout the codebase:

**Source code files**:
```
js/semio/semio.ts          # TypeScript domain logic (7741 lines)
py/engine/engine.py        # Python computational engine (7727 lines)
net/Semio/Semio.cs         # C# Rhino/Grasshopper integration (5734 lines)
go/repo/main.go            # Go CLI tool
```

**Configuration files**:
```
package.json               # npm workspace configuration
tsconfig.json              # TypeScript compiler settings
pyproject.toml             # Python project configuration
Semio.csproj               # .NET project file
nx.json                    # Nx monorepo configuration
```

**Schema files**:
```
sql/sqlite/schema.sql      # SQLite database schema
graphql/semio/schema.graphql  # GraphQL API schema
jsonschema/kit.json        # JSON validation schema
```

**Asset files**:
```
assets/models/*.glb        # 3D model files
assets/icons/*.svg         # UI icons
locales/en.json           # Internationalization strings
```

**Kit structure** (static kit as .zip):
```
kit.zip/
├── .semio/
│   └── kit.db             # SQLite database with all entities
├── models/
│   ├── wall.glb           # 3D model for Wall type
│   └── column.glb         # 3D model for Column type
└── images/
    └── preview.png        # Kit preview image
```

File operations in semio:
```typescript
// Reading kit from file
const kitData = await fs.readFile("metabolism.json", "utf-8");
const kit: Kit = JSON.parse(kitData);

// Writing kit to file
await fs.writeFile("metabolism.json", JSON.stringify(kit, null, 2));

// File references in Kit
interface File {
  guid: Guid;
  path: string;           // Relative path within kit
  remoteUrl?: string;     // Optional remote URL
}
```

**Why files exist**

Files provide:
- **Persistence**: Kit data survives program termination
- **Portability**: `.zip` kits can be shared, copied, versioned
- **Tooling**: Any text editor can view source code
- **Version control**: Git tracks file changes
- **Interoperability**: JSON/SQLite are universal formats

**What it enables**

- Permanent storage of kits and designs
- Version control for all code and configuration
- Distribution of kits as portable archives
- Standard formats readable by any tool
- Backup by copying file system

**What it limits**

- File system hierarchy may not match logical structure
- Large 3D models are slow to load
- Path differences between Windows (`\`) and Unix (`/`)
- Concurrent file access requires care

---

#### 4.2 Folders: Organizing Files

**Plain explanation**

Folders group related files. semio uses a carefully organized folder structure: `js/` for JavaScript/TypeScript, `py/` for Python, `net/` for .NET/C#, `go/` for Go. Within each, subfolders separate concerns: source code, tests, configuration.

This structure lets different languages coexist, enables focused development (only build what you're working on), and makes navigation intuitive.

**Technical explanation**

**semio in context**: The monorepo folder structure:

```
semio/
├── assets/                  # Shared static assets
│   ├── icons/               # UI icons
│   ├── models/              # Example 3D models
│   └── semio/               # Test fixtures
├── js/                      # JavaScript/TypeScript
│   ├── semio/               # @semio/js - core library
│   │   ├── sketchpad/       # Sketchpad app
│   │   ├── elements/        # Shared UI components
│   │   └── locales/         # i18n translations
│   ├── vscode/              # @semio/vscode extension
│   ├── desktop/             # @semio/desktop (Electron)
│   └── docs/                # @semio/docs website
├── py/                      # Python
│   └── engine/              # @semio/engine
│       ├── engine.py        # Main source
│       └── test_engine.py   # Tests
├── net/                     # .NET/C#
│   ├── Semio/               # Core library
│   ├── Semio.Grasshopper/   # Grasshopper plugin
│   └── Semio.Tests/         # Unit tests
├── go/                      # Go
│   ├── repo/                # CLI tool
│   └── mcp/                 # MCP server
├── sql/                     # SQL schemas
│   └── sqlite/
├── graphql/                 # GraphQL schemas
├── examples/                # Example kits
│   └── metabolism/          # Main demo kit
├── tickets/                 # Development tickets
│   └── 2025/01/15/          # Date-organized
└── reports/                 # Generated reports
```

Key concepts:
- **Monorepo root**: Contains configuration for entire project
- **Language folders**: Separate ecosystems (js/, py/, net/, go/)
- **Package subfolders**: Each publishable unit in its language folder
- **Feature subfolders**: Logical grouping within packages
- **Parent directory**: One level up (..)

**Why hierarchies matter**

Flat organization fails at scale. With 10 files, you can find things. With 10,000, you need structure. Hierarchies provide:

- **Context**: Location indicates purpose
- **Isolation**: Different features in different folders
- **Namespacing**: Same filename in different folders is okay
- **Navigation**: Drill down from general to specific

**What it enables**

- Organization of large projects (14+ packages in semio)
- Convention-based structure (everyone knows js/ has TypeScript)
- Module systems based on folder structure
- Isolation of concerns (engine logic separate from UI)
- Selective work (only build what you're changing)

**What it limits**

- Files can only be in one folder
- Deep hierarchies are tedious to navigate
- Restructuring requires updating import paths
- Cross-folder dependencies need explicit configuration

---

#### 4.3 Modules: Grouping Related Code

**Plain explanation**

A module is a file (or group of files) that exports specific functions, types, or values for other code to use. semio's `semio.ts` is a module exporting hundreds of types and functions. Each app (Home, Kit, Design) is a module exporting its components and hooks.

Think of modules like specialized teams. The `semio.ts` module handles domain logic, the `Sketchpad.tsx` module handles UI orchestration. Each has internal operations (private) and services for others (exported).

**Technical explanation**

**semio in context**: Module patterns across languages:

**TypeScript (js/semio/semio.ts)**:

```typescript
// ============================================================
// EXAMPLE 1: TYPESCRIPT MODULES - Export/Import System
// ============================================================
// Modules let you share code between files.
// "export" makes something available to other files
// "import" brings in something from another file

// ----- EXPORTING TYPES -----
// "export" keyword makes these available to other modules
export type Guid = string;  // Alias: Guid is just a string with a meaningful name

export interface Point {    // Interface = shape of data
  x: number;  // Required: every Point MUST have x, y, z
  y: number;
  z: number;
}

export interface Piece {
  guid: Guid;        // Required: unique identifier
  name?: string;     // Optional (?) - might not exist
  plane?: Plane;     // Optional - position in 3D space
}

// ----- EXPORTING FUNCTIONS -----
export const guid = () => uuidv7();  // Function that generates new GUID
// Other files can call: import { guid } from './semio'; const id = guid();

export const getPointDiff = (before: Point, after: Point): PointDiff => { 
  // Compare two points, return what changed
  // ... implementation ...
};

// ----- PRIVATE HELPERS (NOT EXPORTED) -----
// NO export keyword = private to this file
const normalizeVector = (v: Vector): Vector => { 
  // Only this file can use this function
  // ... implementation ...
};

// ----- IMPORTING FROM OTHER MODULES -----
import { z } from "zod";        // Import 'z' from the 'zod' validation library
import { uuidv7 } from "uuidv7"; // Import uuidv7 function for generating GUIDs
// The {} means we're importing specific named exports
```

**Python (py/engine/engine.py)**:

```python
# ============================================================
# EXAMPLE 2: PYTHON MODULES - Classes and Conventions
# ============================================================
# Python modules are files. Import by filename.
# Public/private is by CONVENTION, not enforced.

# ----- IMPORTING -----
from pydantic import BaseModel  # Import BaseModel class from pydantic library
# "from X import Y" means: from module X, get the thing named Y

# ----- EXPORTING VIA CLASS DEFINITIONS -----
class Point(BaseModel):  # This class IS the export
    x: float  # Type annotation: x must be a float (decimal number)
    y: float
    z: float
    # Pydantic validates types automatically!

class Kit(BaseModel):
    name: str                    # Required string field
    types: list[Type] = []       # Optional list, defaults to empty []
    designs: list[Design] = []   # list[Type] means "list containing Type items"

# ----- PRIVATE CONVENTION: UNDERSCORE PREFIX -----
def _normalize_vector(v: Vector) -> Vector:
    # Leading underscore _ means "private - don't use outside this file"
    # Python doesn't ENFORCE this, it's just a convention
    ...
```

**C# (net/Semio/Semio.cs)**:

```csharp
// ============================================================
// EXAMPLE 3: C# MODULES - Namespaces and Access Modifiers
// ============================================================
// C# uses namespaces to group code and access modifiers to control visibility.
// Unlike Python, C# ENFORCES visibility rules at compile time.

// Namespace groups related code (like a folder for code)
namespace Semio
{
    // ----- PUBLIC EXPORTS -----
    // "public" = anyone can use this from any file/project
    public class Kit { ... }   // Other code: new Semio.Kit()
    public class Piece { ... } // Other code: new Semio.Piece()
    
    // ----- INTERNAL: ASSEMBLY-ONLY -----
    // "internal" = only code in the SAME compiled unit (DLL) can use this
    internal static class Helpers { 
        // Other classes in Semio can use this
        // But external code (like a Grasshopper plugin) cannot
        ...
    }
    
    // ----- PRIVATE: CLASS-ONLY -----
    // Inside a class:
    private void ComputeInternal() { 
        // Only THIS class can call this method
        // Even other classes in the same namespace cannot
        ...
    }
}
```
```

**Why modules exist**

Without modules, all code exists in one namespace:
- Every name must be unique across entire codebase
- No way to hide internal implementation
- Dependencies are unclear
- Large files become unmanageable

Modules provide structure, privacy, and explicit dependencies.

**What it enables**

- Separation of concerns (`semio.ts` for domain, `Sketchpad.tsx` for UI)
- Reusable code units across apps
- Explicit imports show dependencies
- Private implementation details hidden
- Parallel development (different modules, different developers)
- Testing modules in isolation

**What it limits**

- Module boundaries require design thought
- Circular dependencies cause build errors
- Import paths must be maintained
- Overly fine-grained modules add overhead

---

#### 4.4 Packages: Collections of Modules

**Plain explanation**

A package is a collection of modules distributed together. semio has 14+ packages: `@semio/js`, `@semio/engine`, `@semio/vscode`, etc. Each is a complete unit that can be versioned, published, and used independently.

Package managers (npm, pip, NuGet) download, install, and update packages from central repositories.

**Technical explanation**

**semio in context**: Package structure across ecosystems:

**JavaScript (npm)**:

```json
// ============================================================
// EXAMPLE 1: JAVASCRIPT PACKAGE - package.json
// ============================================================
// package.json is the "identity card" of a JavaScript package.
// It tells npm: what is this package, what does it need, what does it contain.

// js/semio/package.json
{
  "name": "@semio/js",        // Package name - the @ means it's "scoped" (organized under semio)
  "version": "0.1.0",         // Version number - 0.1.0 means early development
  "type": "module",           // Use modern ES modules (import/export syntax)
  "main": "index.ts",         // Entry point - what file to load first
  
  "dependencies": {           // PRODUCTION dependencies - needed to RUN the code
    "zod": "^3.24.4",         // Validation library - ^3.24.4 means "3.24.4 or higher, but < 4.0"
    "uuidv7": "^1.0.2",       // GUID generation library
    "three": "^0.175.0"       // 3D rendering library (Three.js)
  },
  
  "devDependencies": {        // DEVELOPMENT dependencies - only needed to BUILD/TEST
    "typescript": "^5.8.3",   // TypeScript compiler - converts TS to JS
    "vitest": "^3.2.3"        // Testing framework
  }
}
```

**Python (pyproject.toml)**:

```toml
# ============================================================
# EXAMPLE 2: PYTHON PACKAGE - pyproject.toml
# ============================================================
# pyproject.toml is Python's modern package configuration file.
# Similar to package.json but uses TOML format (simpler than JSON).

# py/engine/pyproject.toml
[project]
name = "semio-engine"        # Package name on PyPI
version = "0.1.0"            # Semantic version
dependencies = [             # Required to run
    "pydantic>=2.10.6",      # Data validation - >=2.10.6 means "2.10.6 or higher"
    "fastapi>=0.115.0",      # Web framework for API
    "sqlmodel>=0.0.22",      # SQL database integration
]

[project.optional-dependencies]  # Optional extras
dev = ["pytest>=8.0", "ruff>=0.8.6"]  # Only needed for development
# Install with: pip install semio-engine[dev]
```

**C# (NuGet)**:

```xml
<!-- ============================================================ -->
<!-- EXAMPLE 3: C# PACKAGE - .csproj file                         -->
<!-- ============================================================ -->
<!-- .csproj is C#'s project file. Defines target framework and   -->
<!-- NuGet package dependencies.                                  -->

<!-- net/Semio/Semio.csproj -->
<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <TargetFramework>net48</TargetFramework>  <!-- Target .NET Framework 4.8 (for Rhino) -->
  </PropertyGroup>
  
  <ItemGroup>  <!-- NuGet package references -->
    <PackageReference Include="Newtonsoft.Json" Version="13.0.3" />
    <!-- JSON serialization library - Version is EXACT (not ^) -->
    
    <PackageReference Include="Microsoft.Data.Sqlite" Version="8.0.0" />
    <!-- SQLite database access -->
  </ItemGroup>
</Project>
```

**Monorepo workspace (npm)**:

```json
// ============================================================
// EXAMPLE 4: MONOREPO - Multiple Packages in One Repo
// ============================================================
// A monorepo contains multiple packages that can depend on each other.
// The root package.json defines "workspaces" - folders containing packages.

// package.json (root of semio repo)
{
  "name": "semio",            // Root package name
  "workspaces": [             // These folders contain packages
    "assets",                 // @semio/assets - icons, fonts, images
    "js/*",                   // All folders in js/ are packages
    "py/engine",              // Python engine is a package
    "yak"                     // Yak packager for Rhino
  ]
  // Running "npm install" at root installs dependencies for ALL workspaces
  // Packages can import each other: import { Kit } from '@semio/js'
}
```

**Why packages were invented**

Sharing code was once manual—copy files between projects. Packages provide:
- **Discovery**: npm, PyPI, NuGet to find code
- **Versioning**: Manage breaking changes
- **Dependencies**: Automatically get what packages need
- **Updates**: Simple command to get new versions

**What it enables**

- semio workspaces share code via local packages
- External libraries (Three.js, Zod) as versioned dependencies
- Semantic versioning for compatibility (`^3.24.4` = compatible with 3.x)
- Lock files for reproducible builds (`package-lock.json`)
- Publishing to npm/PyPI/NuGet for public consumption

**What it limits**

- Dependency conflicts (different packages need different versions)
- Security vulnerabilities in dependencies
- Package maintenance burden
- Breaking changes in major versions

---

#### 4.5 Libraries: Code You Reuse

**Plain explanation**

A library is code you use in your program. You call library functions; the library doesn't control your program. semio uses dozens of libraries: Three.js for 3D rendering, Zod for validation, Y.js for collaboration, XState for state machines.

Each library represents thousands of hours of specialized work that semio leverages rather than rebuilding.

**Technical explanation**

**semio in context**: Key libraries and their roles:

**Core TypeScript libraries**:

```typescript
// ============================================================
// EXAMPLE 1: KEY JAVASCRIPT LIBRARIES IN semio
// ============================================================
// Libraries are pre-built code that solves common problems.
// Instead of writing thousands of lines, you import and use them.

// js/semio/semio.ts

// ----- ZOD: Schema Validation -----
// Zod validates that data has the right shape/types at runtime
import { z } from "zod";  // Import the 'z' object from zod library

const PointSchema = z.object({  // Define what a valid Point looks like
  x: z.number(),  // x must be a number
  y: z.number(),  // y must be a number
  z: z.number(),  // z must be a number
});
// Now: PointSchema.parse(data) throws error if data is invalid

// ----- THREE.JS: 3D Graphics -----
// Three.js handles all the complex WebGL/3D math for us
import * as THREE from "three";  // Import everything from three as "THREE"

const scene = new THREE.Scene();  // Create a 3D scene (container for objects)
const geometry = new THREE.BoxGeometry(1, 1, 1);  // Create a 1x1x1 cube shape
// Three.js handles: lighting, cameras, materials, rendering, etc.

// ----- Y.JS: Real-Time Collaboration -----
// Y.js uses CRDTs (Conflict-free Replicated Data Types) for sync
import * as Y from "yjs";

const yDoc = new Y.Doc();  // Create a Y.js document
const yTypes = yDoc.getArray("types");  // Get/create array named "types"
// Changes to yTypes automatically sync to all connected users!

// ----- XSTATE: State Machines -----
// XState manages complex state transitions predictably
import { createMachine, assign } from "xstate";

const sketchpadMachine = createMachine({ 
  // Define states, events, and transitions
  // XState ensures only valid state changes happen
  ...
});
```

**Python libraries**:

```python
# ============================================================
# EXAMPLE 2: KEY PYTHON LIBRARIES IN semio
# ============================================================
# Python has excellent data science and API libraries.

# py/engine/engine.py

# ----- PYDANTIC: Data Validation -----
# Pydantic automatically validates data types
from pydantic import BaseModel, Field
# Classes inheriting BaseModel get automatic validation

# ----- FASTAPI: Web API Framework -----
# FastAPI makes building REST APIs simple and fast
from fastapi import FastAPI
# Decorators like @app.get("/kits") define API endpoints

# ----- SQLMODEL: Database ORM -----
# SQLModel combines SQLAlchemy and Pydantic for easy database access
from sqlmodel import SQLModel, Session
# Define Python classes, SQLModel creates SQL tables

# ----- GRAPHENE: GraphQL -----
# Graphene implements GraphQL in Python
import graphene
# Allows flexible querying of data
```

**C# libraries**:

```csharp
// ============================================================
// EXAMPLE 3: KEY C# LIBRARIES IN semio
// ============================================================
// C# integrates with the Rhino/Grasshopper ecosystem.

// net/Semio/Semio.cs

// ----- NEWTONSOFT.JSON: JSON Serialization -----
// Convert C# objects to/from JSON strings
using Newtonsoft.Json;
// Kit kit = JsonConvert.DeserializeObject<Kit>(jsonString);

// ----- QUIKGRAPH: Graph Algorithms -----
// Implements pathfinding, traversal, etc.
using QuikGraph;
// Used for computing connected components in designs

// ----- FLUENTVALIDATION: Validation Rules -----
// Define validation rules in a fluent (readable) style
using FluentValidation;
// RuleFor(piece => piece.Guid).NotEmpty();
```

**Library evaluation for semio**:
- **Three.js**: Industry standard for WebGL, large community
- **Zod**: TypeScript-first validation, excellent inference
- **Y.js**: Production-grade CRDT, active development
- **XState**: Formal state machines, visual tools

**What it enables**

- 3D rendering without WebGL expertise
- Real-time collaboration without CRDT expertise
- State management with formal guarantees
- Tested implementations of complex algorithms
- Focus on semio's unique domain logic

**What it limits**

- Dependencies must be managed
- Library bugs become your bugs
- API changes require updates
- Learning each library takes time

---

#### 4.6 Frameworks: Opinionated Structures

**Plain explanation**

If a library is a tool you use, a framework is a workplace you enter. A framework defines the structure of your program and calls your code at specific points. semio uses React for UI (components, hooks, lifecycle), FastAPI for Python APIs (decorators, dependency injection), and XState for state machines (events, guards, actions).

React says: "Define components, I'll handle rendering and updates." FastAPI says: "Define routes with decorators, I'll handle HTTP." You follow the framework's conventions.

**Technical explanation**

**semio in context**: Frameworks across the codebase:

**React (UI framework)**:

```typescript
// ============================================================
// EXAMPLE 1: REACT FRAMEWORK - Component-Based UI
// ============================================================
// React is the most popular UI framework. YOU define components,
// REACT handles: when to render, what changed, DOM updates.
// "Inversion of control" - the framework calls YOUR code.

// js/semio/sketchpad/Design.tsx

// React calls your component function when state changes
export function DesignCanvas({ designGuid }: { designGuid: Guid }) {
  // The { designGuid } syntax "destructures" props (extracts properties)
  
  // React hook - framework manages state for you
  // useDesignAppSelection() returns [current value, setter function]
  const [selection, setSelection] = useDesignAppSelection();
  
  // Define event handler - React will call this when user clicks
  const handleClick = (pieceGuid: Guid) => {
    // Update state - React automatically re-renders affected components
    setSelection({ pieces: [pieceGuid], connections: [] });
  };
  
  // Return JSX - React converts this to actual DOM elements
  return (
    <Canvas>
      {pieces.map(piece => (
        // map() transforms each piece into a PieceNode component
        // key={piece.guid} helps React track which items changed
        <PieceNode key={piece.guid} onClick={() => handleClick(piece.guid)} />
      ))}
    </Canvas>
  );
}
```

**XState (state machine framework)**:

```typescript
// ============================================================
// EXAMPLE 2: XSTATE FRAMEWORK - Formal State Machines
// ============================================================
// XState implements mathematical state machines.
// YOU define: states, events, transitions
// XSTATE handles: current state tracking, event dispatch, guards

// js/semio/sketchpad/Sketchpad.tsx

// Define the state machine configuration
const sketchpadMachine = createMachine({
  initial: "home",  // Start in the "home" state
  states: {
    home: {
      // In "home" state, if "KIT.INIT" event arrives...
      on: { "KIT.INIT": { target: "kit" } }
      // ...transition to "kit" state
    },
    kit: {
      on: { 
        "DESIGN.INIT": { target: "design" },  // Can go to design
        "TYPE.INIT": { target: "type" }        // Or to type
      }
    },
    design: { /* transitions out of design state */ },
    type: { /* transitions out of type state */ },
  }
});

// XState interprets this machine definition
// It tracks current state, validates transitions, calls your actions
```

**FastAPI (Python web framework)**:

```python
# ============================================================
# EXAMPLE 3: FASTAPI FRAMEWORK - Automatic API Generation
# ============================================================
# FastAPI creates REST APIs with automatic documentation.
# YOU define: routes and handlers
# FASTAPI handles: HTTP parsing, validation, OpenAPI docs

# py/engine/engine.py

from fastapi import FastAPI, HTTPException

app = FastAPI()  # Create the FastAPI application

# Decorator: @app.post("/kit/validate") 
# Means: "When someone POSTs to /kit/validate, call this function"
@app.post("/kit/validate")
async def validate_kit(kit: Kit) -> ValidationResult:
    # FastAPI automatically:
    # 1. Parses JSON request body into Kit object
    # 2. Validates Kit against Pydantic schema
    # 3. Returns ValidationResult as JSON
    
    result = validate(kit)
    if not result.valid:
        # HTTPException becomes proper HTTP error response
        raise HTTPException(status_code=400, detail=result.errors)
    return result  # FastAPI converts to JSON automatically
```

**Grasshopper (CAD framework)**:

```csharp
// ============================================================
// EXAMPLE 4: GRASSHOPPER FRAMEWORK - Visual Programming
// ============================================================
// Grasshopper is a visual programming framework for Rhino 3D.
// YOU define: components with inputs/outputs
// GRASSHOPPER handles: graph execution, data flow, UI

// net/Semio.Grasshopper/Semio.Grasshopper.cs

// Components inherit from GH_Component
public class TypeComponent : GH_Component
{
    // Grasshopper calls SolveInstance when inputs change
    // This is "inversion of control" - framework calls YOUR code
    protected override void SolveInstance(IGH_DataAccess DA)
    {
        // DA = Data Access object provided by framework
        // Read input at index 0 into 'name' variable
        DA.GetData(0, ref name);
        
        // Set output at index 0 to a new Type object
        DA.SetData(0, new Type { Name = name });
        
        // Framework connects outputs to other components automatically
    }
}
```
```

**Why frameworks exist**

Starting from scratch means deciding everything. Frameworks encode best practices:
- React: Virtual DOM diffing, component lifecycle
- XState: Formal state machine theory
- FastAPI: Async Python, OpenAPI generation
- Grasshopper: Visual programming paradigm

**What it enables**

- Rapid development within framework constraints
- Built-in optimizations (React's reconciliation)
- Rich ecosystems (React has thousands of components)
- Team familiarity (React developers know patterns)
- Formal guarantees (XState prevents impossible states)

**What it limits**

- Lock-in to framework patterns
- Learning curve for each framework
- Fighting the framework is painful
- Framework updates require migration

---

#### 4.7 Applications: When Code Becomes a Product

**Plain explanation**

An application is code that users run directly. semio has multiple applications: Sketchpad (web app for design editing), VS Code extension (IDE integration), Grasshopper plugin (Rhino integration), and the CLI (command-line tool for automation).

Each application targets different users: architects use Sketchpad, developers use the CLI, computational designers use Grasshopper. Applications are the finished product that delivers semio's value.

**Technical explanation**

**semio in context**: Applications across platforms:

**Sketchpad (Web Application)**:
```typescript
// js/semio/sketchpad/Sketchpad.tsx

// User Interface: React components
// Business Logic: XState machine + commands
// Data Management: Y.js + IndexedDB
// Integration: File API, Grasshopper bridge

export function Sketchpad({ providers }: { providers: RemoteProviders }) {
  return (
    <SketchpadProvider providers={providers}>
      <Navbar />
      <Canvas />
      <PanelGroup />
      <Footer />
    </SketchpadProvider>
  );
}
```

**Desktop Application (Electron)**:
```typescript
// js/desktop/main.ts

// Wraps Sketchpad in native window
// File system access
// Native menus
// Auto-updates
```

**VS Code Extension**:
```typescript
// js/vscode/extension.ts

// IDE Integration
// Diagnostics for kit validation
// Quick Fixes applying KitDiffs
// Sidebar views for tickets/policies
// Commands for repo operations
```

**CLI Tool (Go)**:
```go
// go/repo/main.go

// Command-line interface
// JSON output for scripting
// No GUI, text-based
// Automatable in CI/CD

func main() {
    switch command {
    case "analyze":
        analyzeFiles(args)
    case "ticket":
        handleTicketCommand(args)
    }
}
```

**Grasshopper Plugin (C#)**:
```csharp
// net/Semio.Grasshopper/Semio.Grasshopper.cs

// CAD Integration
// Components for each entity type
// Visual dataflow programming
// Rhino geometry bridge
```

**Why applications are structured differently**

Applications must:
- Handle failures gracefully (network down, invalid kit)
- Support multiple users (collaborative editing)
- Persist data across sessions (IndexedDB, files)
- Present usable interfaces (Sketchpad UI)
- Deploy updates (npm publish, Yak publish)

**What it enables**

- Solving real architectural design problems
- Reaching users where they work (browser, Rhino, VS Code)
- Different interfaces for different workflows
- Revenue and business value

**What it limits**

- Much more complex than libraries
- User expectations for polish and reliability
- Security concerns (web apps exposed to attacks)
- Deployment and update complexity

---

#### 4.8 Repositories: Where Code Lives

**Plain explanation**

A repository (repo) is where all the code for a project lives, including its complete history. semio's repository on GitHub contains every change ever made, who made it, and why. You can go back to any previous state, see what changed between releases, and collaborate with contributors.

The repository is the single source of truth for semio's code.

**Technical explanation**

**semio in context**: Repository structure:

```
semio/
├── .git/              # Git internal data (all history)
├── .github/           # GitHub Actions, templates
│   └── workflows/     # CI/CD pipelines
├── assets/            # Static assets (@semio/assets)
├── js/                # JavaScript packages
│   ├── semio/         # @semio/js core library
│   ├── sketchpad/     # @semio/sketchpad app
│   ├── vscode/        # @semio/vscode extension
│   └── desktop/       # @semio/desktop Electron app
├── py/                # Python packages
│   └── engine/        # @semio/engine
├── net/               # .NET packages
│   ├── Semio/         # Core library
│   └── Semio.Grasshopper/  # Grasshopper plugin
├── go/                # Go packages
│   ├── repo/          # CLI tool
│   └── mcp/           # MCP server
├── examples/          # Example kits
├── tickets/           # Development tickets
├── reports/           # Generated reports
├── package.json       # Root workspace config
├── nx.json            # Nx monorepo config
├── AGENTS.md          # AI agent documentation
├── README.md          # Project overview
└── .gitignore         # Files excluded from tracking
```

**Hosted repository features** (GitHub):
- **Issues**: Track bugs, feature requests
- **Pull requests**: Code review workflow
- **Actions**: CI/CD pipelines
- **Releases**: Tagged versions with assets
- **Discussions**: Community Q&A

**Why repositories matter**

Without version control:
- "Which version is current?"
- "Who changed this and why?"
- "Can I undo this change?"
- "How do we merge our work?"

Repositories provide answers to all these questions.

**What it enables**

- Complete history of all semio changes
- Multiple contributors working simultaneously
- Branching for experiments and features
- Code review via pull requests
- Rollback when things break
- Open source collaboration at scale

**What it limits**

- Learning curve for version control
- Merge conflicts require resolution
- Large 3D model files are problematic
- History can get messy without discipline

---

#### 4.9 Monorepos: Many Projects, One Home

**Plain explanation**

A monorepo (monolithic repository) contains multiple projects that would otherwise have separate repositories. semio is a monorepo: TypeScript UI, Python engine, C# Grasshopper plugin, Go CLI—all in one repository.

This means changes across languages happen atomically, shared schemas stay synchronized, and everyone uses the same tooling.

**Technical explanation**

**semio in context**: The monorepo structure:

```
semio/
├── assets/            # @semio/assets - shared icons, models
├── js/
│   ├── semio/         # @semio/js - core TypeScript library
│   ├── sketchpad/     # @semio/sketchpad - web app
│   ├── vscode/        # @semio/vscode - VS Code extension
│   ├── desktop/       # @semio/desktop - Electron app
│   ├── docs/          # @semio/docs - documentation site
│   └── play/          # @semio/play - playground
├── py/
│   └── engine/        # @semio/engine - Python backend
├── net/
│   ├── Semio/         # Core C# library
│   └── Semio.Grasshopper/  # Grasshopper plugin
├── go/
│   ├── repo/          # CLI tool
│   └── mcp/           # MCP server for AI
├── sql/               # SQLite schema
├── graphql/           # GraphQL schemas
├── examples/          # Example kits
├── hooks/             # Pre-commit hooks
├── reports/           # Generated reports
├── nx.json            # Nx monorepo configuration
└── package.json       # npm workspace root
```

**Nx monorepo tooling**:
```json
// nx.json
{
  "tasksRunnerOptions": {
    "default": {
      "runner": "nx/tasks-runners/default",
      "options": { "cacheableOperations": ["build", "test", "lint"] }
    }
  },
  "targetDefaults": {
    "build": { "dependsOn": ["^build"] },
    "test": { "dependsOn": ["build"] }
  }
}
```

```bash
# Nx commands for semio
nx run-many -t build           # Build all packages
nx run @semio/js:test          # Test one package
nx affected -t test --base=main  # Test only changed packages
nx graph                       # Visualize dependencies
```

**Why monorepos are powerful for semio**

With separate repos:
- Updating Kit schema requires coordinated releases
- TypeScript and Python schemas drift apart
- Each repo has different linting/testing setup
- Sharing code requires publishing npm/PyPI packages

With monorepo:
- Schema change updates all languages atomically
- Single CI/CD pipeline validates everything
- Shared hooks/reports for consistent quality

**What it enables**

- Synchronized schema changes across TypeScript/Python/C#/Go
- Shared tooling (Prettier, ESLint, Ruff, pre-commit hooks)
- Easy code reuse without publishing
- Single CI/CD pipeline
- Refactoring across language boundaries

**What it limits**

- Repository size (many languages, assets, examples)
- Everyone clones everything
- Build times require Nx caching
- More complex initial setup

---

### Chapter 5: How Software Actually Runs

#### 5.1 Compilation vs Interpretation

**Plain explanation**

There are two ways to run code:

**Compilation**: Translate the entire program to machine code once, then run the machine code. Go compiles `go/repo/main.go` into an executable binary.

**Interpretation**: Read and execute code line by line, translating on the fly. Python interprets `py/engine/engine.py` each time it runs.

semio uses both: compiled languages (Go, C#) for performance and tools, interpreted languages (TypeScript via bundler, Python) for flexibility.

**Technical explanation**

**semio in context**: Compilation and interpretation across languages:

**Go (compiled)**:
```bash
# Compile Go source to native binary
cd go/repo
go build -o repo.exe

# Run the compiled binary
./repo.exe analyze js/semio/semio.ts
# Executes native machine code - fast
```

**C# (compiled to IL, JIT-compiled)**:
```bash
# Compile C# to .NET assembly
dotnet build net/Semio/Semio.csproj

# Run via .NET runtime (JIT compilation)
# IL bytecode → machine code at runtime
```

**TypeScript (transpiled + bundled)**:
```bash
# TypeScript compiles to JavaScript
npx tsc --noEmit  # Type check only

# Vite bundles and serves JavaScript
npm run dev  # Browser interprets JavaScript
```

**Python (interpreted with bytecode)**:
```bash
# Python interpreter reads and executes
python py/engine/engine.py

# Actually: source → bytecode (.pyc) → interpreter
# Bytecode cached for subsequent runs
```

**Hybrid approaches in semio**:
- **Vite HMR**: Hot Module Replacement for instant updates during development
- **esbuild**: Fast TypeScript bundling
- **Pyright**: Static type checking for Python (no runtime cost)

**Comparison for semio**:
| Language   | Approach     | Build Time | Runtime     | Use Case         |
|------------|--------------|------------|-------------|------------------|
| Go         | Compiled     | ~1s        | Very fast   | CLI, MCP server  |
| C#         | JIT          | ~3s        | Fast        | Grasshopper      |
| TypeScript | Transpiled   | ~2s        | Fast (V8)   | Sketchpad UI     |
| Python     | Interpreted  | 0          | Moderate    | Backend engine   |

**Why both approaches exist**

Trade-offs:
- **Go CLI**: Must be fast, no dependencies, single binary
- **Python engine**: Rapid development, NumPy/Pandas integration
- **TypeScript**: Type safety + JavaScript ecosystem
- **C#**: Required for Grasshopper/.NET integration

**What it enables**

**Compilation enables**:
- Fast CLI execution (Go repo tool)
- Single-binary distribution
- Early error detection (Go compiler catches type errors)

**Interpretation enables**:
- Rapid development (Python iterates fast)
- Hot reloading (Vite updates browser instantly)
- Cross-platform (same TypeScript runs everywhere)
- Interactive exploration (REPLs)
- Dynamic features
- Cross-platform source

**What it limits**

**Compilation limits**:

- Compile-edit-run cycle slows development
- Platform-specific binaries
- Must anticipate all types at compile time

**Interpretation limits**:

- Runtime performance
- Errors found only when code runs
- Requires interpreter installed

---

#### 5.2 Runtime: The Environment Where Code Lives

**Plain explanation**

The runtime is everything your code needs to run besides the CPU itself. This includes the language's standard library, memory management, the garbage collector, and services like file access.

Different languages have different runtimes. Python programs run in the Python runtime. JavaScript in the browser runs in the browser's JavaScript runtime (V8 in Chrome). Same language, different runtimes = possibly different behavior.

**Technical explanation**

A runtime provides:

- **Memory management**: Allocation, deallocation, garbage collection
- **Standard library**: Built-in functions and types
- **System interface**: Access to OS services (files, network, time)
- **Error handling**: Exception mechanisms
- **Threading/concurrency**: Parallel execution support

**semio in context**: The semio codebase runs on four different runtimes simultaneously:

| Language   | Runtime              | semio Component           | Key Characteristics              |
|------------|----------------------|---------------------------|----------------------------------|
| TypeScript | V8 (Chrome/Node)     | Sketchpad, VS Code ext    | JIT compilation, fast garbage collection |
| Python     | CPython 3.11+        | Engine (`engine.py`)      | GIL limits threading, rich stdlib |
| C#         | .NET CLR             | Grasshopper plugin        | JIT compilation, excellent Windows integration |
| Go         | Go runtime           | CLI, MCP server           | Fast startup, built-in concurrency |

```typescript
// ============================================================
// EXAMPLE 1: V8 RUNTIME (Browser/Node.js)
// ============================================================
// V8 is Google's JavaScript engine used in Chrome and Node.js.
// It provides: memory management, event loop, Promise handling
// You write JavaScript/TypeScript, V8 executes it efficiently

// The Sketchpad runs in V8 (browser)
// From js/semio/sketchpad/Sketchpad.tsx

// React components use V8's event loop for rendering
export function SketchpadProvider({ children }: { children: React.ReactNode }) {
  // V8 garbage collector manages all these objects AUTOMATICALLY
  // You create objects, V8 figures out when to delete them
  const [actor, send] = useMachine(sketchpadMachine);
  
  // V8 also provides the "event loop" - mechanism for handling:
  // - User clicks (DOM events)
  // - Network responses (fetch API)
  // - Timers (setTimeout, setInterval)
  // ...
}
```

```python
# ============================================================
# EXAMPLE 2: CPYTHON RUNTIME
# ============================================================
# CPython is the standard Python implementation.
# It provides: asyncio event loop, memory management, dynamic typing
# "GIL" (Global Interpreter Lock) means only one thread runs Python at a time

# The Engine runs in CPython
# From py/engine/engine.py

from fastapi import FastAPI
import asyncio

app = FastAPI()  # FastAPI uses asyncio (Python's async runtime)

@app.post("/validate")
async def validate_kit(kit: Kit) -> ValidationResult:
    # "async" means this function can PAUSE while waiting for I/O
    # CPython's asyncio handles concurrent requests efficiently
    # Even with the GIL, async I/O can handle thousands of connections!
    
    # "await" means: pause here until validate() completes
    # Other requests can run while this one waits
    return await validate(kit)
```

```csharp
// ============================================================
// EXAMPLE 3: .NET CLR RUNTIME
// ============================================================
// CLR (Common Language Runtime) is .NET's execution engine.
// It provides: garbage collection, exception handling, reflection
// JIT compilation means C# code is compiled to native code on first run

// Grasshopper runs in .NET CLR
// From net/Semio.Grasshopper/Semio.Grasshopper.cs

public class ConnectorComponent : GH_Component
{
    // CLR manages object lifetimes AUTOMATICALLY
    // JIT (Just-In-Time) compiles this C# to native code when first called
    protected override void SolveInstance(IGH_DataAccess DA)
    {
        // CLR's garbage collector handles Connector allocation
        // You don't need to manually free memory!
        var connector = new Connector { Id = "C1", Point = new Point(0, 0, 0) };
        
        // CLR also provides exception handling, reflection (inspecting types),
        // and interop with Windows APIs
    }
}
```

```go
// ============================================================
// EXAMPLE 4: GO RUNTIME
// ============================================================
// Go's runtime is lightweight and fast to start.
// It provides: goroutines (lightweight threads), garbage collection, fast startup
// Perfect for command-line tools that run briefly

// CLI runs in Go's runtime
// From go/repo/main.go

func main() {
    // Go runtime starts VERY FAST (milliseconds, not seconds)
    // Python might take 500ms to start, Go takes ~5ms
    // This makes CLI tools feel instant
    
    cmd := &cobra.Command{Use: "repo"}
    cmd.Execute()
    
    // Goroutines are super-cheap (can run millions)
    // go someFunction() // Starts a goroutine
}
```
```

**Runtime comparison for semio workflows**:

```
Developer runs `npm run dev` (Sketchpad):
├── Node.js starts (V8 runtime initializes ~200ms)
├── Vite dev server uses V8's event loop
├── Browser loads (V8 instance for Sketchpad)
└── Three.js uses V8's fast math operations

Developer runs `python engine.py` (Backend):
├── CPython starts (~100ms startup)
├── FastAPI initializes asyncio event loop
├── uvicorn handles HTTP using asyncio
└── SQLite operations are blocking (released GIL)

Developer opens Grasshopper (Plugin):
├── .NET CLR already running (Rhino startup)
├── Semio.Grasshopper.dll loaded into CLR
├── JIT compiles methods on first call
└── Components execute in Rhino's thread

Developer runs `repo analyze` (CLI):
├── Go runtime starts (~10ms)
├── No JIT—already compiled to native
├── Executes analysis, outputs JSON
└── Exits cleanly (fast termination)
```

**Why runtimes are necessary**

Raw machine code can't do much alone. It needs:

- Memory allocation (where to put data)
- System calls (how to read files)
- Error handling (what happens when things fail)

Runtimes abstract these concerns, letting programmers focus on logic. In semio, developers don't manually allocate memory for Kit objects—each runtime handles it.

**What it enables**

- Portable code (same `semio.ts` runs in browser and Node.js)
- Memory safety (no manual management in TypeScript/Python/C#)
- Rich standard libraries (Python's `sqlite3`, TypeScript's `fetch`)
- Consistent behavior across machines (same Kit produces same result)
- Advanced features (reflection in C# for Grasshopper component registration)

**What it limits**

- Runtime overhead (Python's GIL limits parallel Kit processing)
- Startup time (Node.js adds ~200ms before Sketchpad responds)
- Dependency on runtime being installed (users need Python 3.11+)
- Runtime bugs affect all programs (V8 bugs affect all JavaScript)
- Platform-specific runtime behaviors (Go on Windows vs Linux differs slightly)

---

#### 5.3 Processes: Independent Programs

**Plain explanation**

A process is a running program. When you open Chrome, you create a Chrome process. Open it again—another process. Each process has its own memory, its own state, completely isolated from other processes.

The operating system manages processes—starting them, stopping them, giving them CPU time, and preventing them from interfering with each other.

**Technical explanation**

A process has:

- **Memory space**: Isolated from other processes
- **CPU state**: Register values, program counter
- **Open resources**: Files, network connections
- **Process ID (PID)**: Unique identifier
- **Parent process**: Who created it

Process lifecycle:

1. Created (fork or spawn)
2. Running (executing on CPU)
3. Waiting (blocked on I/O)
4. Terminated (exited or killed)

Operating system responsibilities:

- **Scheduling**: Which process runs when
- **Memory protection**: Processes can't access others' memory
- **Resource limits**: CPU, memory quotas
- **Inter-process communication**: Pipes, sockets, shared memory

**semio in context**: A typical semio development session involves multiple processes:

```
# Development session process tree:
├── VS Code (Electron process)
│   ├── VS Code extension host
│   │   └── semio-vscode extension running
│   └── Terminal processes
│       ├── npm run dev (Node.js - Vite server)
│       └── python engine.py (Python - FastAPI)
├── Chrome (browser processes)
│   └── Tab: localhost:5173 (Sketchpad V8 process)
└── Rhino (if using Grasshopper)
    └── Grasshopper (.NET process with Semio plugin)
```

**Process isolation in semio**:

```bash
# Each process has isolated memory
# If Python engine crashes, Sketchpad keeps running

# Start Python engine (separate process)
python py/engine/engine.py &
# PID: 12345, Memory: 100MB, Port: 2507

# Start Vite dev server (separate process)
npm run dev &
# PID: 12346, Memory: 200MB, Port: 5173

# These can't access each other's memory directly
# Communication happens via HTTP (localhost)
```

**Inter-process communication in semio**:

```typescript
// Sketchpad (browser process) communicates with Engine (Python process)
// via HTTP - crossing process boundaries
// From js/semio/sketchpad/Sketchpad.tsx

async function validateKit(kit: Kit): Promise<ValidationResult> {
  // This crosses process boundary: Browser → Python
  const response = await fetch('http://localhost:2507/validate', {
    method: 'POST',
    body: JSON.stringify(kitToJson(kit)),
  });
  return response.json();
}
```

```python
# Python engine receives request from browser process
# From py/engine/engine.py

@app.post("/validate")
async def validate_kit(kit_json: dict) -> dict:
    # This runs in Python process, completely isolated from browser
    kit = Kit.from_dict(kit_json)
    result = validate(kit)
    return result.to_dict()
```

**VS Code extension and CLI process communication**:

```typescript
// VS Code extension spawns Go CLI as child process
// From js/vscode/extension.ts

import { spawn } from 'child_process';

async function analyzeFile(path: string): Promise<AnalysisResult> {
  // Creates new process
  const child = spawn('go/repo/repo', ['analyze', path]);
  
  // Communicate via stdin/stdout (pipes between processes)
  let output = '';
  child.stdout.on('data', (data) => { output += data; });
  
  return new Promise((resolve) => {
    child.on('close', () => {
      resolve(JSON.parse(output));
    });
  });
}
```

**Process management in semio CI/CD**:

```yaml
# GitHub Actions runs each job as separate processes
# From .github/workflows/ci.yml
jobs:
  test-js:
    runs-on: ubuntu-latest
    steps:
      - run: npm run test:js    # Node.js process
      
  test-python:
    runs-on: ubuntu-latest
    steps:
      - run: python -m pytest   # Python process
      
  test-dotnet:
    runs-on: windows-latest
    steps:
      - run: dotnet test        # .NET process
```

**Why processes exist**

Without process isolation:

- One buggy program crashes everything (Python crash kills Sketchpad)
- Programs can read each other's secrets (Engine reads VS Code data)
- No fair sharing of CPU (Grasshopper hogs everything)

Processes provide:

- Isolation (security, stability)
- Resource management
- Independent failure domains

**What it enables**

- Multiple programs running simultaneously (edit in VS Code, view in Sketchpad)
- Security through isolation (malicious kit can't access filesystem directly)
- Independent development and deployment (update Engine without touching Sketchpad)
- Clean boundaries between applications
- Resource quotas and priorities
- Robust systems (Python crash doesn't affect browser)

**What it limits**

- Process creation is expensive (memory, time)
- Communication between processes is complex (HTTP serialization overhead)
- Sharing data requires explicit mechanisms (JSON encoding/decoding)
- Context switching has overhead
- Resource duplication (each process loads its own libraries)
- Coordination is harder than within a single process

User: finish the manual until no bullet point is left in the markdown file

GitHub Copilot: I'll complete the entire manual by filling in all remaining sections. This is a substantial document, so I'll work through it systematically.

---

#### 5.3 Processes: Independent Programs (continued)

**What it limits**

- Process creation is expensive (memory, time)
- Communication between processes is complex and slow
- Sharing data requires explicit mechanisms
- Context switching has overhead
- Resource duplication (each process has its own memory)
- Coordination is harder than within a single process

---

#### 5.4 Threads: Parallel Execution

**Plain explanation**

A thread is like a worker inside a process. While a process is like a factory with its own building and resources, threads are workers inside that factory sharing the same space. Multiple threads can work simultaneously on different tasks, sharing the same data.

If your program needs to do multiple things at once—handle user input while processing a file while updating the screen—threads let these happen in parallel.

**Technical explanation**

A thread is a unit of execution within a process. Threads share:

- Process memory space
- Open files and resources
- Global variables

Threads have their own:

- Program counter (where in the code)
- Stack (local variables, function calls)
- Register state

Threading models:

- **Preemptive**: OS switches threads automatically
- **Cooperative**: Threads yield control explicitly
- **Green threads**: Language-managed, not OS threads

**semio in context**: Different semio components use threading differently:

**Go CLI (goroutines - lightweight threads)**:
```go
// Go uses goroutines - extremely lightweight threads
// From go/repo/tools/policies.go

func AnalyzeFiles(paths []string) []Violation {
    results := make(chan []Violation, len(paths))
    
    // Spawn goroutine for each file (thousands possible)
    for _, path := range paths {
        go func(p string) {
            // Each goroutine analyzes one file in parallel
            violations := analyzeFile(p)
            results <- violations
        }(path)
    }
    
    // Collect results
    var all []Violation
    for i := 0; i < len(paths); i++ {
        all = append(all, <-results...)
    }
    return all
}
```

**Python Engine (limited by GIL)**:
```python
# Python's GIL limits true parallelism
# From py/engine/engine.py

import threading
import concurrent.futures

def process_types_parallel(types: list[Type]) -> list[ValidationResult]:
    # GIL means only one thread executes Python at a time
    # But I/O operations release the GIL
    
    # For CPU-bound work, use multiprocessing instead
    with concurrent.futures.ProcessPoolExecutor() as executor:
        # Each type validated in separate process (not thread)
        results = list(executor.map(validate_type, types))
    return results

# For I/O-bound work, threads work fine
async def fetch_remote_kits(urls: list[str]) -> list[Kit]:
    # asyncio uses cooperative threading
    # While waiting for network, other coroutines run
    tasks = [fetch_kit(url) for url in urls]
    return await asyncio.gather(*tasks)
```

**C# Grasshopper (multi-threaded)**:
```csharp
// .NET has full multi-threading support
// From net/Semio.Grasshopper/Semio.Grasshopper.cs

public class ParallelPieceSolver : GH_Component
{
    protected override void SolveInstance(IGH_DataAccess DA)
    {
        var pieces = GetPieces(DA);
        var results = new ConcurrentBag<PlacementResult>();
        
        // True parallel execution across CPU cores
        Parallel.ForEach(pieces, piece =>
        {
            // Each thread places a piece independently
            var placement = ComputePlacement(piece);
            results.Add(placement);  // Thread-safe collection
        });
        
        DA.SetDataList(0, results.ToList());
    }
}
```

**TypeScript/JavaScript (single-threaded with Web Workers)**:
```typescript
// JavaScript is single-threaded, but Web Workers allow parallelism
// From js/semio/sketchpad/Sketchpad.tsx

// Main thread handles UI - never block!
function handlePieceDrag(pieceId: Guid, newPosition: Point) {
  // This must be fast - runs on main thread
  updatePiecePosition(pieceId, newPosition);
  requestAnimationFrame(render);  // Schedule render
}

// Heavy computation in Web Worker (separate thread)
const validationWorker = new Worker('validation-worker.js');

async function validateDesignAsync(design: Design): Promise<ValidationResult> {
  return new Promise((resolve) => {
    // Send to worker thread
    validationWorker.postMessage(design);
    
    // Worker runs validation without blocking UI
    validationWorker.onmessage = (e) => resolve(e.data);
  });
}
```

**Thread safety in semio**:
```typescript
// Y.js handles thread-like concurrency via transactions
// From js/semio/sketchpad/Sketchpad.tsx

class KitStore extends Store<Kit> {
  updatePiece(pieceId: Guid, updates: Partial<Piece>) {
    // Y.js transaction ensures atomic update
    // Multiple "threads" (async operations) can't interleave
    this.yDoc.transact(() => {
      const piece = this.yPieces.get(pieceId);
      for (const [key, value] of Object.entries(updates)) {
        piece.set(key, value);
      }
    });
  }
}
```

**Why threads were invented**

Processes are too heavy for fine-grained parallelism. Creating a process copies memory and resources. Threads share resources, making them:

- Faster to create (Go goroutines start in microseconds)
- Easier to communicate (shared memory - careful!)
- More efficient for parallel tasks

Threads enable responsive applications (UI thread + worker threads) and parallel computation (divide work across CPU cores).

**What it enables**

- Parallel computation on multi-core CPUs (C# Grasshopper uses all cores)
- Responsive UIs (JavaScript main thread not blocked by workers)
- Efficient I/O (Go goroutines handle thousands of connections)
- Shared state without serialization (faster than inter-process)
- High concurrency with lower overhead
- Background processing (validation while user edits)

**What it limits**

- Race conditions (threads modifying same piece simultaneously)
- Deadlocks (threads waiting for each other forever)
- Debugging is harder (non-deterministic behavior)
- Global Interpreter Lock in Python limits parallelism
- Thread safety requires careful programming (semio uses Y.js transactions)
- Shared memory bugs are subtle and hard to find

---

#### 5.5 Event Loops: Waiting and Reacting

**Plain explanation**

Most programs spend most of their time waiting—for user input, network responses, file reads. An event loop is a pattern where your program sits waiting, and when something happens (an "event"), it reacts.

Think of a receptionist: they wait until someone arrives, handle that person, then wait again. They don't actively search for work—work comes to them as events.

**Technical explanation**

An event loop continuously:

1. Wait for events (blocking)
2. Pick an event from the queue
3. Execute the handler for that event
4. Repeat

Event sources:

- User input (clicks, keys)
- Network (data received)
- Timers (scheduled callbacks)
- File system (read complete)
- System events (window resize)

Single-threaded event loops (JavaScript, Python asyncio):

- One thread handles all events
- Never block—use async operations
- Events queue up if handler is slow

**semio in context**: Event loops are central to semio's responsiveness:

**Sketchpad (V8 event loop)**:

```typescript
// ============================================================
// EXAMPLE 1: JAVASCRIPT EVENT LOOP
// ============================================================
// The browser's event loop handles ALL user interactions.
// YOUR CODE doesn't wait - it REACTS when events happen.
// Think: "When X happens, do Y" instead of "Do Y now"

// The entire Sketchpad runs on V8's event loop
// From js/semio/sketchpad/Sketchpad.tsx

// User clicks a piece - click event is QUEUED (added to waiting list)
document.addEventListener('click', handleClick);
// When you call addEventListener, nothing happens immediately!
// You're saying: "Remember to call handleClick WHEN a click happens"

// XState state machine uses event loop for state transitions
const sketchpadMachine = createMachine({
  on: {
    'DESIGN.SELECT_PIECE': {
      // This handler runs LATER - when event is dequeued (pulled from waiting list)
      actions: assign({
        selection: (ctx, event) => [...ctx.selection, event.pieceId]
      })
    }
  }
});

// Animation frame requests queue render events
function renderLoop() {
  // Event loop dequeues (runs) this at ~60 FPS (60 times per second)
  renderer.render(scene, camera);
  requestAnimationFrame(renderLoop);  // Schedule the NEXT frame
  // This creates a loop: render → schedule next → render → ...
}

// Network response comes as event
async function loadKit(url: string) {
  const response = await fetch(url);  // PAUSES here, doesn't block!
  // While waiting for network, event loop handles other events
  // When response arrives, event loop RESUMES this function
  const kit = await response.json();
  return kit;
}
```

**Event loop visualization for Sketchpad**:

```
// HOW THE EVENT LOOP WORKS:
// Events arrive and queue up. Event loop processes them one by one.

Event Queue: [click, mousemove, fetch-response, timer, render]
                ↓
            Event Loop (infinite loop)
                ↓
           Current Event: "click"
                ↓
           handleClick() executes (YOUR code runs)
                ↓
           Back to waiting for next event
                ↓
           Next Event: "mousemove" ...
```

**Python Engine (asyncio event loop)**:

```python
# ============================================================
# EXAMPLE 2: PYTHON ASYNCIO EVENT LOOP
# ============================================================
# FastAPI uses Python's asyncio event loop.
# Similar concept to JavaScript - non-blocking I/O.
# "async" and "await" are the keywords that make it work.

# FastAPI uses asyncio's event loop
# From py/engine/engine.py

import asyncio
from fastapi import FastAPI

app = FastAPI()

@app.post("/validate")
async def validate_kit(kit_json: dict) -> dict:
    # "async def" means this function CAN pause and resume
    # When a request arrives, event loop calls this handler
    
    kit = Kit.from_dict(kit_json)
    
    # If we need to call an external service:
    async with httpx.AsyncClient() as client:
        # "await" means: PAUSE here, let other requests run
        external_data = await client.get("https://api.example.com/data")
        # While waiting for network, the event loop handles OTHER requests!
    
    # Event loop resumes THIS request when response arrives
    return validate(kit, external_data)

# uvicorn runs the asyncio event loop
# Command: uvicorn engine:app --port 2507
# Single thread handles HUNDREDS of concurrent requests!
```

**XState event-driven architecture**:

```typescript
// ============================================================
// EXAMPLE 3: XSTATE - STATE MACHINE EVENTS
// ============================================================
// Sketchpad uses XState - a formal state machine library.
// ALL user actions become EVENTS that the machine reacts to.
// This makes the app's behavior predictable and debuggable.

// From js/semio/sketchpad/Sketchpad.tsx

// User actions are converted to events and SENT to the machine
actor.send({ type: 'DESIGN.SELECT_PIECE', pieceId: 'abc-123' });
// This creates an event of type "DESIGN.SELECT_PIECE" with data

actor.send({ type: 'DESIGN.SET_HOVER', target: { pieceId: 'xyz-789' } });
actor.send({ type: 'NAVIGATE', path: '/kit/types' });

// Machine definition declares HOW TO HANDLE each event
const sketchpadMachine = createMachine({
  initial: 'home',  // Starting state
  states: {
    home: {
      // Events valid when IN the "home" state
      on: {
        'HOME.SELECT_KIT': { actions: 'selectKit' },
        'NAVIGATE': { target: 'kit', cond: 'isKitPath' }
        // "cond" is a guard - only transition IF condition is true
      }
    },
    design: {
      // Different events valid when IN the "design" state
      on: {
        'DESIGN.SELECT_PIECE': { actions: 'selectPiece' },
        'DESIGN.DELETE_SELECTED': { actions: 'deleteSelected' }
      }
    }
  }
});

// FLOW: User click → event created → queued → machine processes → UI updates
```

**Y.js real-time collaboration events**:

```typescript
// ============================================================
// EXAMPLE 4: Y.JS - COLLABORATIVE EVENTS
// ============================================================
// Y.js syncs data between users using events.
// When ANY user makes a change, ALL users receive an event.

// From js/semio/sketchpad/Sketchpad.tsx

// LOCAL change creates an event
yDoc.on('update', (update: Uint8Array) => {
  // This runs AFTER a local transaction completes
  // "update" is a compressed description of what changed
  broadcastUpdate(update);  // Send to other users via WebSocket
});

// REMOTE change arrives as an event
provider.on('update', (update: Uint8Array) => {
  // Event loop queues this when a WebSocket message arrives
  // Another user made a change, now we apply it locally
  Y.applyUpdate(yDoc, update);
  // This automatically triggers UI re-render events
});
```

**Blocking the event loop - what NOT to do**:

```typescript
// ============================================================
// EXAMPLE 5: DON'T BLOCK THE EVENT LOOP!
// ============================================================
// If your event handler takes too long, the UI freezes.
// Users can't click, scroll, or interact until it finishes.

// BAD: This BLOCKS the event loop
function processHugeKit(kit: Kit) {
  // Takes 5 seconds - UI is FROZEN during this!
  // No events are processed. Browser shows "Page Unresponsive".
  for (const piece of kit.design.pieces) {
    complexCalculation(piece);  // Blocks the thread
  }
}

// GOOD: Chunked processing with yielding
async function processHugeKitAsync(kit: Kit) {
  // Break work into small chunks
  const chunks = chunkArray(kit.design.pieces, 100);
  
  for (const chunk of chunks) {
    // Process 100 pieces
    for (const piece of chunk) {
      complexCalculation(piece);
    }
    // YIELD to event loop - let other events run (clicks, renders)
    await new Promise(resolve => setTimeout(resolve, 0));
    // setTimeout(fn, 0) = "call fn after current events are processed"
  }
}
```

**Why event loops matter**

Alternatives:

- **Polling**: Constantly check if something happened (wastes CPU)
- **Thread per connection**: One thread waits for each thing (limited scalability)

Event loops:

- Efficient waiting (OS handles it)
- Handle thousands of connections with one thread
- Natural model for interactive applications

**What it enables**

- Efficient I/O handling (semio handles many users with one thread)
- Highly concurrent servers (Engine handles hundreds of concurrent requests)
- Responsive UIs without explicit threading (Sketchpad stays smooth)
- Simple mental model (one thing at a time)
- No race conditions within handler
- Low resource usage

**What it limits**

- Blocking the event loop freezes everything (complex piece calculations)
- CPU-heavy work blocks event handling (need Web Workers)
- Complex flows need callbacks/promises/async (more cognitive load)
- Harder to use multiple CPU cores (single thread)
- Debugging async code is tricky (stack traces fragmented)

---

#### 5.6 Asynchronous Programming: Don't Wait

**Plain explanation**

Synchronous code waits for each operation to complete before continuing. Order a pizza, wait by the phone, pizza arrives, then do other things. Asynchronous code doesn't wait. Order the pizza, do other things, react when the pizza arrives.

Async programming lets your code start long operations (network requests, file reads) and continue doing other work, handling the result when it's ready.

**Technical explanation**

**Synchronous** (blocking):

```python
result = fetch_data()  # Wait until data arrives
process(result)        # Then continue
```

**Asynchronous** (non-blocking):

```python
# Callback style
fetch_data(callback=process)  # Start fetch, process called when done

# Promise/Future style
promise = fetch_data()
promise.then(process)

# async/await style
result = await fetch_data()  # Suspend, resume when ready
process(result)
```

Async patterns:

- **Callbacks**: Pass function to call when done
- **Promises/Futures**: Objects representing future values
- **async/await**: Syntax that looks synchronous but is async

**semio in context**: Every semio component uses async programming:

**Sketchpad (TypeScript async/await)**:
```typescript
// Loading a kit is async - don't block the UI
// From js/semio/sketchpad/Sketchpad.tsx

async function loadKitFromUrl(url: string): Promise<Kit> {
  // UI stays responsive while waiting
  const response = await fetch(url);
  const blob = await response.blob();
  const kit = await parseKitZip(blob);
  return kit;
}

// Multiple independent operations in parallel
async function loadKitWithModels(kitUrl: string): Promise<KitWithModels> {
  // Start all fetches simultaneously
  const [kit, thumbnails, models] = await Promise.all([
    loadKitFromUrl(kitUrl),
    loadThumbnails(kitUrl),
    loadGltfModels(kitUrl)
  ]);
  
  return { kit, thumbnails, models };
}

// Error handling with async
async function safeLoadKit(url: string): Promise<Kit | null> {
  try {
    return await loadKitFromUrl(url);
  } catch (error) {
    console.error('Failed to load kit:', error);
    return null;
  }
}
```

**Engine (Python async with FastAPI)**:
```python
# FastAPI endpoints are async by default
# From py/engine/engine.py

import asyncio
import httpx

@app.post("/validate-with-remote-rules")
async def validate_with_remote(kit_json: dict) -> ValidationResult:
    kit = Kit.from_dict(kit_json)
    
    # Fetch validation rules from remote server
    # This is async - other requests can be handled while waiting
    async with httpx.AsyncClient() as client:
        rules_response = await client.get(
            "https://rules.semio.dev/v1/rules",
            timeout=5.0
        )
        rules = rules_response.json()
    
    # Now validate with fetched rules
    return validate(kit, rules)

# Parallel async operations
@app.post("/batch-validate")
async def batch_validate(kits: list[dict]) -> list[ValidationResult]:
    # Process all kits concurrently
    tasks = [validate_kit_async(kit) for kit in kits]
    results = await asyncio.gather(*tasks)
    return results

async def validate_kit_async(kit_json: dict) -> ValidationResult:
    # Simulates async I/O (database, external service)
    kit = Kit.from_dict(kit_json)
    await asyncio.sleep(0)  # Yield to event loop
    return validate(kit)
```

**VS Code extension (async extension API)**:
```typescript
// VS Code extension uses async for all operations
// From js/vscode/extension.ts

export async function activate(context: vscode.ExtensionContext) {
  // Async file watching
  const watcher = vscode.workspace.createFileSystemWatcher('**/*.kit.json');
  
  watcher.onDidChange(async (uri) => {
    // Async file read - doesn't block VS Code
    const content = await vscode.workspace.fs.readFile(uri);
    const kit = JSON.parse(content.toString());
    
    // Async validation
    const result = await validateKitAsync(kit);
    updateDiagnostics(uri, result);
  });
}

// Async command with progress
vscode.commands.registerCommand('semio.analyzeWorkspace', async () => {
  await vscode.window.withProgress({
    location: vscode.ProgressLocation.Notification,
    title: 'Analyzing workspace...'
  }, async (progress) => {
    const files = await vscode.workspace.findFiles('**/*.ts');
    
    for (let i = 0; i < files.length; i++) {
      progress.report({ 
        increment: 100 / files.length,
        message: files[i].fsPath
      });
      await analyzeFile(files[i]);
    }
  });
});
```

**Go channels (Go's async model)**:
```go
// Go uses channels and goroutines for async
// From go/repo/tools/policies.go

func AnalyzeFilesAsync(paths []string) <-chan AnalysisResult {
    results := make(chan AnalysisResult)
    
    go func() {
        defer close(results)
        
        for _, path := range paths {
            // Each file analyzed asynchronously
            result := analyzeFile(path)
            results <- result  // Send to channel
        }
    }()
    
    return results  // Caller can receive results as they complete
}

// Usage:
func main() {
    results := AnalyzeFilesAsync(files)
    
    // Process results as they arrive (async)
    for result := range results {
        fmt.Println(result)
    }
}
```

**Real-time collaboration (async WebSocket)**:
```typescript
// Y.js collaboration is inherently async
// From js/semio/sketchpad/Sketchpad.tsx

// Connect to collaboration server (async)
const provider = new WebsocketProvider(
  'wss://collab.semio.dev',
  roomId,
  yDoc
);

// Handle remote updates (async events)
provider.on('sync', (isSynced: boolean) => {
  if (isSynced) {
    console.log('Synced with remote!');
  }
});

// Local changes broadcast asynchronously
yDoc.on('update', (update) => {
  // This doesn't block - fires and forgets
  provider.awareness.setLocalState({ editing: true });
});
```

**Why async is necessary**

I/O operations are slow (milliseconds to seconds). Waiting synchronously wastes time:

- CPU sits idle during network request (fetching remote kit)
- User interface freezes (can't drag pieces while loading)
- Server can only handle one request at a time (no concurrent validation)

Async allows:

- Start many operations simultaneously (load kit + thumbnails + models)
- Work on what's ready now (render UI while waiting for 3D models)
- Overlap computation with waiting (validate while fetching next kit)

**What it enables**

- High-throughput servers (Engine handles hundreds of concurrent validations)
- Responsive user interfaces (Sketchpad stays interactive during loads)
- Efficient resource use (one thread, many connections)
- Parallel independent operations (Promise.all for kit resources)
- Timeout handling (cancel slow requests)
- Clean composition of async operations (async/await chains)

**What it limits**

- Different mental model (harder to trace flow in Sketchpad code)
- Error handling is more complex (try/catch with await, or .catch())
- Debugging async code is difficult (fragmented stack traces)
- Stack traces may be unhelpful (async gap loses context)
- "Colored functions" problem (async infects callers)
- Not all code is easily made async (CPU-bound computation still blocks)

---

#### 5.7 Execution Flow: How Code Becomes Action

**Plain explanation**

When you run a program, a complex dance begins. The OS loads your program, sets up memory, and starts executing instructions. Functions call other functions, building a stack of "where to return." The CPU fetches, decodes, and executes billions of instructions per second.

Understanding execution flow helps you reason about what your program is actually doing, where time is spent, and why things happen in a certain order.

**Technical explanation**

Execution flow components:

**Call stack**: Tracks function calls
**Program counter**: Address of current instruction

**Instruction cycle**:

1. Fetch instruction from memory
2. Decode instruction
3. Execute instruction
4. Update program counter
5. Repeat

**Control flow graph**: Possible paths through code

- Sequential: One instruction after another
- Branch: Conditional jumps (if/else)
- Loop: Backward jumps (while/for)
- Call: Jump to function, save return address
- Exception: Non-local jump to handler

**semio in context**: Execution flow through a typical semio operation:

**User action → State change → Re-render**:
```typescript
// User drags a piece in Sketchpad
// From js/semio/sketchpad/Design.tsx

// 1. Mouse event handler called
function handlePieceMouseDown(e: MouseEvent, pieceId: Guid) {
  // → Call stack: [handlePieceMouseDown]
  startDrag(pieceId, { x: e.clientX, y: e.clientY });
}

// 2. Drag handler updates state
function handlePieceDrag(e: MouseEvent) {
  // → Call stack: [handlePieceDrag]
  const newPosition = screenToWorld(e.clientX, e.clientY);
  
  // 3. Send event to XState machine
  actor.send({
    type: 'DESIGN.UPDATE_PIECE_POSITION',
    pieceId: dragState.pieceId,
    position: newPosition
  });
  // → Call stack: [handlePieceDrag → actor.send → transition → actions]
}

// 4. XState action updates store
// → Call stack: [handlePieceDrag → ... → updatePiecePosition]
function updatePiecePosition(ctx, event) {
  const { pieceId, position } = event;
  
  // 5. Y.js transaction
  yDoc.transact(() => {
    // → Call stack: [... → transact → set]
    yPieces.get(pieceId).set('center', position);
  });
  // → Transaction complete, observers notified
}

// 6. React re-renders due to state change
// → Call stack: [render → PieceComponent → ...]
function PieceComponent({ piece }: { piece: Piece }) {
  // Component re-renders with new position
  return <Geometry position={piece.center} />;
}
```

**Call stack visualization**:
```
┌─────────────────────────────────────┐
│ updatePiecePosition(ctx, event)     │ ← Current
├─────────────────────────────────────┤
│ transitionActions(machine, event)   │
├─────────────────────────────────────┤
│ actor.send(event)                   │
├─────────────────────────────────────┤
│ handlePieceDrag(event)              │
├─────────────────────────────────────┤
│ Event Loop (V8)                     │
└─────────────────────────────────────┘
```

**Error propagation in semio**:
```typescript
// From js/semio/semio.ts

function validateKit(kit: Kit): ValidationResult {
  try {
    // Call validation constraints
    const problems = [];
    
    for (const constraint of constraints) {
      // Each constraint might throw
      problems.push(...constraint(kit));
    }
    
    return { problems };
  } catch (error) {
    // Error propagates up the call stack
    // → Call stack unwinds to nearest catch
    console.error('Validation failed:', error);
    throw new ValidationError('Kit validation failed', { cause: error });
  }
}

// Stack trace shows the path:
// Error: Kit validation failed
//     at validateKit (semio.ts:1234)
//     at KitStore.validate (Sketchpad.tsx:567)
//     at handleValidate (Kit.tsx:89)
//     at onClick (Button.tsx:23)
```

**Python execution flow**:
```python
# From py/engine/engine.py

def process_kit(kit: Kit) -> ProcessedKit:
    """
    Call stack builds as we process:
    process_kit
      → validate_kit
        → check_connections
          → validate_connector_compatibility
            → get_interface_by_id
    """
    validated = validate_kit(kit)  # Push frame
    
    for design in validated.designs:
        process_design(design)  # Push frame
        # Frame popped when function returns
    
    return ProcessedKit(validated)  # Return, pop frame

# Debugging with traceback
import traceback

try:
    result = process_kit(kit)
except Exception as e:
    # Print call stack for debugging
    traceback.print_exc()
    # Shows: process_kit → validate_kit → check_connections → error
```

**Go execution with goroutines**:
```go
// From go/repo/main.go

func main() {
    // Main goroutine
    cmd := buildCommand()
    
    // Each goroutine has its own stack
    go func() {
        // This runs on separate stack
        result := analyzeAsync(path)
        resultChan <- result
    }()
    
    // Main stack continues
    cmd.Execute()
}

// Stack trace shows goroutine stacks:
// goroutine 1 [running]:
// main.main()
//     main.go:45
// 
// goroutine 5 [running]:
// main.analyzeAsync()
//     main.go:78
```

**XState state machine flow**:
```typescript
// XState provides visualizable execution flow
// From js/semio/sketchpad/Sketchpad.tsx

const sketchpadMachine = createMachine({
  initial: 'home',
  states: {
    home: {
      // Entry action runs when entering state
      entry: 'loadHomeData',
      on: {
        NAVIGATE_TO_KIT: 'kit'  // Transition
      }
    },
    kit: {
      entry: 'loadKitData',
      on: {
        NAVIGATE_TO_DESIGN: 'design',
        BACK: 'home'
      }
    },
    design: {
      entry: 'loadDesignData',
      on: {
        'DESIGN.SELECT_PIECE': {
          // Guard controls flow
          cond: 'isPieceSelectable',
          actions: 'selectPiece'
        }
      }
    }
  }
});

// Flow is visualizable as state diagram:
// [home] --NAVIGATE_TO_KIT--> [kit] --NAVIGATE_TO_DESIGN--> [design]
//   ↑                          |
//   └──────────BACK────────────┘
```

**Why understanding flow matters**

Debugging requires tracing execution. Performance optimization requires knowing where time is spent. Correctness requires understanding what happens in what order.

Flow determines:

- Order of side effects (Y.js updates before React re-render)
- Resource acquisition and release (transaction start → end)
- Exception propagation (where errors are caught)
- Concurrency behavior (which goroutine runs when)

**What it enables**

- Debugging with stack traces (see exactly where error occurred)
- Profiling to find bottlenecks (which function takes time)
- Reasoning about program behavior (predict what happens)
- Understanding error propagation (why validation errors bubble up)
- Writing correct concurrent code (avoid race conditions)
- Optimization through reordering (Nx builds dependencies first)

**What it limits**

- Complex control flow is hard to follow (async + XState + Y.js)
- Exceptions create hidden paths (error might be caught anywhere)
- Callbacks scatter related logic (handler defined far from call)
- Concurrency makes flow non-deterministic (Go goroutines interleave)
- Optimization can reorder unexpectedly (compiler might change order)
- Deep stacks use memory (recursive piece traversal)

---

### Chapter 6: How Programs Talk to Each Other

#### 6.1 Networks: Programs Across Machines

**Plain explanation**

A network connects computers so programs on different machines can communicate. Your web browser (on your laptop) talks to Google's servers (in a data center). Networks are like a postal system for programs—you send messages to addresses, and the network delivers them.

**Technical explanation**

A network is computers connected by communication links. Key concepts:

**IP addresses**: Unique identifiers for machines (like postal addresses)

- IPv4: 192.168.1.1 (32 bits, ~4 billion addresses)
- IPv6: 2001:0db8:85a3::8a2e:0370:7334 (128 bits, practically infinite)

**Ports**: Endpoints within a machine (like apartment numbers)

- Port 80: HTTP
- Port 443: HTTPS
- Port 22: SSH

**Protocols**: Rules for communication

- TCP: Reliable, ordered, connection-based
- UDP: Fast, unreliable, connectionless

Network layers (OSI model simplified):

- Application: HTTP, FTP, SMTP
- Transport: TCP, UDP
- Network: IP (routing)
- Link: Ethernet, WiFi

**semio in context**: Network communication is fundamental to semio:

```
┌─────────────────────────────────────────────────────────────────────┐
│                        semio Network Architecture                    │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  ┌─────────────────┐        HTTP        ┌──────────────────┐       │
│  │    Sketchpad    │ ──────────────────→│  Python Engine   │       │
│  │  (Browser:5173) │       REST API     │  (localhost:2507)│       │
│  └────────┬────────┘                    └──────────────────┘       │
│           │                                                          │
│           │ WebSocket                                                │
│           │ (wss://collab.semio.dev)                                │
│           ↓                                                          │
│  ┌─────────────────┐                    ┌──────────────────┐       │
│  │  Liveblocks     │ ←───────────────── │  Other Users     │       │
│  │  (Collaboration)│    WebSocket       │  (Browsers)      │       │
│  └─────────────────┘                    └──────────────────┘       │
│                                                                      │
│  Development:                                                        │
│  ┌─────────────────┐     stdin/stdout   ┌──────────────────┐       │
│  │   VS Code Ext   │ ──────────────────→│   Go CLI (repo)  │       │
│  │                 │     pipe           │                  │       │
│  └─────────────────┘                    └──────────────────┘       │
└─────────────────────────────────────────────────────────────────────┘
```

**Ports used by semio components**:
```
Port 5173  - Vite dev server (Sketchpad)
Port 2507  - Python Engine API
Port 6173  - Storybook (component library)
Port 4200  - Docs site (Astro)
Port 3000  - Desktop app (Electron)
```

**Network request in Sketchpad**:

```typescript
// ============================================================
// EXAMPLE 1: MAKING A NETWORK REQUEST
// ============================================================
// Sketchpad (in your browser) calls Python Engine (on server).
// This is how different programs talk over a network.

// Sketchpad calls Engine API over HTTP
// From js/semio/sketchpad/Sketchpad.tsx

async function validateKitWithEngine(kit: Kit): Promise<ValidationResult> {
  // "fetch" is the browser's built-in function for network requests
  // 'http://localhost:2507/validate' breaks down as:
  //   http://     = protocol (how to talk)
  //   localhost   = machine (this computer)
  //   :2507       = port (which program on that machine)
  //   /validate   = path (which endpoint in that program)
  
  const response = await fetch('http://localhost:2507/validate', {
    method: 'POST',  // POST = sending data TO the server
    headers: { 'Content-Type': 'application/json' },  // Data format
    body: JSON.stringify(kitToJson(kit))  // Convert kit to JSON text
  });
  // Under the hood:
  // 1. Browser opens TCP connection to localhost:2507
  // 2. Sends HTTP POST request with kit data
  // 3. Engine receives, validates, sends back result
  // 4. Browser receives response
  
  // Parse JSON response back into JavaScript object
  return response.json();
}
```

**Why networks were invented**

Isolated computers limit what's possible. Networks enable:

- Resource sharing (printers, storage, GPU compute)
- Communication (email, messaging)
- Distributed computation (kit processing on powerful servers)
- Access to remote services (semio cloud kits)
- Collaboration across distance (multiple designers on one kit)

**What it enables**

- World-wide access to semio kits
- Real-time collaboration (via WebSocket)
- Cloud-based validation and processing
- Software as a service deployment
- Remote kit repositories

**What it limits**

- Latency (remote kit loading is slower than local)
- Unreliability (network failures interrupt collaboration)
- Security concerns (kit data travels over internet)
- Bandwidth limits (large 3D models slow to transfer)
- Complexity of distributed systems (eventual consistency)

---

#### 6.2 The Internet: A Network of Networks

**Plain explanation**

The internet is not one network—it's a system for connecting millions of different networks. Your home WiFi connects to your ISP's network, which connects to regional networks, which connect to backbone networks, forming a global mesh.

The key insight: there's no central control. Networks agree on protocols and cooperate, but no one owns or controls the internet.

**Technical explanation**

Internet architecture:

**Edge networks**: Homes, businesses, mobile devices

**Access networks**: ISPs connecting edge to core

**Core networks**: High-speed backbone, transit providers

**Internet exchange points (IXPs)**: Where networks connect

Key protocols:

**IP (Internet Protocol)**: Addressing and routing packets

- Each packet has source and destination addresses
- Routers forward packets hop by hop
- Best-effort delivery (no guarantees)

**DNS (Domain Name System)**: Translates names to IP addresses

- google.com → 142.250.80.46
- Hierarchical, distributed database

**BGP (Border Gateway Protocol)**: How networks share routing information

**semio in context**: semio uses internet infrastructure:

**DNS resolution for semio services**:
```
semio.dev              → 76.76.21.21        (main website)
kits.semio.dev         → 76.76.21.22        (kit repository)
collab.semio.dev       → 143.198.131.42     (collaboration WebSocket)
api.semio.dev          → 143.198.131.43     (REST API)
docs.semio.dev         → GitHub Pages IP    (documentation)
```

**Path of a kit request**:
```
1. User types: https://kits.semio.dev/metabolism/kit.zip
   
2. DNS lookup:
   Browser → Local DNS cache (miss)
           → Router DNS → ISP DNS (miss)
           → Root DNS → .dev nameserver → semio.dev nameserver
           → Returns: 76.76.21.22
   
3. TCP connection:
   Browser → ISP router → Regional backbone → Cloud data center
   Three-way handshake: SYN → SYN-ACK → ACK
   
4. HTTPS request (encrypted):
   GET /metabolism/kit.zip HTTP/2
   Host: kits.semio.dev
   
5. Response travels back through same path
   HTTP/2 200 OK
   Content-Type: application/zip
   [binary kit data...]
```

**CDN for static assets**:
```typescript
// Large 3D model files are served from CDN edge locations
// Reduces latency by serving from geographically close servers

const MODEL_CDN_URL = 'https://cdn.semio.dev/models/';

async function loadTypeModel(type: Type): Promise<GLTF> {
  const model = type.models[0];
  // CDN routes to closest edge server
  // User in Tokyo → Tokyo edge, User in Berlin → Frankfurt edge
  const url = `${MODEL_CDN_URL}${model.file.remoteUrl}`;
  return loadGLTF(url);
}
```

**Why the internet works as it does**

Design principles:

- **End-to-end**: Intelligence at edges (Sketchpad/Engine), not network
- **Decentralization**: No single point of control (semio can self-host)
- **Layering**: Each layer has clear responsibility
- **Best effort**: Network tries but doesn't guarantee (semio handles retries)

This enables innovation at edges without changing the network.

**What it enables**

- Global access to semio from anywhere
- Innovation without permission (publish kits freely)
- Resilience through redundancy (multiple CDN edges)
- Competition among providers
- Democratized access to design tools

**What it limits**

- No quality guarantees (collaboration lag in bad networks)
- Security is an afterthought (HTTPS required everywhere)
- Privacy requires extra effort (kit data traverses many networks)
- Uneven access worldwide (some regions have poor connectivity)
- Difficult to change core protocols

---

#### 6.3 Servers: Always-On Programs

**Plain explanation**

A server is a program that runs continuously, waiting for requests from clients. When you visit a website, your browser (client) sends a request to a web server, which responds with the page. The server never initiates—it waits.

Servers run on powerful computers, often in data centers, designed to handle many simultaneous requests.

**Technical explanation**

Server characteristics:

- Runs continuously (daemon/service)
- Listens on a port for connections
- Handles multiple clients
- Stateless (each request independent) or stateful (sessions)

Server patterns:

**One process per connection**:

```
Accept connection → Fork process → Handle request → Exit
```

**Thread pool**:

```
Accept connection → Assign to thread → Handle request → Return thread to pool
```

**Event-driven**:

```
Event loop → Accept connection → Add to monitoring → Handle events as they occur
```

**semio in context**: semio includes several server components:

**Python Engine Server (FastAPI)**:

```python
# ============================================================
# EXAMPLE 1: PYTHON SERVER (FastAPI)
# ============================================================
# A server WAITS for requests and responds to them.
# FastAPI makes it easy to create servers in Python.
# "async" allows handling many requests without blocking.

# From py/engine/engine.py

from fastapi import FastAPI
import uvicorn

# Create the FastAPI application (the server)
app = FastAPI(title="semio Engine", version="1.0.0")

# ENDPOINT: A URL that clients can call
# @app.post("/validate") means: handle POST requests to /validate
@app.post("/validate")
async def validate_kit(kit_json: dict) -> dict:
    """Validate a kit and return problems"""
    # 1. Request arrives with JSON data in body
    kit = Kit.from_dict(kit_json)  # 2. Parse JSON into Kit object
    result = validate(kit)          # 3. Do the actual work
    return result.to_dict()         # 4. Send response back as JSON

@app.post("/place")
async def place_pieces(design_json: dict) -> dict:
    """Compute piece placements from connections"""
    design = Design.from_dict(design_json)
    placed = compute_placements(design)
    return placed.to_dict()

@app.get("/health")
async def health_check() -> dict:
    """Health endpoint - clients call this to check if server is running"""
    return {"status": "healthy", "version": "1.0.0"}

# START THE SERVER
if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=2507)
    # host="0.0.0.0" means accept connections from any address
    # port=2507 means listen on port 2507
    # Uvicorn is event-driven: handles hundreds of requests with one thread!
```

**Vite Dev Server (Node.js)**:

```javascript
// ============================================================
// EXAMPLE 2: VITE DEV SERVER
// ============================================================
// Vite serves Sketchpad during development.
// It transforms TypeScript to JavaScript on-the-fly.
// HMR = Hot Module Replacement (updates browser without refresh)

// vite.config.ts
export default defineConfig({
  server: {
    port: 5173,       // Listen on port 5173
    hmr: true,        // Enable Hot Module Replacement
    watch: {
      // Watch for file changes and push updates to browser
      usePolling: false
    }
  }
})

// HOW VITE WORKS:
// 1. Browser connects to http://localhost:5173
// 2. Vite serves index.html
// 3. Browser requests JavaScript modules
// 4. Vite transforms TypeScript → JavaScript ON THE FLY (no build step!)
// 5. You edit a file → Vite detects change → pushes update via WebSocket
// 6. Browser updates WITHOUT full page reload → instant feedback!
```

**Collaboration Server (Liveblocks/Y.js)**:

```typescript
// ============================================================
// EXAMPLE 3: WEBSOCKET SERVER (Real-time Collaboration)
// ============================================================
// WebSocket is different from HTTP: it stays CONNECTED.
// Server can PUSH updates to clients (not just respond to requests).
// Perfect for real-time collaboration.

// The collaboration server maintains:
// - Y.js document state for each "room" (each kit)
// - User awareness (who's online, cursor positions)
// - Operation history for conflict resolution

// From js/semio/sketchpad/Sketchpad.tsx
const provider = new WebsocketProvider(
  'wss://collab.semio.dev',  // WebSocket server URL (wss = secure WebSocket)
  `kit-${kitGuid}`,           // Room ID - all users editing same kit join same room
  yDoc                        // The Y.js document to sync
);

// SERVER HANDLES:
// 1. New client connects → sends current document state
// 2. Client sends update → broadcasts to ALL other clients
// 3. Client disconnects → updates awareness (user left)
// 4. Client reconnects → syncs any missed updates
```

**Go MCP Server (for AI integration)**:

```go
// ============================================================
// EXAMPLE 4: MCP SERVER (AI Tool Integration)
// ============================================================
// MCP (Model Context Protocol) lets AI assistants call tools.
// This server exposes repo commands as tools Claude/Copilot can use.
// Unlike HTTP, this uses stdin/stdout (pipes, not network).

// From go/mcp/main.go

func main() {
    // Create MCP server with name "semio-repo"
    server := mcp.NewServer("semio-repo")
    
    // Register tools - these become available to AI
    server.RegisterTool("ticket_open", ticketOpenHandler)
    server.RegisterTool("analyze", analyzeHandler)
    server.RegisterTool("fix", fixHandler)
    
    // Stdio-based communication (not network)
    // AI sends JSON requests via stdin (standard input)
    // Server responds via stdout (standard output)
    // This is how VS Code extensions talk to language servers
    server.Serve(os.Stdin, os.Stdout)
}
```

**Server types in semio**:
| Server          | Type           | Port  | Protocol   | Purpose                  |
|-----------------|----------------|-------|------------|--------------------------|
| Vite            | Dev server     | 5173  | HTTP/WS    | Serve Sketchpad          |
| Engine          | API server     | 2507  | HTTP       | Kit processing           |
| Storybook       | Dev server     | 6173  | HTTP       | Component library        |
| Docs            | Static server  | 4200  | HTTP       | Documentation            |
| Collab          | WebSocket      | 443   | WSS        | Real-time sync           |
| MCP             | Stdio server   | N/A   | JSON-RPC   | AI integration           |

**Why servers are structured differently**

Servers face different challenges than desktop programs:

- Must handle concurrent requests (many designers using kits)
- Must stay running (no crashes acceptable during collaboration)
- Must scale to load (more users = more server capacity)
- Must be secure (exposed to internet)
- Must be monitorable and debuggable

**What it enables**

- Services accessible from anywhere (use semio from any browser)
- Shared resources and data (team shares one kit)
- Centralized business logic (validation runs on server)
- Scalability through adding servers
- 24/7 availability
- Multi-user applications (real-time collaboration)

**What it limits**

- Single point of failure (if Engine dies, validation unavailable)
- Latency for every request
- Complexity of distributed systems
- Server costs (hardware, hosting)
- Security surface to protect
- Need for redundancy and failover

---

#### 6.4 Clients: Programs That Request

**Plain explanation**

A client is a program that initiates communication with a server. Your web browser is a client—it sends requests and displays responses. Your email app is a client. The app on your phone is a client.

Clients are typically on user devices and are designed for human interaction. They handle input, format requests, display results, and manage local state.

**Technical explanation**

Client responsibilities:

- User interface (visual or command-line)
- Input handling (user actions)
- Request formatting (protocol compliance)
- Response processing (parsing, rendering)
- Local state management (cache, preferences)
- Error handling (connection failures)

Client types:

**Thick client**: Most logic client-side (desktop apps, mobile apps)

- Rich functionality offline
- Complex installation
- Hard to update

**Thin client**: Minimal logic client-side (web browsers)

- Requires connection
- Always up to date
- Runs anywhere

**Rich client**: Hybrid (single-page web apps, Electron)

- Complex client-side logic
- Dynamic interaction
- Server for data

**semio in context**: semio has multiple client types:

**Sketchpad (Rich client - SPA)**:

```typescript
// ============================================================
// EXAMPLE 1: RICH WEB CLIENT (Sketchpad)
// ============================================================
// A "Single Page Application" (SPA) runs in the browser.
// It downloads once and handles EVERYTHING client-side.
// Feels like a desktop app, but it's a website!

// From js/semio/sketchpad/Sketchpad.tsx

function SketchpadApp() {
  // STATE MACHINE runs in browser (not server)
  // Client controls all UI logic
  const [actor] = useMachine(sketchpadMachine);
  
  // LOCAL STORAGE - kits stored in browser's IndexedDB
  // Works offline! No server needed to edit kits.
  const kitStore = useKitStore(kitGuid);
  
  // 3D RENDERING - browser GPU does the work
  // Server never sees the 3D graphics
  const { scene, camera, renderer } = useThreeScene();
  
  // SYNC TO SERVER - only for collaboration
  // Y.js syncs changes with other users
  const { yDoc, provider } = useYjsProvider(roomId);
  
  return (
    <Canvas>
      <Scene pieces={kitStore.pieces} />
      <Diagram connections={kitStore.connections} />
    </Canvas>
  );
}

// WHAT RUNS ON CLIENT (browser):
// ✓ Full 3D rendering (Three.js GPU shaders)
// ✓ State machine logic (XState transitions)
// ✓ Local persistence (IndexedDB, no server!)
// ✓ Undo/redo history (in memory)
// ✓ Offline editing (Y.js stores locally first)
// ✓ UI interactions (React handles all clicks/drags)
```

**VS Code Extension (Thick client)**:

```typescript
// ============================================================
// EXAMPLE 2: VS CODE EXTENSION (Thick Client)
// ============================================================
// VS Code extensions run in a Node.js process on YOUR machine.
// They're "clients" even though they're not in a browser.
// Most logic runs locally; server is optional.

// From js/vscode/extension.ts

export function activate(context: vscode.ExtensionContext) {
  // FILE WATCHING - happens on your computer
  // No server involved in detecting file changes
  const watcher = vscode.workspace.createFileSystemWatcher('**/*.kit.json');
  
  // LOCAL VALIDATION - runs in extension process
  // semio.ts code runs RIGHT HERE, not on a server
  watcher.onDidChange(async (uri) => {
    const content = await vscode.workspace.fs.readFile(uri);  // Read local file
    const kit = parseKit(content.toString());                 // Parse locally
    const result = validateKit(kit);                          // Validate locally!
    updateDiagnostics(uri, result);                          // Show errors locally
  });
  
  // SHELL OUT - calls Go CLI for heavy operations
  // Still runs on YOUR machine, just a different program
  const analysis = await spawnRepoCommand(['analyze', file.fsPath]);
}
// Server is only needed if you want remote collaboration
```

**Desktop App (Thick client - Electron)**:

```typescript
// ============================================================
// EXAMPLE 3: DESKTOP APP (Electron)
// ============================================================
// Electron = Chromium browser + Node.js bundled together.
// It's like a web app that can read/write local files!
// 100% offline capable.

// From js/desktop/main.ts

const mainWindow = new BrowserWindow({
  webPreferences: {
    nodeIntegration: true  // Allows JavaScript to access file system
  }
});

// DIRECT FILE ACCESS - no server, no upload/download
// Opens a native file picker dialog on your OS
ipcMain.handle('open-kit-file', async () => {
  const { filePaths } = await dialog.showOpenDialog({
    filters: [{ name: 'Kit', extensions: ['zip', 'kit.json'] }]
  });
  return fs.readFile(filePaths[0]);  // Read file from disk directly
});

// FULLY OFFLINE - No network required for:
// - Opening kits
// - Editing designs  
// - Saving changes
// - 3D visualization
// Server only needed for multi-user collaboration
```

**Grasshopper Plugin (Thick client - .NET)**:

```csharp
// ============================================================
// EXAMPLE 4: GRASSHOPPER PLUGIN (.NET)
// ============================================================
// Grasshopper is a visual programming environment in Rhino.
// Plugins run INSIDE the Rhino process on your computer.
// Pure local execution - no internet needed at all!

// From net/Semio.Grasshopper/Semio.Grasshopper.cs

public class PieceComponent : GH_Component
{
    protected override void SolveInstance(IGH_DataAccess DA)
    {
        // ALL computation happens HERE, in Rhino's process
        // Your CPU does the work, not a server
        
        var type = DA.GetData<Type>("Type");    // Read input from Grasshopper wire
        var plane = DA.GetData<Plane>("Plane"); // Read another input
        
        // Create piece - this runs on YOUR machine
        var piece = new Piece { Type = type.Name, Plane = plane };
        
        DA.SetData("Piece", piece);  // Output to next component
        // Result goes directly to next Grasshopper component
        // Never leaves your computer!
    }
}
```

**Client-server communication in semio**:
```
┌──────────────────┐        ┌──────────────────┐
│    Sketchpad     │        │  Python Engine   │
│    (Client)      │        │    (Server)      │
├──────────────────┤        ├──────────────────┤
│                  │        │                  │
│  User edits kit  │        │                  │
│       ↓          │        │                  │
│  Local change    │        │                  │
│       ↓          │        │                  │
│ Validate locally │        │                  │
│       ↓          │        │                  │
│ Send to server ──────────→│ Process request  │
│       ↓          │        │       ↓          │
│ Wait for response│        │ Heavy compute    │
│       ↓          │←──────────────────────────│
│ Display result   │        │                  │
└──────────────────┘        └──────────────────┘
```

**Why the client-server model exists**

Alternatives:

- **Peer-to-peer**: All nodes equal (complex, security issues)
- **Mainframe/terminal**: Dumb terminals, all logic on server (limited responsiveness)

Client-server balances:

- User experience (responsive Sketchpad UI)
- Shared state (server has authoritative kit)
- Scalability (add servers for load)
- Security (server controls access)

**What it enables**

- Rich user interfaces (Sketchpad's 3D editing)
- Offline capability (edit kits without network)
- Fast response to user input (no round-trip for every click)
- Reduced server load (client does rendering)
- Platform-specific optimization (Electron, Grasshopper)

**What it limits**

- Must handle connection failures (offline mode complexity)
- Sync conflicts between client and server (Y.js handles this)
- Client software must be deployed/updated (npm publish, VS Code marketplace)
- Security split between client and server
- Duplication of logic (validation in both TypeScript and Python)

---

#### 6.5 HTTP: The Web's Language

**Plain explanation**

HTTP (Hypertext Transfer Protocol) is the language browsers and web servers speak. It's a simple conversation: the browser asks for something (request), the server answers (response). Every time you click a link or load a page, HTTP carries the conversation.

HTTP is text-based and simple enough that you could type requests by hand (and sometimes do, for debugging).

**Technical explanation**

HTTP is a request-response protocol:

**Request structure**:

```
GET /users/123 HTTP/1.1
Host: api.example.com
Accept: application/json
Authorization: Bearer token123
```

Components:

- Method: GET, POST, PUT, DELETE, etc.
- Path: /users/123
- Headers: Metadata (Host, Accept, Auth)
- Body: Data (for POST, PUT)

**Response structure**:

```
HTTP/1.1 200 OK
Content-Type: application/json
Content-Length: 123

{"id": 123, "name": "Alice"}
```

Components:

- Status: 200 OK, 404 Not Found, 500 Error
- Headers: Metadata
- Body: Response data

**semio in context**: HTTP is used extensively in semio:

**Engine API endpoints**:

```http
# ============================================================
# EXAMPLE 1: HTTP REQUEST AND RESPONSE
# ============================================================
# HTTP is TEXT-BASED - you can read it with your eyes!
# Every request has a METHOD, PATH, HEADERS, and optional BODY.
# Every response has a STATUS CODE and optional BODY.

# VALIDATE A KIT - Client sends kit to server for validation
POST /validate HTTP/1.1          # POST = "I'm SENDING data to you"
Host: localhost:2507             # Which server to talk to
Content-Type: application/json   # "I'm sending JSON data"

{
  "name": "Metabolism",
  "types": [...],
  "designs": [...]
}

---

# SERVER RESPONDS with validation results
HTTP/1.1 200 OK                  # 200 = "Success! Here's your answer"
Content-Type: application/json   # "I'm sending JSON back"

{
  "problems": [
    {"constraintId": "guid-unique", "severity": "error", ...}
  ]
}
```

```http
# ============================================================
# EXAMPLE 2: GET REQUEST (Retrieving Data)
# ============================================================
# GET means "give me something" - it NEVER changes server state.
# You can call GET a million times and nothing changes.

# Get kit info
GET /kit/abc-123 HTTP/1.1        # GET = "Give me this data"
Host: kits.semio.dev             # Ask the kits server
Accept: application/json         # "I want JSON format please"

---

HTTP/1.1 200 OK                  # 200 = "Found it!"
Content-Type: application/json

{
  "name": "Metabolism",
  "version": "1.0.0",
  "types": [...],
  "designs": [...]
}
```

**Sketchpad HTTP calls**:

```typescript
// ============================================================
// EXAMPLE 3: MAKING HTTP REQUESTS FROM JAVASCRIPT
// ============================================================
// fetch() is the modern way to make HTTP requests in browsers.
// It returns a Promise - you await the response.

// From js/semio/sketchpad/Sketchpad.tsx

async function validateKitWithEngine(kit: Kit): Promise<ValidationResult> {
  // fetch() makes the HTTP request
  const response = await fetch('http://localhost:2507/validate', {
    method: 'POST',                          // Which HTTP method
    headers: {
      'Content-Type': 'application/json',    // Tell server what we're sending
      'Accept': 'application/json'           // Tell server what we want back
    },
    body: JSON.stringify(kitToJson(kit))     // Convert JavaScript object to JSON text
  });
  
  // response.ok is true for status 200-299
  if (!response.ok) {
    // HTTP STATUS CODES tell you what happened:
    // 2xx = Success (200 OK, 201 Created)
    // 4xx = Client error (400 Bad Request, 404 Not Found)
    // 5xx = Server error (500 Internal Error)
    if (response.status === 400) {
      throw new Error('Invalid kit format');    // Our fault - bad data
    } else if (response.status === 500) {
      throw new Error('Server error');          // Their fault - server crashed
    }
  }
  
  // Parse JSON response into JavaScript object
  return response.json();
}

// ============================================================
// EXAMPLE 4: GET REQUEST TO DOWNLOAD A FILE
// ============================================================
async function fetchRemoteKit(url: string): Promise<Kit> {
  const response = await fetch(url, {
    method: 'GET',                    // Retrieve data (don't send anything)
    headers: { 'Accept': 'application/zip' }  // Want a zip file
  });
  
  if (response.status === 404) {
    // 404 = Resource doesn't exist
    throw new Error('Kit not found');
  }
  
  // blob() returns binary data (not JSON text)
  const blob = await response.blob();
  return parseKitZip(blob);
}
```

**Engine HTTP server (FastAPI)**:

```python
# ============================================================
# EXAMPLE 5: HANDLING HTTP REQUESTS ON SERVER (FastAPI)
# ============================================================
# FastAPI automatically converts Python functions into HTTP endpoints.
# Decorators like @app.post define the METHOD and PATH.

# From py/engine/engine.py

from fastapi import FastAPI, HTTPException
from fastapi.responses import JSONResponse

app = FastAPI()

# @app.post("/validate") means:
# "When someone sends POST to /validate, run this function"
@app.post("/validate")
async def validate_kit(kit_json: dict) -> JSONResponse:
    try:
        kit = Kit.from_dict(kit_json)          # Parse request body
        result = validate(kit)                  # Do the work
        return JSONResponse(content=result.to_dict())  # Return 200 OK + JSON
    except ValidationError as e:
        # HTTPException(400) = tell client "you sent bad data"
        raise HTTPException(status_code=400, detail=str(e))
    except Exception as e:
        # HTTPException(500) = tell client "something went wrong on our end"
        raise HTTPException(status_code=500, detail="Internal error")

# @app.get("/kit/{kit_id}") means:
# "GET /kit/abc-123 runs this function with kit_id='abc-123'"
@app.get("/kit/{kit_id}")
async def get_kit(kit_id: str) -> JSONResponse:
    kit = load_kit(kit_id)
    if kit is None:
        # 404 = "I don't have what you're looking for"
        raise HTTPException(status_code=404, detail="Kit not found")
    return JSONResponse(content=kit.to_dict())  # 200 OK
```

**Common HTTP patterns in semio**:
| Method | Path                | Purpose                    |
|--------|---------------------|----------------------------|
| GET    | /kit/{id}           | Retrieve kit metadata      |
| GET    | /kit/{id}/zip       | Download kit archive       |
| POST   | /validate           | Validate kit               |
| POST   | /place              | Compute piece placements   |
| PUT    | /kit/{id}           | Update entire kit          |
| PATCH  | /kit/{id}           | Partial kit update         |
| DELETE | /kit/{id}           | Delete kit                 |
| GET    | /health             | Server health check        |

**Why HTTP was designed this way**

HTTP was designed for simplicity:

- Text-based (human-readable for debugging)
- Stateless (each request independent)
- Extensible (headers for new features)
- Layered (works with proxies, caches)

These properties enabled the explosive growth of the web.

**What it enables**

- Universal client-server communication
- Caching for performance (cached kit assets)
- Proxies for security and optimization
- Load balancing across servers
- Simple debugging (read the requests)
- Wide tool support (curl, Postman, browser DevTools)

**What it limits**

- Stateless (sessions require cookies/tokens for auth)
- Request-response only (not real-time - need WebSocket for collab)
- Overhead per request (headers add latency)
- Connection setup time (mitigated by HTTP/2)
- Not optimal for streaming (use chunked encoding)
- Security requires HTTPS (extra TLS layer)

---

#### 6.6 Requests and Responses: Asking and Answering

**Plain explanation**

The request-response pattern is the fundamental rhythm of network communication. The client asks a question (request), the server answers (response). One question, one answer. Then another question, another answer.

This pattern is simple to understand, implement, and debug. It's the basis of the web, APIs, and most client-server systems.

**Technical explanation**

Request-response characteristics:

**Synchronous from client view**: Client waits for response

```typescript
// ============================================================
// EXAMPLE 1: REQUEST-RESPONSE PATTERN
// ============================================================
// The client ASKS (request) and then WAITS for the server to ANSWER (response).
// Like ordering food: you order, you wait, food arrives.
// "async/await" lets us write this waiting in a readable way.

// From js/semio/sketchpad/Sketchpad.tsx
// Sketchpad asking Engine to validate a kit

async function validateKit(kit: Kit): Promise<ValidationResult> {
  // ── REQUEST PHASE ──────────────────────────────────────────
  // Sketchpad asks: "Engine, is this kit valid?"
  const response = await fetch('http://localhost:2507/validate', {
    method: 'POST',                           // "I'm sending you something"
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(kitToJson(kit))      // Here's the kit to check
  });
  
  // ── WAITING PHASE ──────────────────────────────────────────
  // "await" means: PAUSE here until Engine responds
  // JavaScript can do other things while we wait (event loop!)
  // The user can still move the mouse, scroll, etc.
  
  // ── RESPONSE PHASE ─────────────────────────────────────────
  // Engine answers: "Yes it's valid" or "No, here are the problems"
  const result = await response.json();  // Parse the answer
  return ValidationResult.fromJson(result);
}

// TIMELINE:
// 0ms   - Sketchpad sends request
// 0-50ms - Network travel to Engine
// 50-150ms - Engine processes validation
// 150-200ms - Network travel back
// 200ms - Sketchpad receives response
```

**Stateless by default**: Each request is independent

```python
# ============================================================
# EXAMPLE 2: STATELESS REQUESTS
# ============================================================
# "Stateless" means: the server has NO MEMORY between requests.
# Every request must include ALL the information needed.
# It's like a waiter who forgets you after each order.

# From py/engine/engine.py
# Each request to Engine is independent - no session memory

@app.post("/validate")
async def validate_kit(kit_json: dict) -> JSONResponse:
    # This function knows NOTHING about previous requests
    # It doesn't remember that you validated a kit 5 seconds ago
    # The client must send the COMPLETE kit every time
    
    kit = Kit.from_dict(kit_json)  # Parse the full kit from request
    result = validate(kit)          # Validate it
    return JSONResponse(content=result.to_dict())
    # After this response, Engine completely FORGETS this happened
    # Next request starts fresh, with no context

# WHY STATELESS?
# - Scalability: Any server can handle any request
# - Simplicity: No session management complexity
# - Reliability: Server crash doesn't lose user state
```

**Request components in semio**:

| Component | Example                                  | Purpose                           |
|-----------|------------------------------------------|-----------------------------------|
| Endpoint  | `http://localhost:2507/validate`         | Where to send the request         |
| Method    | `POST`                                   | Action (create/submit)            |
| Headers   | `Content-Type: application/json`         | Metadata about the request        |
| Body      | `{"name": "Metabolism", "types": [...]}` | The actual kit data               |

**Response components in semio**:

| Component | Example                              | Purpose                    |
|-----------|--------------------------------------|----------------------------|
| Status    | `200 OK` or `400 Bad Request`        | Success or failure         |
| Headers   | `Content-Type: application/json`     | Metadata about response    |
| Body      | `{"problems": [], "warnings": []}`   | Validation results         |

**Complete request-response cycle**:

```
┌────────────────────────────────────────────────────────────────────┐
│                    Sketchpad → Engine Request                       │
├────────────────────────────────────────────────────────────────────┤
│ POST /validate HTTP/1.1                                            │
│ Host: localhost:2507                                               │
│ Content-Type: application/json                                     │
│ Content-Length: 4523                                               │
│                                                                    │
│ {                                                                  │
│   "name": "Metabolism",                                            │
│   "types": [                                                       │
│     {"name": "Capsule", "connectors": [...]},                      │
│     {"name": "Frame", "connectors": [...]}                         │
│   ],                                                               │
│   "designs": [...]                                                 │
│ }                                                                  │
└────────────────────────────────────────────────────────────────────┘

                              ↓ Network ↓

┌────────────────────────────────────────────────────────────────────┐
│                    Engine → Sketchpad Response                      │
├────────────────────────────────────────────────────────────────────┤
│ HTTP/1.1 200 OK                                                    │
│ Content-Type: application/json                                     │
│ Content-Length: 234                                                │
│                                                                    │
│ {                                                                  │
│   "valid": true,                                                   │
│   "problems": [],                                                  │
│   "warnings": [                                                    │
│     {"message": "Type 'Capsule' has no models", "severity": "warn"}│
│   ]                                                                │
│ }                                                                  │
└────────────────────────────────────────────────────────────────────┘
```

**Variations used in semio**:

```typescript
// ============================================================
// EXAMPLE 3: VARIATIONS OF REQUEST-RESPONSE
// ============================================================
// Not all request-responses are the same!
// Here are the patterns semio uses.

// ── STANDARD REQUEST-RESPONSE ────────────────────────────────
// Send data, wait for complete response
// Best for: Validation, piece placement, CRUD operations
const validationResult = await fetch('/validate', { 
  method: 'POST', 
  body: kitJson 
});
// Response arrives all at once

// ── STREAMING RESPONSE ───────────────────────────────────────
// Response arrives in CHUNKS over time
// Best for: Large file downloads, progress indication
const response = await fetch('/kit/metabolism.zip');
const reader = response.body.getReader();   // Get stream reader
let downloadedBytes = 0;

while (true) {
  const { done, value } = await reader.read();  // Read next chunk
  if (done) break;                               // No more chunks
  downloadedBytes += value.length;
  progressCallback(downloadedBytes);             // Update progress bar
  // Chunks arrive over time: 64KB, 64KB, 64KB, ...
}

// ── BATCH REQUEST ────────────────────────────────────────────
// Send multiple items, get multiple results in ONE request
// Best for: Efficiency when you have many items to process
const results = await fetch('/validate-batch', {
  method: 'POST',
  body: JSON.stringify({ 
    kits: [kit1, kit2, kit3]  // Send 3 kits at once
  })
});
// Response: { results: [result1, result2, result3] }
// More efficient than 3 separate requests (less network overhead)
```

**Why this pattern matters**

Request-response is:

- Simple: Easy to understand and implement
- Debuggable: Inspect individual requests (browser DevTools, curl)
- Testable: Mock Engine responses for Sketchpad tests
- Cacheable: Same kit validation = same result
- Scalable: Stateless enables multiple Engine instances

**What it enables**

- Clear conversation structure (Sketchpad asks, Engine answers)
- Easy debugging and logging (every request/response logged)
- Timeout handling (give up after 30 seconds)
- Retry logic (try again if network fails)
- Caching (don't re-validate unchanged kit)
- Load balancing (multiple Engine servers)

**What it limits**

- Latency per request (round-trip time)
- Server can't initiate (Engine can't push updates)
- Polling for updates is inefficient (need WebSocket for collab)
- Connection overhead (mitigated by HTTP/2)
- Not ideal for real-time (Y.js uses WebSocket instead)
- Request limits and throttling

---

#### 6.7 APIs: Structured Communication

**Plain explanation**

An API (Application Programming Interface) is a contract: if you send this kind of request, you'll get this kind of response. APIs define what functions are available, what inputs they expect, and what outputs they produce.

semio's Engine has an API that means other programs (Sketchpad, Grasshopper, VS Code) can validate kits, compute placements, and transform designs—all without knowing how Engine works internally. APIs turn services into building blocks.

**Technical explanation**

**semio API types**:

**Web APIs (HTTP)**: Engine's REST endpoints

```
# ============================================================
# EXAMPLE 1: WEB APIs (HTTP ENDPOINTS)
# ============================================================
# Web APIs are accessed over the network using HTTP.
# Every endpoint has a METHOD + URL that defines what it does.

GET  /kit/metabolism      → Kit JSON        # Retrieve the Metabolism kit
POST /validate           → ValidationResult # Send kit, get problems back
POST /place              → PlacementResult  # Compute 3D positions
```

**Library APIs (Function calls)**: Core domain functions

```typescript
// ============================================================
// EXAMPLE 2: LIBRARY API (TypeScript)
// ============================================================
// Library APIs are functions you call directly in your code.
// No network - the code runs in YOUR process.
// This is how Sketchpad uses semio's domain logic.

// From js/semio/semio.ts - TypeScript API

import { Kit, validateKit, applyKitDiff } from '@semio/js';

// Each function is an "API" - a defined interface to functionality
const kit = loadKit('./metabolism.zip');      // Load API: file → Kit object
const result = validateKit(kit);              // Validation API: Kit → problems
const newKit = applyKitDiff(kit, diff);       // Diff API: apply changes
// These functions define WHAT you can do, not HOW it's done internally
```

```python
# ============================================================
# EXAMPLE 3: LIBRARY API (Python)
# ============================================================
# Python Engine exposes the SAME concepts as TypeScript.
# Different language, same API design patterns.

# From py/engine/engine.py - Python API

from semio import Kit, validate, place_pieces

kit = Kit.from_file('metabolism.zip')         # Load API: same concept!
result = validate(kit)                        # Validation API: same concept!
placements = place_pieces(design)             # Placement API: Python-only
# The API provides consistency across languages
```

```csharp
// ============================================================
// EXAMPLE 4: LIBRARY API (C#)
// ============================================================
// C# in Grasshopper uses the same domain API patterns.
// Architects using Grasshopper access the SAME kit concepts.

// From net/Semio/Semio.cs - C# API

using Semio;

var kit = Kit.FromFile("metabolism.zip");     // Load API: C# syntax
var result = Validator.Validate(kit);         // Validation API: C# style
var planes = Placer.ComputePlanes(design);    // Placement API: 3D positioning
// API CONSISTENCY: Same operations, adapted to each language's idioms
```

**System APIs (OS services)**: File access

```typescript
// ============================================================
// EXAMPLE 5: SYSTEM API (Operating System)
// ============================================================
// System APIs let you talk to the operating system.
// The OS provides services like file access, networking, etc.

// Node.js file system API used by CLI
import { readFile, writeFile } from 'fs/promises';

// readFile is a SYSTEM API - it asks the OS to read a file
const kitData = await readFile('kit.json', 'utf8');

// writeFile is a SYSTEM API - it asks the OS to write a file
await writeFile('kit.json', JSON.stringify(kit));

// The OS handles the actual disk I/O
// Your code just calls the API
```

**Engine API design**:

| Aspect         | semio Engine Choice                           |
|----------------|-----------------------------------------------|
| Endpoints      | Resource-based: `/kit`, `/validate`, `/place` |
| Methods        | HTTP verbs: GET, POST, PUT, DELETE            |
| Request format | JSON body with kit/design structure           |
| Response format| JSON with result or error details             |
| Authentication | None (local dev) or JWT (production)          |
| Error handling | HTTP status codes + error JSON                |
| Versioning     | URL path: `/v1/validate`, `/v2/validate`      |

**API documentation in semio**:

```yaml
# From openapi/schema.json (OpenAPI/Swagger format)
paths:
  /validate:
    post:
      summary: Validate a kit
      requestBody:
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/Kit'
      responses:
        '200':
          description: Validation successful
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/ValidationResult'
        '400':
          description: Invalid kit format
```

```graphql
# From graphql/semio/schema.graphql (GraphQL schema)
type Query {
  kit(id: ID!): Kit
  validateKit(kit: KitInput!): ValidationResult!
}

type Kit {
  name: String!
  types: [Type!]!
  designs: [Design!]!
}
```

**Why APIs are essential**

APIs enable semio's multi-language architecture:

- TypeScript Sketchpad calls Python Engine via HTTP API
- Grasshopper C# uses the same domain API patterns
- Go CLI wraps operations for command-line access
- VS Code extension calls repo CLI API
- Future mobile app would use same Engine API

**What it enables**

- Building on semio services (Grasshopper uses Engine)
- Separation of Sketchpad (frontend) and Engine (backend)
- Multiple clients, one Engine (Sketchpad, CLI, Grasshopper)
- Ecosystem development (third-party integrations)
- Programmatic access (scripting with Python API)
- Testing in isolation (mock API responses)

**What it limits**

- API design is hard (must anticipate uses)
- Breaking changes affect all consumers (Sketchpad, Grasshopper)
- Rate limits and quotas (future production deployment)
- Dependency on Engine availability
- Authentication complexity (JWT, session management)
- Documentation must be maintained across languages

---

#### 6.8 REST: A Common Pattern

**Plain explanation**

REST (Representational State Transfer) is a style for designing web APIs. It treats everything as resources (kits, types, designs) with standard operations (create, read, update, delete). You manipulate resources using HTTP methods on URLs.

REST is simple, widely understood, and works well with HTTP. semio's Engine follows REST conventions for its HTTP API.

**Technical explanation**

**REST principles in semio Engine**:

**Resources**: Things with URLs

```
# ============================================================
# EXAMPLE 1: REST RESOURCES AS URLs
# ============================================================
# In REST, everything is a "resource" with a unique URL.
# URLs form a HIERARCHY that mirrors your data structure.
# Think of it like folders on a computer.

/kits                          # Collection: ALL kits
/kits/metabolism               # Single kit: "Metabolism" 
/kits/metabolism/types         # Sub-collection: types IN that kit
/kits/metabolism/designs       # Sub-collection: designs IN that kit
/kits/metabolism/types/capsule # Single type: "Capsule" IN that kit
/kits/metabolism/designs/tower # Single design: "Tower" IN that kit

# THE URL TELLS YOU WHAT YOU'RE WORKING WITH
# No need for special "get kit" or "list types" endpoints
# The URL structure IS the organization
```

**HTTP methods map to operations**:

| Method | semio Endpoint                | Operation        | Idempotent? |
|--------|-------------------------------|------------------|-------------|
| GET    | `/kits/metabolism`            | Read kit         | Yes         |
| POST   | `/kits`                       | Create new kit   | No          |
| PUT    | `/kits/metabolism`            | Replace kit      | Yes         |
| PATCH  | `/kits/metabolism`            | Partial update   | No          |
| DELETE | `/kits/metabolism`            | Delete kit       | Yes         |

**Stateless**: Each request contains all needed context

```python
# ============================================================
# EXAMPLE 2: STATELESS REST REQUESTS
# ============================================================
# REST is STATELESS - every request includes EVERYTHING needed.
# The server doesn't remember previous requests.
# This makes scaling easy: any server can handle any request.

# From py/engine/engine.py

@app.post("/validate")
async def validate_kit(kit: KitInput) -> ValidationResult:
    # The FULL kit is in this request
    # Server doesn't remember "what kit are we working on?"
    # Every request is independent and complete
    return validate(Kit.from_input(kit))

# WHY STATELESS?
# - Load balancing: request 1 → server A, request 2 → server B
# - Crash recovery: server restart doesn't lose session
# - Caching: same request always gives same response
# - Simplicity: no session management needed
```

**Representations**: Resources have JSON representations

```json
// ============================================================
// EXAMPLE 3: JSON REPRESENTATIONS
// ============================================================
// When you GET a resource, you get its JSON "representation"
// This is how the resource looks as data

GET /kits/metabolism

{
  "name": "Metabolism",           // The kit's name
  "version": "1.0.0",             // Version number
  "types": [                      // Array of types IN this kit
    {"guid": "abc-123", "name": "Capsule"},
    {"guid": "def-456", "name": "Frame"}
  ],
  "designs": [                    // Array of designs IN this kit
    {"guid": "ghi-789", "name": "Nakagin Tower"}
  ]
}
// The JSON IS the "representation" of the kit resource
```

**HATEOAS**: Responses include links to related resources

```json
// ============================================================
// EXAMPLE 4: HATEOAS (Links to Related Resources)
// ============================================================
// HATEOAS = Hypermedia As The Engine Of Application State
// Fancy way of saying: responses include LINKS to related things
// Client doesn't need to know URLs - server provides them!

GET /kits/metabolism

{
  "name": "Metabolism",
  "_links": {                              // Related resources
    "self": "/kits/metabolism",            // This resource's URL
    "types": "/kits/metabolism/types",     // Where to get its types
    "designs": "/kits/metabolism/designs", // Where to get its designs
    "validate": "/kits/metabolism/validate", // How to validate
    "download": "/kits/metabolism.zip"     // Where to download full kit
  }
}
// Client can NAVIGATE by following links
// Like clicking hyperlinks on a webpage
```

**RESTful Engine implementation**:

```python
# ============================================================
# EXAMPLE 5: COMPLETE REST API IMPLEMENTATION
# ============================================================
# FastAPI makes it easy to build REST APIs in Python.
# Each endpoint follows REST conventions.
# HTTP methods (GET, POST, PUT, DELETE) map to operations.

# From py/engine/engine.py

from fastapi import FastAPI, HTTPException
from fastapi.responses import JSONResponse, FileResponse

app = FastAPI()

# ── GET: Read a resource (safe, cacheable) ────────────────────
# GET never changes data - safe to call multiple times
@app.get("/kits/{kit_id}")
async def get_kit(kit_id: str) -> JSONResponse:
    kit = load_kit(kit_id)              # Find the kit
    if not kit:
        raise HTTPException(404, "Kit not found")  # 404 = doesn't exist
    return JSONResponse(content=kit.to_dict())     # 200 = here it is

# ── POST: Create a new resource (not idempotent) ──────────────
# POST creates something new - calling twice = two resources
@app.post("/kits")
async def create_kit(kit: KitInput) -> JSONResponse:
    saved = save_kit(Kit.from_input(kit))  # Create and save
    return JSONResponse(content=saved.to_dict(), status_code=201)
    # 201 = "Created" - new resource now exists

# ── PUT: Replace a resource entirely (idempotent) ─────────────
# PUT replaces completely - calling twice has same result
@app.put("/kits/{kit_id}")
async def replace_kit(kit_id: str, kit: KitInput) -> JSONResponse:
    if not kit_exists(kit_id):
        raise HTTPException(404, "Kit not found")
    saved = replace_kit(kit_id, Kit.from_input(kit))  # Replace entirely
    return JSONResponse(content=saved.to_dict())       # 200 = done

# ── DELETE: Remove a resource (idempotent) ────────────────────
# DELETE removes - calling twice is safe (already gone)
@app.delete("/kits/{kit_id}")
async def delete_kit(kit_id: str) -> JSONResponse:
    if not kit_exists(kit_id):
        raise HTTPException(404, "Kit not found")
    remove_kit(kit_id)                                 # Delete it
    return JSONResponse(content={"deleted": kit_id})   # 200 = done
```

**Why REST exists**

REST emerged from studying what made the web successful:

- Simple URL-based addressing (every kit has a URL)
- Standard methods everyone understands (GET, POST, etc.)
- Stateless for scalability (Engine can have multiple instances)
- Cacheable responses (GET /kits/metabolism can be cached)

It applies web architecture principles to APIs.

**What it enables**

- Uniform interface (Sketchpad and CLI use same patterns)
- Scalability (stateless Engine)
- Caching (cached kit fetches)
- Visibility (standard methods)
- Tooling (Postman, curl for testing)
- Wide understanding (every developer knows REST)

**What it limits**

- Multiple requests for related data (get kit, then types, then connectors)
- Over-fetching (get entire kit when only need one type)
- No standard for complex operations (placement is POST /place, not RESTful)
- Versioning challenges (/v1/ vs /v2/)
- Sometimes awkward for non-CRUD operations (validation isn't a resource)
- No real-time updates (need WebSocket for collaboration)

---

#### 6.9 JSON: Data Format for Exchange

**Plain explanation**

JSON (JavaScript Object Notation) is a way to write data as text. It looks like JavaScript code and is easy for both humans and computers to read. semio uses JSON to send and receive data between all its components.

JSON is like a universal language for data exchange—TypeScript, Python, C#, and Go can all read and write it.

**Technical explanation**

**JSON syntax for semio data**:

```json
// ============================================================
// EXAMPLE 1: JSON STRUCTURE FOR A KIT
// ============================================================
// JSON has a very simple syntax with just a few rules:
// - Curly braces {} for objects (key-value pairs)
// - Square brackets [] for arrays (lists)
// - Strings in double quotes "like this"
// - Numbers, booleans, null without quotes

{
  "name": "Metabolism",           // String: kit name
  "version": "1.0.0",             // String: semantic version
  "types": [                      // Array: list of type objects
    {                             // Object: each type is an object
      "guid": "abc-123-def-456",  // String: unique identifier
      "name": "Capsule",          // String: human-readable name
      "connectors": [             // Array: nested array of connectors
        {
          "id": "bottom",                           // String
          "point": {"x": 0, "y": 0, "z": 0},        // Object: 3D point
          "direction": {"x": 0, "y": 0, "z": -1}    // Object: 3D vector
        }
      ],
      "models": []                // Array: empty arrays are valid
    }
  ],
  "designs": [],                  // Array: empty for now
  "isVirtual": false,             // Boolean: true or false
  "canScale": true,               // Boolean: true or false  
  "canMirror": false              // Boolean: no quotes!
}
// NOTE: Real JSON doesn't allow comments - these are for explanation
```

**JSON types used in semio**:

| JSON Type | semio Usage                                  |
|-----------|----------------------------------------------|
| Object    | `{}` Kit, Type, Design, Piece, Connection    |
| Array     | `[]` types list, connectors list, pieces     |
| String    | `"Capsule"` names, guids, descriptions       |
| Number    | `0.5` coordinates, angles, scale factors     |
| Boolean   | `true/false` isVirtual, canScale, canMirror  |
| Null      | `null` optional fields like parent           |

**Parsing/serializing in each language**:

```typescript
// ============================================================
// EXAMPLE 2: JSON IN TYPESCRIPT
// ============================================================
// JSON stands for "JavaScript Object Notation"
// TypeScript/JavaScript has NATIVE support for JSON.

// TypeScript (js/semio/semio.ts)
import { z } from 'zod';

// PARSING: JSON text → JavaScript object
const kitJson = '{"name": "Metabolism", "types": [...]}';  // Text string
const kitData = JSON.parse(kitJson);  // → JavaScript object
// Now kitData.name === "Metabolism"

// VALIDATION: Zod checks the structure is correct
const kit = KitSchema.parse(kitData);  // Throws if invalid

// SERIALIZING: JavaScript object → JSON text
const outputJson = JSON.stringify(kitToJson(kit), null, 2);
// null = no special replacer
// 2 = indent with 2 spaces (pretty print)
```

```python
# ============================================================
# EXAMPLE 3: JSON IN PYTHON
# ============================================================
# Python has a built-in 'json' module.
# Pydantic adds validation on top of parsing.

# Python (py/engine/engine.py)
import json
from pydantic import BaseModel

# PARSING: JSON text → Python dict
kit_json = '{"name": "Metabolism", "types": [...]}'  # Text string
kit_data = json.loads(kit_json)  # → Python dictionary
# Now kit_data["name"] == "Metabolism"

# VALIDATION: Pydantic checks the structure
kit = Kit.model_validate(kit_data)  # Raises if invalid

# SERIALIZING: Python object → JSON text
output_json = json.dumps(kit.model_dump(), indent=2)
# indent=2 = pretty print with 2 spaces
```

```csharp
// ============================================================
// EXAMPLE 4: JSON IN C#
// ============================================================
// C# uses System.Text.Json (modern) or Newtonsoft.Json (legacy).
// JsonSerializer handles parsing and serializing.

// C# (net/Semio/Semio.cs)
using System.Text.Json;

// PARSING: JSON text → C# object
var kitJson = "{\"name\": \"Metabolism\", \"types\": [...]}";
var kit = JsonSerializer.Deserialize<Kit>(kitJson);
// Generic type <Kit> tells what kind of object to create
// Now kit.Name == "Metabolism"

// SERIALIZING: C# object → JSON text
var outputJson = JsonSerializer.Serialize(kit, new JsonSerializerOptions { 
    WriteIndented = true   // Pretty print
});
```

```go
// ============================================================
// EXAMPLE 5: JSON IN GO
// ============================================================
// Go has encoding/json in the standard library.
// Uses Marshal (serialize) and Unmarshal (parse) naming.

// Go (go/semio/semio.go)
import "encoding/json"

// PARSING: JSON text → Go struct
kitJson := `{"name": "Metabolism", "types": [...]}`  // Backticks = raw string
var kit Kit                                           // Empty struct
err := json.Unmarshal([]byte(kitJson), &kit)         // Fill it from JSON
// &kit = pointer to kit (so Unmarshal can modify it)
// Now kit.Name == "Metabolism"

// SERIALIZING: Go struct → JSON text
outputJson, err := json.MarshalIndent(kit, "", "  ")
// "" = no prefix
// "  " = indent with 2 spaces
```

**JSON Schema for validation**:

```json
// ============================================================
// EXAMPLE 6: JSON SCHEMA (Defining Valid JSON Structure)
// ============================================================
// JSON Schema is "JSON that describes other JSON"
// It defines what a valid kit looks like.
// Validation tools check if JSON matches the schema.

// From jsonschema/kit.json
{
  "$schema": "http://json-schema.org/draft-07/schema#",  // Schema version
  "type": "object",                 // Kit must be an object {}
  "properties": {                   // Allowed properties:
    "name": { 
      "type": "string",             // name must be a string
      "minLength": 1                // name can't be empty ""
    },
    "version": { 
      "type": "string",             // version must be a string
      "pattern": "^\\d+\\.\\d+\\.\\d+$"  // Must match X.Y.Z format
    },
    "types": {
      "type": "array",              // types must be an array []
      "items": { "$ref": "#/definitions/Type" }  // Each item = Type
    }
  },
  "required": ["name", "types"]     // These fields MUST exist
}

// WHAT THE SCHEMA CATCHES:
// ✗ { "name": 123 }           → name must be string
// ✗ { "version": "1.0" }      → wrong version format
// ✗ { "types": [...] }        → missing required "name"
// ✓ { "name": "Test", "types": [] }  → valid!
```

**Why JSON became standard**

Before JSON:
- XML: Verbose, complex (`<type><name>Capsule</name></type>`)
- Custom formats: Non-portable, hard to debug

JSON advantages for semio:
- Simple syntax (familiar to JavaScript developers)
- Human-readable (can inspect kit files)
- Widely supported (all four semio languages)
- No schema required for flexibility (but optional JSON Schema)
- Lightweight (smaller than XML)

**What it enables**

- Universal data exchange (Sketchpad ↔ Engine ↔ Grasshopper)
- Easy debugging (read kit JSON in any text editor)
- Cross-language compatibility (TypeScript, Python, C#, Go)
- Flexible structure (add new fields without breaking)
- Native JavaScript support (Sketchpad frontend)
- Simple parsing (every language has built-in JSON)

**What it limits**

- No date type (semio uses ISO strings: `"2024-01-15T10:30:00Z"`)
- No binary data (model files stored separately, referenced by path)
- No comments (can't document kit JSON inline)
- Limited precision for large numbers (use strings for GUIDs)
- No schema enforcement by default (must add validation layer)
- Larger than binary formats (but compression helps for zip archives)

---

#### 6.10 GraphQL: An Alternative Approach

**Plain explanation**

GraphQL is a query language for APIs. Instead of fixed endpoints returning fixed data (REST), you ask for exactly what you want. "Give me the kit's name and only the type names, nothing else." One request, precisely the data you need.

semio uses GraphQL for the repo CLI queries, enabling flexible data fetching for tickets, policies, and contributions.

**Technical explanation**

**GraphQL in semio**: repo CLI endpoint

```graphql
# ============================================================
# EXAMPLE 1: GRAPHQL QUERY
# ============================================================
# In GraphQL, YOU specify exactly what data you want.
# The response matches your query shape EXACTLY.
# No over-fetching (getting data you don't need).
# No under-fetching (needing multiple requests).

# From graphql/repo/schema.graphql
# Query: "Give me tickets from 2025 with these specific fields"

query {
  repo {
    tickets(year: 2025) {      # Filter: only 2025 tickets
      slug                      # Want: ticket slug
      status                    # Want: open/closed status
      summary                   # Want: one-line summary
      author {                  # Want: author info
        name                    # - author's name
        email                   # - author's email
      }
      # NOT requesting: date, files, lines, etc.
      # → Response won't include those fields!
    }
  }
}
```

Response matches query shape exactly:

```json
// ============================================================
// EXAMPLE 2: GRAPHQL RESPONSE
// ============================================================
// The response has EXACTLY the shape you asked for.
// No more, no less. Perfect for client-side efficiency.

{
  "data": {
    "repo": {                           // repo field
      "tickets": [                      // tickets array
        {
          "slug": "VALIDATION-SYSTEM",  // Only fields you asked for
          "status": "closed",
          "summary": "Implement kit validation",
          "author": {
            "name": "usalu",
            "email": "ueli@semio.design"
          }
        }
      ]
    }
  }
}
```

**GraphQL schema for semio domain**:

```graphql
# ============================================================
# EXAMPLE 3: GRAPHQL SCHEMA (Type Definitions)
# ============================================================
# The schema defines what data EXISTS and HOW to query it.
# Think of it as the API contract.
# ! means required (non-null)

# From graphql/semio/schema.graphql

type Kit {
  guid: ID!           # ! = required, ID = unique identifier
  name: String!       # ! = required
  version: String     # No ! = optional (can be null)
  types: [Type!]!     # [Type!]! = required array of required Types
  designs: [Design!]! # Can be empty array, but can't be null
}

type Type {
  guid: ID!
  name: String!
  connectors: [Connector!]!  # List of connectors
  models: [Model!]!          # List of models
  isVirtual: Boolean!        # true/false, required
  canScale: Boolean!
  canMirror: Boolean!
}

type Design {
  guid: ID!
  name: String!
  pieces: [Piece!]!
  connections: [Connection!]!
}

# ENTRY POINTS: What clients can ask for
type Query {
  kit(guid: ID!): Kit                              # Get one kit by GUID
  validateKit(kit: KitInput!): ValidationResult!   # Validate a kit
  placeDesign(design: DesignInput!): PlacementResult!
}

# MUTATIONS: How clients can change data
type Mutation {
  createKit(input: KitInput!): Kit!       # Create, returns new kit
  updateKit(guid: ID!, input: KitInput!): Kit!  # Update existing
  deleteKit(guid: ID!): Boolean!          # Delete, returns success
}
```

**Query examples for common semio operations**:

```graphql
# ============================================================
# EXAMPLE 4: EFFICIENT QUERIES (No Over-Fetching)
# ============================================================
# GraphQL lets you get exactly what you need in ONE request.

# Get only type names and connector counts
# REST would return EVERYTHING about each type
# GraphQL returns ONLY name and connector ids
query TypeOverview {
  kit(guid: "abc-123") {
    name
    types {
      name
      connectors {
        id       # Only getting id, not point/direction/etc
      }
    }
  }
}

# ============================================================
# EXAMPLE 5: NESTED DATA IN ONE REQUEST
# ============================================================
# Get design with pieces and their placements
# With REST, this might be 3+ separate requests
# With GraphQL, it's ONE request

query DesignWithPlacements {
  kit(guid: "abc-123") {
    designs {
      name
      pieces {
        guid
        type { name }        # Nested: get type name
        plane {
          origin { x y z }   # Nested: get coordinates
          xAxis { x y z }
        }
      }
    }
  }
}

# ============================================================
# EXAMPLE 6: MUTATIONS (Changing Data)
# ============================================================
# Mutations are how you create/update/delete in GraphQL

mutation AddType {
  addType(kitGuid: "abc-123", input: {
    name: "NewModule",
    connectors: [],
    isVirtual: false
  }) {
    guid        # Get back the new GUID
    name        # And name (confirm it was created)
  }
}
```
  }
}
```

**GraphQL resolver implementation**:

```go
// From go/repo/graph/resolver.go

func (r *queryResolver) Kit(ctx context.Context, guid string) (*model.Kit, error) {
    kit := r.KitStore.Load(guid)
    if kit == nil {
        return nil, fmt.Errorf("kit not found: %s", guid)
    }
    return kit.ToGraphQL(), nil
}

func (r *queryResolver) Tickets(ctx context.Context, year *int) ([]*model.Ticket, error) {
    tickets := r.TicketStore.List(year)
    result := make([]*model.Ticket, len(tickets))
    for i, t := range tickets {
        result[i] = t.ToGraphQL()
    }
    return result, nil
}
```

**Why GraphQL was invented**

semio chose GraphQL for repo because:

- VS Code extension needs minimal data (bandwidth for responsiveness)
- Different views need different data shapes (ticket list vs detail)
- REST would require many endpoints or over-fetching
- Strong typing catches errors at query time

**What it enables**

- Fetch exactly needed data (only type names, not full models)
- One request for complex data (kit with types and designs)
- Strong typing with schema (IDE autocomplete)
- Self-documenting API (introspection)
- Real-time subscriptions (future: live collaboration)
- Client-driven queries (VS Code asks for what it needs)

**What it limits**

- Complexity on server (resolvers for each field)
- Caching is harder (no URL-based caching like REST)
- Learning curve (query language to learn)
- N+1 query problems (must use DataLoader pattern)
- Security (must limit query depth and complexity)
- Tooling less mature than REST (but improving)

---

#### 6.11 WebSockets: Real-Time Communication

**Plain explanation**

HTTP is like sending letters—request, wait, response. WebSockets are like a phone call—once connected, both sides can talk anytime. The connection stays open, and either side can send messages instantly.

semio uses WebSockets for real-time collaboration. When you edit a design in Sketchpad, other collaborators see changes instantly via Y.js over WebSocket.

**Technical explanation**

**WebSocket connection in semio**:

```
# ============================================================
# WEBSOCKET CONNECTION LIFECYCLE
# ============================================================
# Unlike HTTP (request → response → done), WebSocket STAYS OPEN.
# Both sides can send messages at ANY time.

1. Sketchpad HTTP request with upgrade header    # "Can we switch to WebSocket?"
2. Liveblocks server agrees to upgrade           # "Yes, let's do it"
3. Connection becomes WebSocket                  # Now it's a persistent channel
4. Bidirectional Y.js sync until close           # Both sides send freely
```

```typescript
// ============================================================
// EXAMPLE 1: ESTABLISHING WEBSOCKET CONNECTION
// ============================================================
// Y.js + Liveblocks provides the WebSocket infrastructure.
// We just connect - they handle the complexity.

// From js/semio/sketchpad/Sketchpad.tsx

import { createClient } from '@liveblocks/client';
import { LiveblocksProvider } from '@liveblocks/yjs';

// Create Liveblocks client (manages WebSocket connections)
const client = createClient({
  publicApiKey: 'pk_live_xxxxx',  // API key for authentication
});

// Create Y.js document for this kit
// Y.js is a CRDT library - handles conflict resolution
const yDoc = new Y.Doc();
const yKit = yDoc.getMap('kit');  // The kit data as a Y.js Map

// CONNECT: Join a "room" - all users editing same kit share this room
const room = client.enter('metabolism-kit', {
  initialPresence: { cursor: null, selection: [] }  // What we share about ourselves
});

// SYNC: Connect Y.js to the room via WebSocket
const provider = new LiveblocksProvider(room, yDoc);

// NOW: Any change to yKit automatically syncs to ALL other users!
// Local edit → Y.js → WebSocket → Liveblocks → WebSocket → Others
```

**Real-time collaboration flow**:

```
# ============================================================
# HOW REAL-TIME SYNC WORKS
# ============================================================
# Multiple users edit the same kit simultaneously.
# Changes propagate instantly via WebSocket.

┌──────────────────┐     WebSocket      ┌─────────────────┐
│   Sketchpad A    │ ←──────────────→   │   Liveblocks    │
│   (Browser 1)    │                    │    Server       │
│                  │                    │                 │
│  yDoc.getMap()   │     Y.js sync      │   Room State    │
│  yKit.set(...)   │ ←──────────────→   │  (holds truth)  │
└──────────────────┘                    └────────┬────────┘
        │                                        │
        │ User A adds a piece                    │ WebSocket
        │ → Syncs to server                      ▼
        │                               ┌─────────────────┐
        │                               │   Sketchpad B   │
        │                               │   (Browser 2)   │
        │                               │                 │
        └─ User A sees their change     │  Piece appears! │
           instantly                    │  (from Y.js)    │
                                        └─────────────────┘
```

**Y.js change propagation**:

```typescript
// ============================================================
// EXAMPLE 2: REACTING TO REMOTE CHANGES
// ============================================================
// When OTHER users make changes, your Y.js doc updates automatically.
// Observers let you react to these changes.

// From js/semio/sketchpad/Sketchpad.tsx

// SUBSCRIBE to remote changes
yKit.observe((event) => {
  // This fires when ANYONE (including yourself) changes yKit
  event.changes.keys.forEach((change, key) => {
    if (change.action === 'add') {
      console.log(`Someone added: ${key}`);
    } else if (change.action === 'update') {
      console.log(`Someone updated: ${key}`);
    } else if (change.action === 'delete') {
      console.log(`Someone deleted: ${key}`);
    }
    // React re-renders automatically because we're subscribed
  });
});

// ============================================================
// EXAMPLE 3: MAKING A CHANGE THAT SYNCS
// ============================================================
// When YOU make a change, it automatically syncs to others.

function addPiece(piece: Piece) {
  const yPieces = yDesign.get('pieces') as Y.Array<Piece>;
  
  // This ONE line triggers a cascade:
  yPieces.push([piece]);
  
  // 1. Local Y.js observer fires (your UI updates)
  // 2. Y.js encodes the change as binary delta
  // 3. WebSocket sends delta to Liveblocks
  // 4. Liveblocks broadcasts to ALL other connected clients
  // 5. Their Y.js decodes the delta
  // 6. Their observers fire
  // 7. Their UIs update
  // All in ~50-100ms!
}
```

**Presence for cursor awareness**:

```typescript
// ============================================================
// EXAMPLE 4: PRESENCE (Seeing Other Users)
// ============================================================
// "Presence" is ephemeral state about each user.
// Cursor position, selection, who's online, etc.

// Show other users' cursors and selections

const presence = room.getPresence();

// BROADCAST your cursor when you move the mouse
function onMouseMove(e: MouseEvent) {
  room.updatePresence({
    cursor: { x: e.clientX, y: e.clientY }  // Your cursor position
  });
  // This sends to all other users via WebSocket
}

// SUBSCRIBE to others' presence
room.subscribe('others', (others) => {
  // "others" is an array of all other connected users
  others.forEach((user) => {
    // Render their cursor if they have one
    if (user.presence.cursor) {
      renderCursor(user.id, user.presence.cursor);  // Show their cursor
    }
    // Highlight what they've selected
    if (user.presence.selection) {
      highlightSelection(user.id, user.presence.selection);
    }
  });
});
// Result: You see everyone's cursors moving in real-time!
```

**Characteristics of WebSocket in semio**:

| Feature         | semio Usage                                    |
|-----------------|------------------------------------------------|
| Full-duplex     | Changes flow both directions simultaneously    |
| Persistent      | Single connection for entire editing session   |
| Low latency     | No HTTP overhead per update                    |
| Binary support  | Y.js sends efficient binary updates            |
| Framing         | Messages have clear boundaries                 |

**Alternatives considered**:

- **HTTP polling**: Too slow for real-time (100ms delay feels sluggish)
- **Server-Sent Events (SSE)**: Server push only, can't send updates
- **WebRTC**: Peer-to-peer, more complex (considered for future)

**Why WebSockets matter**

HTTP limitations for collaboration:
- Client must initiate (Engine can't push updates)
- Connection overhead per request
- Polling wastes resources

WebSockets enable:
- Instant server-to-client updates (see collaborator's changes)
- Efficient bidirectional communication (send and receive)
- Real-time user experience (feels like local editing)

**What it enables**

- Real-time collaborative editing (multiple designers on same kit)
- Live cursor and selection awareness
- Instant conflict resolution (Y.js CRDT)
- Live notifications (type added, design modified)
- Efficient updates (only deltas, not full kit)
- Undo/redo that respects others' changes

**What it limits**

- Connection state to manage (reconnection logic)
- Scaling requires infrastructure (Liveblocks handles this)
- Firewalls/proxies may interfere (fallback to polling)
- No automatic reconnection (must implement)
- Memory for connection state (one socket per collaborator)
- Different programming model than request-response

---

## Part 3: Real-World Systems

### Chapter 7: How Modern Systems Work

#### 7.1 Frontend: What Users See

**Plain explanation**

The frontend is everything that happens on the user's device—the browser, the desktop app, the Grasshopper canvas. It's the visual interface, the buttons they click, the designs they create, the responses they see. The frontend is where architects/designers and software meet.

semio's Sketchpad is a React-based frontend that runs in the browser, presenting 3D designs, handling user interactions, and synchronizing with collaborators.

**Technical explanation**

**semio frontend architecture (Sketchpad)**:

```
┌──────────────────────────────────────────────────────────────────┐
│                      Sketchpad Frontend                           │
├──────────────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────────────────┐ │
│  │                    React Components                          │ │
│  │  ┌───────────┐  ┌───────────┐  ┌───────────┐  ┌───────────┐ │ │
│  │  │  Navbar   │  │   Canvas  │  │  Panels   │  │  Footer   │ │ │
│  │  └─────┬─────┘  └─────┬─────┘  └─────┬─────┘  └─────┬─────┘ │ │
│  └────────┼──────────────┼──────────────┼──────────────┼───────┘ │
│           │              │              │              │         │
│  ┌────────┼──────────────┼──────────────┼──────────────┼───────┐ │
│  │        ▼              ▼              ▼              ▼       │ │
│  │               XState State Machine                          │ │
│  │      (navigation, selection, hover, tools, panels)          │ │
│  └─────────────────────────┬───────────────────────────────────┘ │
│                            │                                     │
│  ┌─────────────────────────┼───────────────────────────────────┐ │
│  │            ▼            │                                   │ │
│  │      Y.js + Liveblocks  │   Kit data (collaborative)       │ │
│  │   ┌─────────────────────┴─────────────────────────┐         │ │
│  │   │  yKit.types  │  yKit.designs  │  yKit.files   │         │ │
│  │   └───────────────────────────────────────────────┘         │ │
│  └─────────────────────────────────────────────────────────────┘ │
│                                                                   │
│  ┌─────────────────────────────────────────────────────────────┐ │
│  │                     Three.js Scene                           │ │
│  │   ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐        │ │
│  │   │  Models │  │  Pieces │  │  Grid   │  │ Camera  │        │ │
│  │   └─────────┘  └─────────┘  └─────────┘  └─────────┘        │ │
│  └─────────────────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────────────────┘
```

**Frontend technologies in Sketchpad**:

```tsx
// ============================================================
// EXAMPLE 1: REACT COMPONENT STRUCTURE
// ============================================================
// React is a library for building user interfaces from components.
// A "component" is a reusable piece of UI (like a LEGO brick for UIs).
// Components nest inside each other to form the complete interface.

// From js/semio/sketchpad/Sketchpad.tsx

export function Sketchpad({ providers }: SketchpadProps) {
  return (
    // SketchpadProvider gives all children access to shared state
    <SketchpadProvider>
      <Navbar />          {/* Top bar: navigation, breadcrumbs, panel toggles */}
      <Canvas>            {/* Main working area (holds windows) */}
        <Scene3D />       {/* Three.js 3D rendering of the design */}
        <Diagram />       {/* 2D force-directed graph of connections */}
        <Table />         {/* Spreadsheet-like tabular data view */}
      </Canvas>
      <Panels>            {/* Side panels that slide in/out */}
        <Workbench />     {/* Browse types and designs in the kit */}
        <Details />       {/* Edit properties of selected item */}
        <Settings />      {/* User preferences (theme, language) */}
      </Panels>
      <Footer />          {/* Bottom bar: status info, quick actions */}
    </SketchpadProvider>
  );
  // HIERARCHY: Each component is responsible for ONE thing
  // This makes the code easier to understand and maintain
}
```

**CSS with Tailwind in Sketchpad**:

```tsx
// ============================================================
// EXAMPLE 2: TAILWIND CSS STYLING
// ============================================================
// Tailwind CSS uses "utility classes" - small, single-purpose classes.
// Instead of: .my-button { height: 40px; display: flex; }
// You write: className="h-small flex"
// This keeps styles close to the component that uses them.

// From js/semio/sketchpad/elements.tsx

export function Action({ icon, text, onClick }: ActionProps) {
  return (
    <button
      onClick={onClick}
      className="
        h-small w-small           /* SIZE: 5-unit height and width */
        flex items-center gap-1   /* LAYOUT: flexbox, centered, 1-unit gap */
        hover:bg-active           /* INTERACTION: highlight on hover */
        border border-element     /* BORDER: thin border with theme color */
        text-tiny                 /* TEXT: 3-unit font size */
      "
    >
      {icon && <span className="h-tiny w-tiny">{icon}</span>}
      {text}
    </button>
  );
}
// SEMANTIC COLORS: We use names like "active", "element" instead of "#3B82F6"
// This means themes (light/dark) work automatically!
```

**Three.js for 3D rendering**:

```tsx
// ============================================================
// EXAMPLE 3: THREE.JS 3D RENDERING
// ============================================================
// Three.js is a library for 3D graphics in the browser.
// @react-three/fiber wraps Three.js for use with React components.
// Each <group>, <mesh>, <Sphere> is a 3D object in the scene.

// From js/semio/sketchpad/Design.tsx

import { Canvas, useThree } from '@react-three/fiber';
import { OrbitControls, useGLTF } from '@react-three/drei';

function PieceModel({ piece, type }: PieceModelProps) {
  // Load the 3D model file (GLTF format)
  const model = useGLTF(type.models[0]?.url ?? '');
  
  // Compute where this piece sits in 3D space
  const plane = computePlane(piece);
  
  return (
    // <group> is a container for 3D objects (like a folder for files)
    <group
      // Position in 3D: [x, y, z]
      // Note: semio uses Y-forward, Three.js uses Z-forward → swap Y and Z
      position={[plane.origin.x, plane.origin.z, -plane.origin.y]}
      // Rotation as quaternion (computed from plane axes)
      rotation={quaternionFromPlane(plane)}
    >
      {/* The actual 3D model from the file */}
      <primitive object={model.scene.clone()} />
      
      {/* Show connectors as small spheres */}
      {type.connectors.map(c => (
        <Sphere key={c.id} position={[c.point.x, c.point.z, -c.point.y]} />
      ))}
    </group>
  );
}
// GPU ACCELERATION: Three.js uses WebGL which runs on the graphics card
// This is why you can render complex 3D scenes smoothly in a browser
```

**Frontend concerns in Sketchpad**:

| Concern           | semio Implementation                                |
|-------------------|-----------------------------------------------------|
| UI design         | React components with Tailwind CSS                  |
| Responsive layout | CSS grid, flexbox, panel resizing                   |
| Accessibility     | ARIA labels, keyboard navigation, tooltips          |
| Performance       | React.memo, useMemo, Y.js efficient updates         |
| State management  | XState machine + Y.js for collaboration             |
| API integration   | fetch() to Engine, Y.js to Liveblocks               |
| 3D rendering      | Three.js via @react-three/fiber                     |

**Why Sketchpad frontend is built separately**

Separation enables:

- Specialized skills (UI/UX designers, frontend developers)
- Different update cycles (UI can evolve without Engine changes)
- Multiple frontends (Browser Sketchpad, Electron Desktop, future mobile)
- Testing in isolation (mock Engine responses)
- Clear boundaries (frontend = presentation, backend = logic)

**What it enables**

- Rich, interactive 3D design experience
- Immediate feedback (see changes as you make them)
- Offline capability (Y.js local persistence)
- Platform-specific optimization (browser vs desktop)
- Rapid UI iteration (hot reload during development)
- User-centered development (focus on designer needs)

**What it limits**

- Runs on user's device (varied GPU capabilities)
- Must handle unreliable networks (reconnection logic)
- Security (JavaScript is visible to users)
- Cross-browser compatibility (Three.js WebGL differences)
- Performance on low-end devices (3D is demanding)
- Bundle size concerns (lazy load heavy components)

---

#### 7.2 Backend: The Hidden Brain

**Plain explanation**

The backend is the server side—the programs that run on servers, invisible to users. It handles complex computations, data validation, and 3D placement algorithms. When you validate a kit, the Engine backend checks all constraints. When you place pieces, the Engine computes 3D transformations.

The Engine is semio's backend—the authoritative place for computation-heavy logic.

**Technical explanation**

**semio backend architecture (Engine)**:

```
┌──────────────────────────────────────────────────────────────────┐
│                        Engine Backend                             │
│                    (py/engine/engine.py)                          │
├──────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │                   FastAPI Application                       │  │
│  │                                                             │  │
│  │  @app.post("/validate")     →  validate_kit()               │  │
│  │  @app.post("/place")        →  place_pieces()               │  │
│  │  @app.get("/kit/{id}")      →  get_kit()                    │  │
│  │  @app.get("/health")        →  health_check()               │  │
│  └───────────────────────────────┬────────────────────────────┘  │
│                                  │                                │
│  ┌───────────────────────────────┼────────────────────────────┐  │
│  │              Domain Logic     │                             │  │
│  │  ┌─────────────┐  ┌──────────┴──────┐  ┌─────────────────┐ │  │
│  │  │  Validation │  │    Placement    │  │   Transformation │ │  │
│  │  │  - GUIDs    │  │  - Graph walk   │  │   - Planes       │ │  │
│  │  │  - Names    │  │  - Connections  │  │   - Rotations    │ │  │
│  │  │  - Refs     │  │  - Hierarchy    │  │   - Translations │ │  │
│  │  └─────────────┘  └─────────────────┘  └─────────────────┘ │  │
│  └────────────────────────────────────────────────────────────┘  │
│                                                                   │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │                    Data Layer                               │  │
│  │  ┌─────────────┐  ┌─────────────────┐  ┌─────────────────┐ │  │
│  │  │   Pydantic  │  │     SQLite      │  │   File Storage  │ │  │
│  │  │   Models    │  │   (kit.db)      │  │   (models/)     │ │  │
│  │  └─────────────┘  └─────────────────┘  └─────────────────┘ │  │
│  └────────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────────┘
```

**Engine server implementation**:

```python
# From py/engine/engine.py

from fastapi import FastAPI, HTTPException  # FastAPI = Python web framework for APIs
from pydantic import BaseModel                # Pydantic = data validation library
import numpy as np                             # NumPy = numerical/math library for 3D

# ============================================================
# EXAMPLE 1: BACKEND SERVER APPLICATION
# ============================================================
# Purpose: Create the "brain" of semio that handles complex computations
# The backend runs on a SERVER - code that clients (browsers, apps) call remotely.
# FastAPI creates HTTP endpoints that accept JSON and return JSON.

app = FastAPI(title="semio Engine", version="1.0.0")  # Create the application instance

# ── VALIDATION ENDPOINT ─────────────────────────────────────
# @app.post("/validate") means: when someone sends a POST request to /validate,
# run this function. POST is used because we're sending data (the kit) to check.

@app.post("/validate")
async def validate_kit(kit: KitInput) -> ValidationResult:
    # async = this function can pause for I/O without blocking other requests
    # kit: KitInput = FastAPI automatically parses JSON body into this type
    # -> ValidationResult = the function returns this type (also becomes JSON)
    
    kit_obj = Kit.from_input(kit)  # Convert JSON-based input to internal object
    
    # Run all validation constraints - each returns a list of problems found
    problems = []                                         # Start with empty list
    problems.extend(check_guid_uniqueness(kit_obj))       # All GUIDs unique?
    problems.extend(check_name_uniqueness(kit_obj))       # Names unique in scope?
    problems.extend(check_connector_references(kit_obj))  # Connectors exist?
    problems.extend(check_connection_validity(kit_obj))   # Connections make sense?
    
    # Return validation result - FastAPI converts this object to JSON automatically
    return ValidationResult(
        valid=len(problems) == 0,  # valid=True if problems list is empty
        problems=problems           # Include the list of problems found
    )

# ── PLACEMENT ALGORITHM ENDPOINT ────────────────────────────
# This computes WHERE each piece goes in 3D space.
# Heavy 3D math runs on the server (fast), not browser (slow).

@app.post("/place")
async def place_pieces(design: DesignInput) -> PlacementResult:
    design_obj = Design.from_input(design)  # Parse JSON into Design object
    
    # STEP 1: Start with FIXED pieces (they have planes defined directly)
    placements = {}                              # Dictionary: piece GUID → Plane
    for piece in get_fixed_pieces(design_obj):   # Loop over pieces with explicit planes
        placements[piece.guid] = piece.plane     # Store the given plane as-is
    
    # STEP 2: BFS (Breadth-First Search) through the connection graph
    # For each connected piece, compute its plane from its parent's plane
    for piece, parent, connection in traverse_connections(design_obj):
        parent_plane = placements[parent.guid]   # Look up parent's computed plane
        piece_plane = compute_connected_plane(   # Calculate THIS piece's plane
            parent_plane, connection, parent, piece
        )
        placements[piece.guid] = piece_plane     # Store the result
    
    return PlacementResult(placements=placements)  # Return all computed planes
```

**Backend computation examples**:

```python
# ============================================================
# EXAMPLE 2: 3D TRANSFORMATION MATHEMATICS
# ============================================================
# Purpose: Calculate where a CHILD piece sits based on its PARENT piece
# This is the core "placement algorithm" - pure 3D geometry math.
# 
# Imagine LEGO bricks: when you attach one brick to another,
# the child's position depends on: where the parent is, and how they connect.

def compute_connected_plane(
    parent_plane: Plane,         # WHERE is the parent piece in 3D space?
    connection: Connection,      # HOW are they connected? (gap, rotation, etc.)
    parent_piece: Piece,         # The parent piece (has connectors)
    child_piece: Piece           # The child piece (has connectors)
) -> Plane:                      # Returns: WHERE the child piece ends up
    
    # STEP 1: Get the CONNECTOR points on each piece
    # Connectors are like "sockets" - predefined attachment points
    parent_connector = get_connector(parent_piece, connection.connected)   # Parent's socket
    child_connector = get_connector(child_piece, connection.connecting)    # Child's socket
    
    # STEP 2: Build the TRANSLATION vector (how far to move)
    # shift = left/right (X), gap = forward/back (Y), rise = up/down (Z)
    translation = np.array([      # NumPy array for vector math
        connection.shift,         # X offset (horizontal side-to-side)
        connection.gap,           # Y offset (depth forward-backward)  
        connection.rise           # Z offset (vertical up-down)
    ])
    
    # STEP 3: Build the ROTATION matrix (how to turn/twist)
    # rotation = spin around Y, turn = spin around Z, tilt = spin around X
    rotation = rotation_matrix(
        connection.rotation,      # Rotation around Y axis (like turning a doorknob)
        connection.turn,          # Rotation around Z axis (like spinning on a chair)
        connection.tilt           # Rotation around X axis (like nodding your head)
    )
    
    # STEP 4: COMBINE translation + rotation to get final position
    # This is linear algebra: matrix multiplication, coordinate transforms
    child_plane = transform_plane(
        parent_plane,             # Start from parent's position
        parent_connector,         # Adjust for parent's connector location
        child_connector,          # Adjust for child's connector location
        translation,              # Apply the gap/shift/rise offsets
        rotation                  # Apply the rotation/turn/tilt twists
    )
    
    return child_plane  # The Plane (origin + axes) where child piece sits in 3D space
```

**Backend languages in semio**:

| Language   | Component      | Why Chosen                                    |
|------------|----------------|-----------------------------------------------|
| Python     | Engine         | NumPy for 3D math, FastAPI for async HTTP     |
| C#         | Grasshopper    | Required by Rhino/.NET ecosystem              |
| Go         | repo CLI       | Fast compilation, single binary distribution  |
| TypeScript | Domain logic   | Shared with frontend for consistency          |

**Why Engine needs to be separate**

Backend separation provides:

- Security (validation runs on trusted server, not client)
- Scalability (add Engine instances for heavy loads)
- Single source of truth (authoritative placement algorithm)
- Shared logic for multiple clients (Sketchpad, Grasshopper, CLI)
- Integration point for AI services (future)

**What it enables**

- Complex 3D computations (GPU on server)
- Validated placement algorithms
- Multiple client support (same Engine for all)
- Horizontal scaling (more servers for more users)
- Background processing (large kit analysis)
- System integration (future: AI, BIM export)

**What it limits**

- Network latency for every computation
- Complexity of distributed systems
- Deployment and operations burden
- Must handle concurrent access (async Python)
- Security responsibility (input validation)
- Costs for servers and infrastructure

---

#### 7.3 Web Applications: Programs in Browsers

**Plain explanation**

A web application runs in your browser—no installation needed. Sketchpad is a web app. You navigate to a URL, and the complete design environment loads and runs.

Web apps combine the reach of the web (any device with a browser) with the interactivity of desktop apps. semio's Sketchpad proves you can build a full 3D CAD tool as a web application.

**Technical explanation**

**Sketchpad web application architecture**:

```
┌────────────────────────────────────────────────────────────────────┐
│                  Sketchpad as Single-Page App (SPA)                │
├────────────────────────────────────────────────────────────────────┤
│                                                                    │
│  1. Initial Load:                                                  │
│     Browser → Vite server → index.html + main.tsx bundle           │
│                                                                    │
│  2. React takes over:                                              │
│     - Renders Sketchpad UI                                         │
│     - XState manages navigation (no page reloads)                  │
│     - React Router handles URL changes                             │
│                                                                    │
│  3. Data flow:                                                     │
│     ┌──────────┐     ┌─────────────┐     ┌──────────────┐         │
│     │ Y.js Doc │ ←→  │  Liveblocks │ ←→  │ Other Users  │         │
│     └────┬─────┘     └─────────────┘     └──────────────┘         │
│          │                                                         │
│          ▼                                                         │
│     ┌──────────────────────────────────────────────────┐          │
│     │              React Components                     │          │
│     │  Navbar │ Canvas │ Scene │ Diagram │ Panels      │          │
│     └──────────────────────────────────────────────────┘          │
│          │                                                         │
│          ▼                                                         │
│     ┌──────────────────────────────────────────────────┐          │
│     │           Engine API (when needed)                │          │
│     │         POST /validate, POST /place               │          │
│     └──────────────────────────────────────────────────┘          │
│                                                                    │
└────────────────────────────────────────────────────────────────────┘
```

**SPA implementation in Sketchpad**:

```tsx
// ============================================================
// EXAMPLE 1: SINGLE-PAGE APPLICATION (SPA) ROUTING
// ============================================================
// Purpose: Handle navigation WITHOUT page reloads
// 
// Traditional websites: click link → server sends new HTML page
// SPA: click link → JavaScript updates the view, URL changes, NO reload
//
// Benefits: faster navigation, app-like feel, state preserved

// From js/semio/sketchpad/Sketchpad.tsx

import { BrowserRouter, Routes, Route } from 'react-router-dom';  // React Router library
import { useActor } from '@xstate/react';                          // XState React hooks

function SketchpadApp() {
  // useActor gives us [current state, send function] from our state machine
  const [state, send] = useActor(sketchpadMachine);
  
  return (
    // BrowserRouter enables URL-based navigation without page reloads
    <BrowserRouter>
      {/* Routes: match URL path to component */}
      <Routes>
        {/* "/" matches the home page → show HomeApp component */}
        <Route path="/" element={<HomeApp />} />
        
        {/* ":kitGuid" is a URL parameter - variable part of the URL */}
        {/* Example: /kit/abc123 → kitGuid = "abc123" */}
        <Route path="/kit/:kitGuid" element={<KitApp />} />
        
        {/* Nested parameters: both kit and design GUIDs in URL */}
        {/* Example: /kit/abc123/design/xyz789 */}
        <Route path="/kit/:kitGuid/design/:designGuid" element={<DesignApp />} />
        
        {/* Type editing for a specific type in a kit */}
        <Route path="/kit/:kitGuid/type/:typeGuid" element={<TypeApp />} />
        
        {/* Quality editing */}
        <Route path="/kit/:kitGuid/quality/:qualityGuid" element={<QualityApp />} />
        
        {/* "/*" means match any path starting with /docs */}
        <Route path="/docs/*" element={<DocsApp />} />
        
        {/* Feedback form page */}
        <Route path="/feedback" element={<FeedbackApp />} />
      </Routes>
    </BrowserRouter>
  );
}
```

**Vite development server**:

```typescript
// ============================================================
// EXAMPLE 2: BUILD TOOL CONFIGURATION
// ============================================================
// Purpose: Configure how the app is built and served
// 
// Vite is a "build tool" - it:
// 1. Serves files during development (with hot reloading)
// 2. Bundles/optimizes files for production
//
// "Bundle" = combine many source files into fewer optimized files

// vite.config.ts for Sketchpad

import { defineConfig } from 'vite';       // Vite's config helper
import react from '@vitejs/plugin-react';   // Plugin for React support

export default defineConfig({
  plugins: [react()],  // Enable React JSX/TSX transformation
  
  server: {
    port: 5173,        // Dev server runs at localhost:5173
    hmr: true,         // HMR = Hot Module Replacement
                       // When you save a file, only that module reloads
                       // You don't lose app state (huge productivity boost!)
  },
  
  build: {
    rollupOptions: {   // Rollup is the bundler Vite uses for production
      output: {
        // Code splitting: instead of one giant bundle, create smaller chunks
        // Each chunk loads only when needed (faster initial load)
        manualChunks: {
          // Put Three.js and React Three Fiber in their own chunk
          'three': ['three', '@react-three/fiber'],
          // Put XState in its own chunk
          'xstate': ['xstate', '@xstate/react'],
          // Put Y.js and Liveblocks in their own chunk
          'yjs': ['yjs', '@liveblocks/client'],
        }
      }
    }
  }
});
```

**Progressive Web App capabilities (future)**:

```json
// ============================================================
// EXAMPLE 3: PROGRESSIVE WEB APP (PWA) MANIFEST
// ============================================================
// Purpose: Make web app "installable" like a native app
//
// PWA = Progressive Web App
// A web app that can be "installed" to home screen, work offline,
// and feel like a native app while being built with web technology.

// manifest.json for installable PWA
{
  "name": "semio Sketchpad",        // Full app name
  "short_name": "Sketchpad",        // Name shown under icon
  "start_url": "/",                 // URL to open when launched
  "display": "standalone",          // Hide browser chrome (looks native)
  "background_color": "#ffffff",    // Splash screen color
  "theme_color": "#000000",         // Browser toolbar color (on mobile)
  "icons": [                        // App icons at various sizes
    {
      "src": "/icons/icon-192.png", // 192x192 pixel icon
      "sizes": "192x192",           // Size declaration
      "type": "image/png"           // File format
    }
  ]
}
```

```typescript
// ============================================================
// EXAMPLE 4: SERVICE WORKER FOR OFFLINE SUPPORT
// ============================================================
// Purpose: Make the app work without internet connection
//
// Service Worker = script that runs in background, intercepts network requests
// It can cache files so the app works offline.
// Think of it as a "proxy" between your app and the network.

// sw.ts - Service Worker file

// 'install' event fires when service worker is first installed
self.addEventListener('install', (event) => {
  // waitUntil: don't finish installing until caching is done
  event.waitUntil(
    // Open a cache called 'sketchpad-v1'
    caches.open('sketchpad-v1').then((cache) => {
      // Cache all the essential files for offline use
      return cache.addAll([
        '/',              // Home page
        '/index.html',    // Main HTML
        '/main.js',       // JavaScript bundle
        '/styles.css',    // CSS styles
        // Add more static assets here...
      ]);
    })
  );
});

// 'fetch' event fires for EVERY network request the app makes
self.addEventListener('fetch', (event) => {
  event.respondWith(
    // Try to find the request in cache first
    caches.match(event.request).then((cached) => {
      // If found in cache, return it; otherwise, fetch from network
      return cached || fetch(event.request);
    })
  );
});
```

**Web app technology stack**:

| Layer        | Technology                              | Purpose                          |
|--------------|-----------------------------------------|----------------------------------|
| Bundler      | Vite                                    | Fast dev server, production build|
| Framework    | React 18                                | Component-based UI               |
| State        | XState + Y.js                           | UI state + collaborative data    |
| 3D           | Three.js + @react-three/fiber           | WebGL rendering                  |
| Styling      | Tailwind CSS v4                         | Utility-first CSS                |
| Routing      | React Router                            | SPA navigation                   |
| i18n         | i18next                                 | Internationalization             |

**Why Sketchpad is a web app**

Web application benefits:

- No installation (just open browser)
- Cross-platform (Windows, Mac, Linux, tablets)
- Always up to date (latest version on refresh)
- Easy sharing (send URL to collaborators)
- Collaboration-first (real-time via WebSocket)

**What it enables**

- Instant access from any device with a browser
- Seamless updates (no download/install)
- Link sharing (share specific design by URL)
- Real-time collaboration (multiple users in same design)
- Cross-platform (one codebase, all platforms)
- Easy deployment (just host static files + API)

**What it limits**

- Browser sandboxing (limited file system access)
- WebGL performance vs native OpenGL
- Memory limits (browser tab constraints)
- Offline requires service worker setup
- Initial load time (large JavaScript bundle)
- No system-level integration (native menus, etc.)

**Why web apps changed computing**

Before web apps:

- Install software per device
- Updates require user action
- Platform-specific development

Web apps provide:

- Instant access (URL)
- Automatic updates
- Cross-platform (write once)
- No install/maintenance burden

**What it enables**

- Universal access
- Continuous deployment
- Cross-platform
- Link sharing
- No installation
- Always up to date

**What it limits**

- Browser sandbox restrictions
- Performance vs. native
- Network dependency
- Limited device access
- Browser compatibility issues
- Complex offline support

---

#### 7.4 Desktop Applications: Native Speed

**Plain explanation**

Desktop applications are installed on your computer—Rhino, Blender, VS Code. They have full access to your machine's capabilities: files, hardware, processing power. They run without a browser, directly on your operating system.

semio has two desktop applications: **Desktop** (Electron-based Sketchpad) and **Grasshopper** (Rhino plugin). Desktop apps offer the richest experience but require installation.

**Technical explanation**

**semio desktop architecture**:

```
┌────────────────────────────────────────────────────────────────────┐
│                     Desktop Deployment Options                      │
├────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │            Electron Desktop (js/desktop/)                   │   │
│  │  ┌─────────────────────────────────────────────────────┐    │   │
│  │  │              Chromium (Browser Engine)               │    │   │
│  │  │  ┌───────────────────────────────────────────────┐  │    │   │
│  │  │  │           Sketchpad React App                  │  │    │   │
│  │  │  │   (Same code as web version)                   │  │    │   │
│  │  │  └───────────────────────────────────────────────┘  │    │   │
│  │  └─────────────────────────────────────────────────────┘    │   │
│  │  ┌─────────────────────────────────────────────────────┐    │   │
│  │  │              Node.js (Main Process)                  │    │   │
│  │  │   - File system access                               │    │   │
│  │  │   - Native menus                                     │    │   │
│  │  │   - System tray                                      │    │   │
│  │  │   - Auto-updates                                     │    │   │
│  │  └─────────────────────────────────────────────────────┘    │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │            Grasshopper Plugin (net/Semio.Grasshopper/)      │   │
│  │  ┌─────────────────────────────────────────────────────┐    │   │
│  │  │                 Rhino 8 (.NET 4.8)                   │    │   │
│  │  │  ┌───────────────────────────────────────────────┐  │    │   │
│  │  │  │         Grasshopper Canvas                     │  │    │   │
│  │  │  │  ┌──────────┐  ┌──────────┐  ┌──────────┐     │  │    │   │
│  │  │  │  │ Kit Comp │  │Type Comp │  │Place Comp│     │  │    │   │
│  │  │  │  └──────────┘  └──────────┘  └──────────┘     │  │    │   │
│  │  │  └───────────────────────────────────────────────┘  │    │   │
│  │  └─────────────────────────────────────────────────────┘    │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
└────────────────────────────────────────────────────────────────────┘
```

**Electron Desktop implementation**:

```typescript
// ============================================================
// EXAMPLE 1: ELECTRON DESKTOP APPLICATION
// ============================================================
// Purpose: Run the web-based Sketchpad as a native desktop app
//
// Electron = framework that bundles Chromium (browser) + Node.js (backend)
// into a single installable app. You write web code, get a desktop app.
//
// Famous Electron apps: VS Code, Slack, Discord, Figma Desktop

// From js/desktop/main.ts

import { app, BrowserWindow, ipcMain, Menu } from 'electron';  // Electron APIs
import path from 'path';  // Node.js path utilities

let mainWindow: BrowserWindow | null = null;  // Reference to our window

// app.whenReady() - Electron is initialized, we can create windows now
app.whenReady().then(() => {
  // Create the main application window
  mainWindow = new BrowserWindow({
    width: 1400,              // Window width in pixels
    height: 900,              // Window height in pixels
    webPreferences: {
      nodeIntegration: false, // Don't expose Node.js directly (security!)
      contextIsolation: true, // Separate contexts for security
      preload: path.join(__dirname, 'preload.js'),  // Safe bridge script
    },
  });

  // Load Sketchpad - SAME React app as the web version!
  if (process.env.NODE_ENV === 'development') {
    mainWindow.loadURL('http://localhost:5173');  // Dev: connect to Vite
  } else {
    mainWindow.loadFile('dist/index.html');       // Prod: load bundled files
  }

  // Native menu - these appear in the OS menu bar (File, Edit, etc.)
  const menu = Menu.buildFromTemplate([
    {
      label: 'File',                              // Menu title
      submenu: [
        { label: 'New Kit', accelerator: 'CmdOrCtrl+N', click: newKit },      // Keyboard shortcut
        { label: 'Open Kit...', accelerator: 'CmdOrCtrl+O', click: openKit },
        { label: 'Save', accelerator: 'CmdOrCtrl+S', click: saveKit },
        { type: 'separator' },                    // Visual divider line
        { label: 'Exit', role: 'quit' },          // OS-level quit
      ],
    },
  ]);
  Menu.setApplicationMenu(menu);  // Apply the menu to the app
});

// ── IPC: COMMUNICATION BETWEEN MAIN AND RENDERER ───────────
// IPC = Inter-Process Communication
// Main process (Node.js) and Renderer (Chromium) are separate processes.
// They communicate via "channels" - like sending messages back and forth.

ipcMain.handle('file:save', async (event, kitData) => {
  // handle() means: when renderer calls 'file:save', run this function
  const { dialog } = require('electron');  // Native OS dialog
  
  // Show the OS "Save File" dialog
  const result = await dialog.showSaveDialog({
    filters: [{ name: 'Kit', extensions: ['zip'] }],  // Only .zip files
  });
  
  if (!result.canceled) {
    // User chose a location - write the file to disk
    await fs.writeFile(result.filePath, kitData);
  }
  
  return result.filePath;  // Return path to renderer
});
```

**Grasshopper plugin implementation**:

```csharp
// ============================================================
// EXAMPLE 2: GRASSHOPPER PLUGIN COMPONENT (C#)
// ============================================================
// Purpose: Create semio components that work in Grasshopper visual programming
//
// Grasshopper = visual programming environment inside Rhino (3D software)
// Users connect "components" (nodes) with wires to define data flows.
// Each component has inputs on the left, outputs on the right.

// From net/Semio.Grasshopper/Semio.Grasshopper.cs

using Grasshopper.Kernel;   // Core Grasshopper API
using Rhino.Geometry;        // Rhino 3D geometry types
using Semio;                 // Our semio domain library

// GH_Component is the base class for all Grasshopper components
public class KitComponent : GH_Component
{
    // Constructor: define component metadata
    public KitComponent() 
        : base(
            "Kit",           // Name (shown on component)
            "Kit",           // Nickname (abbreviated)
            "Construct a semio Kit",  // Description (shown in tooltip)
            "semio",         // Category (tab in ribbon)
            "Kit"            // Subcategory (section in panel)
        ) { }

    // RegisterInputParams: define what goes INTO this component
    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        // AddTextParameter: this input accepts text (string)
        pManager.AddTextParameter(
            "Name", "N",     // Name and Nickname
            "Kit name",      // Description
            GH_ParamAccess.item  // Single value (not list)
        );
        
        // AddParameter with custom type: this input accepts Type objects
        pManager.AddParameter(
            new TypeParam(),   // Custom parameter type for semio Types
            "Types", "T",      // Name and Nickname
            "Types in kit",    // Description
            GH_ParamAccess.list  // Multiple values (list)
        );
        
        pManager.AddParameter(
            new DesignParam(), "Designs", "D", 
            "Designs in kit", 
            GH_ParamAccess.list
        );
    }

    // RegisterOutputParams: define what comes OUT of this component
    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(
            new KitParam(),    // Custom parameter type for Kit
            "Kit", "K",        // Name and Nickname
            "Constructed kit", // Description
            GH_ParamAccess.item
        );
    }

    // SolveInstance: the actual logic that runs when component executes
    protected override void SolveInstance(IGH_DataAccess DA)
    {
        // Declare variables to hold input values
        string name = "";
        var types = new List<TypeGoo>();     // Goo = Grasshopper wrapper class
        var designs = new List<DesignGoo>();
        
        // Read input values from the wires
        DA.GetData(0, ref name);             // Input 0 = Name
        DA.GetDataList(1, types);            // Input 1 = Types (list)
        DA.GetDataList(2, designs);          // Input 2 = Designs (list)

        // Construct the Kit object from inputs
        var kit = new Kit
        {
            Name = name,
            Types = types.Select(t => t.Value).ToList(),    // Unwrap Goo → Type
            Designs = designs.Select(d => d.Value).ToList(),// Unwrap Goo → Design
        };

        // Set output value - will flow down the wire to connected components
        DA.SetData(0, new KitGoo(kit));  // Wrap Kit in Goo for Grasshopper
    }
}
```

**Desktop technology choices in semio**:

| Component         | Technology        | Why                                           |
|-------------------|-------------------|-----------------------------------------------|
| Electron Desktop  | Electron + React  | Reuse web Sketchpad, cross-platform           |
| Grasshopper       | C# .NET 4.8       | Required by Rhino SDK                         |
| File dialogs      | Native (Electron) | System file picker UI                         |
| Auto-update       | electron-updater  | Seamless background updates                   |

**Why desktop apps are still needed for semio**

Desktop required for:

- **Rhino integration**: Grasshopper runs inside Rhino (native C#)
- **Offline-first**: Architects often work without internet
- **Large files**: Kit zips can be hundreds of MB
- **Performance**: 3D rendering benefits from native access
- **Professional workflow**: Designers expect desktop apps

**What it enables**

- Full file system access (save anywhere, recent files)
- Maximum 3D performance (native GPU access in Rhino)
- Offline work (no network dependency)
- System integration (drag files into app)
- Native menus and shortcuts (Cmd/Ctrl+S works)
- Background processing (long validations)

**What it limits**

- Installation friction (must download installer)
- Platform-specific builds (Windows, macOS, Linux)
- Update distribution (auto-update or manual)
- Size (Electron bundles Chromium ~150MB)
- Compatibility across OS versions
- App store policies (macOS notarization)

---

#### 7.5 Mobile Applications: Computers in Your Pocket

**Plain explanation**

Mobile apps run on smartphones and tablets—iOS (iPhone/iPad) and Android. They're designed for touch, small screens, and mobile usage patterns. For semio, mobile could mean reviewing designs on-site, checking kit inventories, or collaborating while away from the desk.

semio doesn't have a mobile app yet, but the architecture supports it: the same Engine API and collaboration infrastructure would power a mobile client.

**Technical explanation**

**Potential semio mobile architecture**:

```
┌────────────────────────────────────────────────────────────────────┐
│                    Future Mobile Strategy                           │
├────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  ┌───────────────────────────────────────────────────────────────┐ │
│  │   Option 1: React Native (Reuse Knowledge)                    │ │
│  │                                                                │ │
│  │   js/mobile/                                                   │ │
│  │   ├── App.tsx          (React Native app shell)               │ │
│  │   ├── components/      (Mobile-optimized UI)                  │ │
│  │   └── shared/          (Import from @semio/js)                │ │
│  │                                                                │ │
│  │   Shares: Zod schemas, domain logic, validation               │ │
│  │   Different: UI components, navigation, touch gestures        │ │
│  └───────────────────────────────────────────────────────────────┘ │
│                                                                     │
│  ┌───────────────────────────────────────────────────────────────┐ │
│  │   Option 2: PWA (No App Store)                                │ │
│  │                                                                │ │
│  │   Responsive Sketchpad + Service Worker                       │ │
│  │   - Touch-friendly UI adjustments                             │ │
│  │   - Offline kit viewing                                        │ │
│  │   - Add to home screen                                         │ │
│  │   - Same codebase as web                                       │ │
│  └───────────────────────────────────────────────────────────────┘ │
│                                                                     │
│  ┌───────────────────────────────────────────────────────────────┐ │
│  │   Shared Infrastructure (Already Exists)                      │ │
│  │                                                                │ │
│  │   Engine API ────────────► Same /validate, /place             │ │
│  │   Liveblocks ────────────► Same real-time collaboration       │ │
│  │   @semio/js  ────────────► Same Kit, Type, Design types       │ │
│  └───────────────────────────────────────────────────────────────┘ │
│                                                                     │
└────────────────────────────────────────────────────────────────────┘
```

**Mobile development approaches**:

```tsx
// ============================================================
// EXAMPLE 1: HYPOTHETICAL REACT NATIVE MOBILE APP
// ============================================================
// Purpose: Show how semio domain code could be reused in a mobile app
//
// React Native = framework to build native iOS/Android apps with React
// The UI components are different (View not div, TouchableOpacity not button)
// But the LOGIC (domain types, validation, API calls) is SHARED with web!

// Hypothetical: js/mobile/KitViewer.tsx

import { View, FlatList, TouchableOpacity } from 'react-native';
// ^ These are React Native components - compile to native iOS/Android views

import { useKit } from '@semio/js';  // SAME domain logic as web and desktop!
// ^ This is the key: Kit, Type, Design, validation - all reusable

export function KitViewer({ kitGuid }: { kitGuid: Guid }) {
  const kit = useKit(kitGuid);  // Same hook, works on mobile too
  
  return (
    // View = like <div> but for native mobile
    <View style={styles.container}>
      {/* FlatList = optimized scrollable list for mobile (like ul but smart) */}
      <FlatList
        data={kit.types}               // Array of Type objects
        renderItem={({ item }) => (    // How to render each Type
          // TouchableOpacity = tappable area that dims on press
          <TouchableOpacity 
            onPress={() => navigateToType(item.guid)}  // Touch → navigate
            style={styles.typeCard}
          >
            {/* Text = like <span> but for mobile */}
            <Text style={styles.typeName}>{item.name}</Text>
            <Text style={styles.connectorCount}>
              {item.connectors.length} connectors
            </Text>
          </TouchableOpacity>
        )}
      />
    </View>
  );
}
```

**Mobile-specific concerns for semio**:

| Concern          | Mobile Challenge                     | semio Approach                    |
|------------------|--------------------------------------|-----------------------------------|
| Touch input      | No hover, finger vs mouse precision  | Larger tap targets, gestures      |
| 3D rendering     | Limited GPU, battery drain           | Simplified models, LOD            |
| Screen size      | Can't show full design canvas        | Focus mode per piece              |
| Offline          | Site visits without connectivity     | Y.js local persistence            |
| Performance      | Slower than desktop/laptop           | Lazy loading, reduced features    |
| Battery          | 3D rendering drains battery          | On-demand rendering               |

**Use cases for mobile semio**:

```
┌─────────────────────────────────────────────────────────────────────┐
│  Mobile Use Cases                                                    │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  👷 On-Site Review                                                   │
│     - View design at construction site                               │
│     - Check piece types and connections                              │
│     - Compare digital to physical                                    │
│                                                                      │
│  📋 Inventory Management                                             │
│     - Scan kit barcodes                                              │
│     - Check type stock levels                                        │
│     - Mark pieces as installed                                       │
│                                                                      │
│  🤝 Quick Collaboration                                              │
│     - Review changes while traveling                                 │
│     - Approve design modifications                                   │
│     - Add comments to pieces                                         │
│                                                                      │
│  📸 Documentation                                                    │
│     - Photo-to-piece mapping                                         │
│     - AR overlay of design on site                                   │
│     - Progress tracking                                              │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

**Why mobile requires rethinking**

Mobile isn't just small Sketchpad:

- Touch vs. mouse (can't hover over connectors)
- Context changes (construction site, meeting room)
- Limited attention (quick lookups, not long sessions)
- Sensors (GPS for location, camera for AR)
- Background restrictions (can't compute placements)
- Push notifications (design updated, collaborator joined)

**What it enables**

- Always with designer (check designs anywhere)
- Location awareness (on-site verification)
- Camera integration (AR overlay on physical space)
- Push notifications (real-time collaboration alerts)
- Native integrations (share to other apps)
- Mobile-specific experiences (scan barcode → view type)

**What it limits**

- Small screens (can't see full graph diagram)
- Battery constraints (3D rendering is expensive)
- App store gatekeeping (Apple/Google review)
- Platform fragmentation (Android device variety)
- Background execution limits (no background sync)
- Two platforms to support (iOS + Android)

---

#### 7.6 Containers: Reproducible Environments

**Plain explanation**

A container is a way to package an application with everything it needs—code, libraries, configuration—into one unit that runs identically anywhere. "Works on my machine" becomes "works on every machine."

semio uses containers for the Engine backend. The Engine container includes Python, FastAPI, NumPy, and all dependencies—anyone can run `docker run semio/engine` and have a working API server.

**Technical explanation**

**Engine container architecture**:

```
┌────────────────────────────────────────────────────────────────────┐
│                    Engine Container                                 │
│                  (py/engine/Dockerfile)                            │
├────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │   Layer 1: Base Image (python:3.11-slim)                    │   │
│  │   - Python 3.11 runtime                                      │   │
│  │   - Minimal Debian Linux                                     │   │
│  └─────────────────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │   Layer 2: System Dependencies                               │   │
│  │   - apt-get install build-essential                          │   │
│  └─────────────────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │   Layer 3: Python Dependencies (pip install)                 │   │
│  │   - fastapi, uvicorn, pydantic                               │   │
│  │   - numpy, scipy (3D math)                                   │   │
│  │   - (cached layer - only rebuilds if requirements change)    │   │
│  └─────────────────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │   Layer 4: Application Code                                  │   │
│  │   - COPY engine.py /app/                                     │   │
│  │   - COPY models/ /app/models/                                │   │
│  └─────────────────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │   Layer 5: Configuration                                     │   │
│  │   - EXPOSE 2507                                              │   │
│  │   - CMD ["uvicorn", "engine:app", "--host", "0.0.0.0"]       │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
└────────────────────────────────────────────────────────────────────┘
```

**Engine Dockerfile**:

```dockerfile
# ============================================================
# EXAMPLE 1: DOCKERFILE - CONTAINER BUILD INSTRUCTIONS
# ============================================================
# Purpose: Define how to build the Engine container image
#
# A Dockerfile is a RECIPE: "start with X, add Y, configure Z"
# Docker reads this file and creates an IMAGE (snapshot).
# You can then RUN that image as a CONTAINER (running instance).

# py/engine/Dockerfile

# ── LAYER 1: BASE IMAGE ─────────────────────────────────────
# FROM = start with an existing image
# python:3.11-slim = Python 3.11 on minimal Debian Linux
FROM python:3.11-slim

# ── LAYER 2: WORKING DIRECTORY ──────────────────────────────
# WORKDIR = all following commands run from this folder
# Like "cd /app" that persists
WORKDIR /app

# ── LAYER 3: SYSTEM DEPENDENCIES ────────────────────────────
# RUN = execute a shell command
# apt-get = Debian package manager (like npm for OS packages)
RUN apt-get update && apt-get install -y \
    build-essential \
    && rm -rf /var/lib/apt/lists/*
# ^ install build tools, then clean up to keep image small

# ── LAYER 4: PYTHON DEPENDENCIES ────────────────────────────
# COPY = copy files from your computer into the image
# Copy requirements FIRST so this layer is cached
# If requirements.txt doesn't change, Docker reuses this layer
COPY requirements.txt ./
RUN pip install --no-cache-dir -r requirements.txt
# ^ install all Python packages listed in requirements.txt

# ── LAYER 5: APPLICATION CODE ───────────────────────────────
# Copy the actual code LAST (changes most often)
COPY engine.py ./
COPY models/ ./models/

# ── LAYER 6: NETWORK CONFIGURATION ──────────────────────────
# EXPOSE = document which port the container listens on
# This doesn't actually open the port - it's documentation
EXPOSE 2507

# ── LAYER 7: HEALTH CHECK ───────────────────────────────────
# HEALTHCHECK = how to verify the container is healthy
# Orchestrators (Kubernetes) use this to restart unhealthy containers
HEALTHCHECK --interval=30s --timeout=10s \
  CMD curl -f http://localhost:2507/health || exit 1

# ── LAYER 8: STARTUP COMMAND ────────────────────────────────
# CMD = the command to run when container starts
# uvicorn = ASGI server for FastAPI
# --host 0.0.0.0 = listen on all network interfaces
CMD ["uvicorn", "engine:app", "--host", "0.0.0.0", "--port", "2507"]
```

**Docker Compose for development**:

```yaml
# ============================================================
# EXAMPLE 2: DOCKER COMPOSE - MULTI-CONTAINER SETUP
# ============================================================
# Purpose: Define how multiple containers work together
#
# docker-compose.yml lets you run multiple containers with one command:
#   docker-compose up
# Instead of running docker run ... for each container separately.

# docker-compose.yml

version: '3.8'          # Docker Compose file format version

services:               # Define the containers ("services")
  
  # ── ENGINE SERVICE ────────────────────────────────────────
  engine:
    build: ./py/engine  # Build from this Dockerfile location
    ports:
      - "2507:2507"     # Map host:2507 → container:2507
    volumes:
      - ./py/engine:/app  # Mount local folder into container
                          # Changes to local files appear inside container
                          # Enables hot-reload during development!
    environment:
      - DEBUG=true        # Pass environment variable to container
    
  # ── LIVEBLOCKS PROXY SERVICE ──────────────────────────────
  liveblocks-proxy:
    image: semio/liveblocks-proxy  # Use pre-built image (not build)
    ports:
      - "4000:4000"       # WebSocket proxy port
    environment:
      - LIVEBLOCKS_SECRET_KEY=${LIVEBLOCKS_SECRET_KEY}
      # ^ ${...} reads from .env file or shell environment
```

**Running semio with Docker**:

```bash
# ============================================================
# EXAMPLE 3: DOCKER COMMANDS
# ============================================================
# Purpose: Common commands for building and running containers

# ── BUILD ───────────────────────────────────────────────────
# Create a container IMAGE from the Dockerfile
docker build -t semio/engine ./py/engine
#            │  └── -t = tag (name) the image "semio/engine"
#            └── ./py/engine = folder containing Dockerfile

# ── RUN ─────────────────────────────────────────────────────
# Start a CONTAINER from the image
docker run -d \
  --name semio-engine \    # Name this container instance
  -p 2507:2507 \           # Port mapping: host:container
  semio/engine             # The image to run
#          └── -d = detached mode (run in background)

# ── INSPECT ─────────────────────────────────────────────────
# Check container logs (stdout/stderr output)
docker logs semio-engine

# Open a shell INSIDE the running container
docker exec -it semio-engine bash
#           │└ -t = allocate terminal
#           └── -i = interactive mode

# ── CLEANUP ─────────────────────────────────────────────────
# Stop and remove the container
docker stop semio-engine && docker rm semio-engine
```

**Container vs VM comparison**:

```
┌─────────────────────────────────────────────────────────────────────┐
│                    Containers vs Virtual Machines                    │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  Virtual Machines:                  Containers:                      │
│  ┌───────────────────┐              ┌───────────────────┐           │
│  │   App A   App B   │              │   App A   App B   │           │
│  ├───────────────────┤              ├───────────────────┤           │
│  │  Guest OS  Guest  │              │ Container Runtime │           │
│  │  (Linux)   (Win)  │              │    (Docker)       │           │
│  ├───────────────────┤              ├───────────────────┤           │
│  │    Hypervisor     │              │    Host OS        │           │
│  ├───────────────────┤              │    (Linux)        │           │
│  │    Host OS        │              └───────────────────┘           │
│  └───────────────────┘                                              │
│                                                                      │
│  Startup: Minutes                   Startup: Seconds                 │
│  Size: Gigabytes                    Size: Megabytes                  │
│  Isolation: Full                    Isolation: Process-level         │
│  Overhead: High                     Overhead: Low                    │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

**Why containers were invented**

Before containers:
- "Works on my machine, not on server"
- Complex server setup (Python version, NumPy version)
- Conflicting dependencies (project A needs X, project B needs Y)
- Environment drift over time

Containers provide for semio:
- Identical environments (dev = staging = prod)
- Fast startup (Engine up in seconds)
- Efficient resource use (run multiple Engines)
- Easy scaling (more containers = more capacity)

**What it enables**

- Reproducible builds (Engine works identically everywhere)
- Consistent environments (no "but it worked locally")
- Fast deployment (ship image, run container)
- Easy horizontal scaling (10 Engine containers for high load)
- Development/production parity (same container everywhere)
- Easy onboarding (new developer runs `docker-compose up`)

**What it limits**

- Orchestration complexity (Kubernetes for production)
- Security considerations (shared kernel attack surface)
- Networking complexity (container-to-container communication)
- State management (containers are ephemeral)
- Learning curve (Docker, Compose, registries)
- Overhead (still more than bare metal)

---

#### 7.7 Orchestration: Managing Many Containers

**Plain explanation**

When you have dozens or hundreds of containers, you can't manage them manually. Orchestration automates: starting containers, distributing them across servers, restarting failed ones, scaling up/down, and routing traffic.

For semio production deployment, Kubernetes would manage multiple Engine instances, handling load balancing and automatic scaling during peak usage.

**Technical explanation**

**semio production architecture (Kubernetes)**:

```
┌────────────────────────────────────────────────────────────────────┐
│                   Kubernetes Cluster                                │
├────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │   Ingress Controller (nginx)                                 │   │
│  │   - SSL termination                                          │   │
│  │   - Route /api/* to Engine service                           │   │
│  │   - Route /* to Sketchpad service                            │   │
│  └────────────────────────────────┬────────────────────────────┘   │
│                                   │                                 │
│           ┌───────────────────────┼───────────────────────┐        │
│           ▼                       ▼                       ▼        │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐    │
│  │  Engine Pod 1   │  │  Engine Pod 2   │  │  Engine Pod 3   │    │
│  │  py/engine      │  │  py/engine      │  │  py/engine      │    │
│  │  Port: 2507     │  │  Port: 2507     │  │  Port: 2507     │    │
│  └─────────────────┘  └─────────────────┘  └─────────────────┘    │
│           ↑                   ↑                   ↑                │
│           └───────────────────┼───────────────────┘                │
│                               │                                     │
│  ┌────────────────────────────┼────────────────────────────────┐   │
│  │            Engine Service (ClusterIP)                        │   │
│  │   - Load balances across Engine pods                         │   │
│  │   - DNS: engine.semio.svc.cluster.local                      │   │
│  └──────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  ┌────────────────────────────────────────────────────────────┐    │
│  │   Horizontal Pod Autoscaler                                  │    │
│  │   - Scale Engine pods based on CPU/request count             │    │
│  │   - Min: 2 pods, Max: 10 pods                                │    │
│  └────────────────────────────────────────────────────────────┘    │
│                                                                     │
└────────────────────────────────────────────────────────────────────┘
```

**Kubernetes manifests for semio**:

```yaml
# ============================================================
# EXAMPLE 1: KUBERNETES DEPLOYMENT MANIFEST
# ============================================================
# Purpose: Define how to run the Engine in a Kubernetes cluster
#
# Kubernetes reads YAML files that describe "desired state":
# "I want 3 Engine pods running, each with this much CPU/memory"
# Kubernetes then makes that happen and KEEPS it that way.

# k8s/engine-deployment.yaml

# ── DEPLOYMENT: DEFINES THE PODS ────────────────────────────
apiVersion: apps/v1         # Kubernetes API version
kind: Deployment            # Type of resource
metadata:
  name: semio-engine        # Name of this deployment
  labels:
    app: engine             # Labels for grouping/selecting
spec:
  replicas: 3               # We want 3 copies of this pod running!
  selector:
    matchLabels:
      app: engine           # Which pods belong to this deployment
  template:                 # Template for creating pods
    metadata:
      labels:
        app: engine         # Pods get this label
    spec:
      containers:           # What runs inside each pod
        - name: engine
          image: semio/engine:1.0.0  # Docker image to use
          ports:
            - containerPort: 2507    # Container listens on 2507
          resources:
            requests:               # Minimum resources to schedule
              memory: "256Mi"       # 256 megabytes RAM
              cpu: "250m"           # 0.25 CPU cores
            limits:                 # Maximum resources allowed
              memory: "512Mi"       # Cap at 512 MB RAM
              cpu: "500m"           # Cap at 0.5 CPU cores
          livenessProbe:            # "Is the container alive?"
            httpGet:                # Check via HTTP request
              path: /health
              port: 2507
            initialDelaySeconds: 10 # Wait 10s before first check
          readinessProbe:           # "Is the container ready for traffic?"
            httpGet:
              path: /health
              port: 2507
            initialDelaySeconds: 5

# ── SERVICE: DNS NAME FOR PODS ──────────────────────────────
# A Service gives pods a stable network identity
# Other services call "engine" and Kubernetes routes to a healthy pod
---
apiVersion: v1
kind: Service
metadata:
  name: engine              # DNS name: engine.default.svc.cluster.local
spec:
  selector:
    app: engine             # Route to pods with this label
  ports:
    - port: 2507            # Expose port 2507
      targetPort: 2507      # Forward to container port 2507
  type: ClusterIP           # Only accessible inside cluster

# ── AUTOSCALER: SCALE UP/DOWN AUTOMATICALLY ─────────────────
# HPA watches CPU usage and adjusts replica count
---
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: engine-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: semio-engine      # Scale this deployment
  minReplicas: 2            # Never go below 2 pods
  maxReplicas: 10           # Never go above 10 pods
  metrics:
    - type: Resource
      resource:
        name: cpu
        target:
          type: Utilization
          averageUtilization: 70  # Scale up when CPU > 70%
```

**Kubernetes provides for semio**:

| Feature           | semio Benefit                                      |
|-------------------|----------------------------------------------------|
| Self-healing      | Engine pod crashes → K8s starts new one            |
| Load balancing    | Traffic distributed across all Engine pods         |
| Rolling updates   | Deploy new Engine version with zero downtime       |
| Resource limits   | Prevent one validation from consuming all memory   |
| Service discovery | Sketchpad finds Engine via DNS name                |
| Autoscaling       | Add Engine pods when many users validate kits      |

**Why orchestration matters**

Manual container management doesn't scale:

- Which server has capacity for new Engine?
- What if an Engine pod crashes at 3am?
- How does Sketchpad find available Engine pods?
- How to deploy Engine updates without downtime?

Kubernetes automates these operational concerns.

**What it enables**

- Running many Engine instances (high availability)
- Automatic scaling (more users → more pods)
- Zero-downtime deployments (rolling updates)
- Resource efficiency (pack pods onto nodes)
- Self-healing systems (crashed pod → new pod)
- Geographic distribution (pods in multiple regions)

**What it limits**

- Significant complexity (K8s is complex)
- Learning curve (YAML manifests, kubectl)
- Operational overhead (monitor cluster health)
- Debugging distributed systems (which pod failed?)
- Cost of running Kubernetes (managed K8s or self-hosted)
- May be overkill (semio could start with docker-compose)

---

#### 7.8 Microservices: Small, Focused Services

**Plain explanation**

Instead of one big program (monolith), microservices split an application into many small services. Each service does one thing: one handles users, another handles orders, another handles payments. They communicate over the network.

semio is **NOT** a microservices architecture—it's a **modular monorepo** with clear boundaries between packages that compile together or communicate locally.

**Technical explanation**

**semio's explicit choice: modular monorepo, not microservices**:

```
      ❌ MICROSERVICES                    ✅ SEMIO'S ACTUAL ARCHITECTURE
     (What semio is NOT)                     (Modular Monorepo)
                                       
  ┌─────────────┐                       ┌──────────────────────────────┐
  │ Type Service │→network→             │       semio monorepo         │
  │   API        │      ↓               │                              │
  └─────────────┘  ┌────────────┐       │  ┌─────────┐  ┌─────────┐   │
                   │Design Svc  │       │  │ @semio  │  │ Engine  │   │
  ┌─────────────┐  │   API      │       │  │  /js    │←→│ py/     │   │
  │Validate Svc │  └────────────┘       │  └─────────┘  └─────────┘   │
  │   API       │→network→              │       │           │         │
  └─────────────┘      ↓                │       ↓           ↓         │
                   ┌────────────┐       │  ┌────────────────────┐     │
  ┌─────────────┐  │ Kit Svc    │       │  │ Single SQLite DB   │     │
  │ Auth Service │  │   API     │       │  │ (per kit)          │     │
  │   API        │  └────────────┘       │  └────────────────────┘     │
  └─────────────┘                       └──────────────────────────────┘
   4 network hops                         Function calls + 1 API call
```

**Why semio rejected microservices**:

| Microservice Argument        | semio Counter-Argument                           |
|-----------------------------|--------------------------------------------------|
| Independent deployment       | Monorepo coordinates all deployments anyway      |
| Scale services independently | 3D math is CPU-bound, all ops need same scaling  |
| Team autonomy               | Small team, no organizational boundaries needed   |
| Technology diversity        | Already have TypeScript, Python, C#, Go—enough!  |
| Isolated failures           | Single user's kit failure is isolated already    |

**What microservices would look like (and why semio avoids it)**:

```typescript
// ============================================================
// EXAMPLE 1: MICROSERVICES VS MONOLITH COMPARISON
// ============================================================
// Purpose: Show WHY semio chose NOT to use microservices
//
// Microservices = every feature is a separate program communicating over network
// Monolith = everything in one program, features call each other directly
//
// semio uses MODULAR MONOLITH: one program, but with clear internal boundaries

// ❌ IF semio used microservices (unnecessary complexity):

// Each "service" would be a separate server with its own API
// Validating a Type would require MULTIPLE network requests

app.post('/types/:typeGuid/validate', async (req, res) => {
  // Network call to Kit Service to get the Type
  const type = await fetch('http://kit-service:3000/types/' + req.params.typeGuid);
  
  // Network call to Connector Service to get connectors
  const connectors = await fetch('http://connector-service:3002/...');
  
  // Network latency: ~5ms × 3 services = ~15ms overhead PER REQUEST
  // Plus serialization/deserialization for each network hop
});

// ✅ What semio actually does (in-process):

function validateType(type: Type): ValidationResult {
  // Direct function call: ~0.001ms (15,000x faster than network!)
  const connectorProblems = validateConnectors(type.connectors);
  
  // No network, no serialization, no service discovery
  return { problems: connectorProblems };
}

// The "boundaries" in semio are PACKAGES, not SERVICES
// @semio/js can't accidentally depend on @semio/sketchpad
// But they call each other via imports, not HTTP
```

**Microservices characteristics** (for reference):

- Single responsibility per service
- Own database per service
- Network communication (HTTP, gRPC, messages)
- Independent deployment and scaling
- Service mesh for discovery

**semio uses these patterns locally instead**:

| Microservice Pattern      | semio Local Equivalent                     |
|---------------------------|--------------------------------------------|
| Service boundaries        | Nx package boundaries (`@semio/js`, etc)   |
| Service contracts         | TypeScript interfaces, JSON schemas        |
| API versioning            | Schema versioning in `jsonschema/`         |
| Service discovery         | Import resolution, single Engine URL       |
| Independent datastores    | Per-kit SQLite files                       |

**Why microservices exist**

Microservices solve problems semio doesn't have:

- Large organizations with 50+ developers
- Teams that can't coordinate releases
- Vastly different scaling requirements (semio's aren't)
- Regulatory isolation requirements (semio doesn't need)

**What microservices enable**

- Independent team ownership (not needed: small team)
- Independent scaling (not needed: uniform workload)
- Technology diversity (already achieved via monorepo)
- Isolated failures (achieved via per-kit isolation)
- Faster deploys (achieved via Nx caching)

**What microservices cost**

- Network latency (every call crosses network)
- Distributed transactions (hard to undo across services)
- Debugging across services (request tracing overhead)
- Operational overhead (12+ services to monitor)
- Testing complexity (need all services running)

---

#### 7.9 Monoliths: Everything Together

**Plain explanation**

A monolith is the opposite of microservices—everything in one application. The entire codebase deploys together as one unit. All features share the same database, the same process, the same deployment.

semio is a **modular monolith**—deliberately choosing monolith architecture with strong internal boundaries via Nx packages.

**Technical explanation**

**semio as a modular monolith**:

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         semio MODULAR MONOLITH                          │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│   ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐   │
│   │  @semio/js  │  │ @semio/docs │  │@semio/vscode│  │@semio/desktop│   │
│   │  (shared)   │  │  (website)  │  │ (extension) │  │ (Electron)  │   │
│   └──────┬──────┘  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘   │
│          │                │                │                │           │
│          └────────────────┼────────────────┼────────────────┘           │
│                           │                │                             │
│                           ▼                ▼                             │
│              ┌────────────────────────────────────┐                      │
│              │      Shared domain logic           │                      │
│              │      js/semio/semio.ts             │                      │
│              └────────────────────────────────────┘                      │
│                                                                          │
│   ┌─────────────────────────────────────────────────────────────────┐   │
│   │   Engine (Python)  ←HTTP→  Sketchpad (TypeScript)               │   │
│   │   Only external communication in entire system                   │   │
│   └─────────────────────────────────────────────────────────────────┘   │
│                                                                          │
│   ┌─────────────────────────────────────────────────────────────────┐   │
│   │   Storage: Per-kit SQLite (kit.db) or IndexedDB (browser)       │   │
│   └─────────────────────────────────────────────────────────────────┘   │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

**Modular boundaries in semio**:

```
nx.json package dependencies:

@semio/js         → (no dependencies, pure domain logic)
@semio/sketchpad  → @semio/js
@semio/docs       → @semio/js
@semio/vscode     → @semio/js
@semio/desktop    → @semio/sketchpad, @semio/js
```

**semio's monolith structure**:

| Module           | Responsibility                    | Deployment      |
|------------------|-----------------------------------|-----------------|
| `js/semio/`      | Shared domain logic               | Bundled into apps|
| `js/sketchpad/`  | React UI for kit editing          | Vite build      |
| `js/docs/`       | Documentation website             | Static site     |
| `js/vscode/`     | VS Code extension                 | VSIX package    |
| `js/desktop/`    | Electron desktop app              | Electron build  |
| `py/engine/`     | Backend computations              | Docker/direct   |
| `net/Semio/`     | Rhino/Grasshopper integration     | Yak package     |
| `go/repo/`       | CLI and MCP tools                 | Go binary       |

**Why semio chose modular monolith**:

```typescript
// ============================================================
// EXAMPLE 1: BENEFITS OF MODULAR MONOLITH
// ============================================================
// Purpose: Show concrete advantages of semio's architecture choice
//
// "Modular Monolith" means:
// - One program (monolith) - features call each other directly
// - Clear boundaries (modular) - packages can't access each other arbitrarily

// ── BENEFIT 1: DIRECT FUNCTION CALLS ───────────────────────
// No network latency, no serialization, just fast function calls

// In Sketchpad (TypeScript):
import { validateKit, applyKitDiff } from '@semio/js';  // Direct import!

function handleKitChange(kit: Kit, diff: KitDiff): Kit {
  const result = validateKit(kit);         // Direct call: ~1ms
  // If microservices: ~15ms (network) + parsing overhead
  
  if (result.problems.length === 0) {
    return applyKitDiff(kit, diff);         // No network needed
  }
  return kit;
}

// ── BENEFIT 2: SHARED TYPES ────────────────────────────────
// Same "Kit" type used everywhere - TypeScript checks consistency
// If js/sketchpad/ uses Kit.types and js/semio/ renames it to Kit.typeList,
// TypeScript compiler catches the error IMMEDIATELY in the same build
```

```typescript
// ============================================================
// EXAMPLE 2: ATOMIC REFACTORING
// ============================================================
// Purpose: Show how modular monolith enables safe, complete refactoring
//
// When you rename something, you can change it EVERYWHERE at once.
// No API versioning, no backwards compatibility, no deprecation periods.

// Rename "Connector" to "Port" everywhere at once:

// STEP 1: Change in js/semio/semio.ts (the shared domain logic)
export interface Port {  // was "Connector"
  point: Point;
  direction: Vector;
}

// STEP 2: TypeScript AUTOMATICALLY catches ALL usages across modules
// - js/sketchpad/Design.tsx uses "Connector" → ❌ ERROR
// - js/vscode/extension.ts uses "Connector" → ❌ ERROR
// Every broken reference appears as a compile error

// STEP 3: Single PR, single deploy, no versioning headaches
// If this were microservices, you'd need:
// - Connector Service v1 → Port Service v2 (keep both running)
// - All clients updated to call v2
// - Deprecation period for v1
// - Finally remove v1
// That's WEEKS of work for a simple rename!
```

**Monolith patterns semio uses**:

| Pattern               | semio Implementation                           |
|-----------------------|------------------------------------------------|
| Internal modules      | Nx packages with clear dependencies            |
| Shared kernel         | `js/semio/semio.ts` used by all TypeScript     |
| Domain-driven layers  | UI → State → Domain → Storage                  |
| Bounded contexts      | Kit, Type, Design, Quality as logical units    |

**Why monolith for semio**:

| Reason                    | Explanation                                    |
|---------------------------|------------------------------------------------|
| Small team                | 1-5 developers, can coordinate easily          |
| Shared domain             | Kit/Type/Design used everywhere                |
| Type safety               | TypeScript catches cross-module bugs           |
| Simple operations         | One deployment, not 10 services                |
| Fast refactoring          | Rename across entire codebase instantly        |
| Reduced complexity        | No service discovery, no distributed tracing   |

**What modular monolith enables**

- Simple development (function calls, not HTTP)
- Easy debugging (single process, stack traces)
- Type safety across modules (TypeScript checks all)
- Atomic refactoring (change everywhere at once)
- Fast builds (Nx caching per package)
- Simple deployment (one artifact per target)

**What it limits**

- Must coordinate deploys (but Nx helps)
- Same language per module (but semio uses multiple repos)
- Team boundaries less enforced (but small team anyway)
- Scaling is all-or-nothing (but Engine scales separately)

---

### Chapter 8: How Collaboration Works

#### 8.1 Version Control: Tracking Every Change

**Plain explanation**

Version control records every change ever made to your code. Who changed what, when, and why. You can go back to any previous state, compare versions, and merge work from different people.

semio uses Git for all version control. Every change to TypeScript, Python, C#, Go, documentation—all tracked in one repository.

**Technical explanation**

**semio's version control setup**:

```
┌────────────────────────────────────────────────────────────────────────┐
│                    semio Git Repository                                │
├────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│   .git/                          ← Complete history since 2021          │
│     objects/                     ← All commits, trees, blobs            │
│     refs/heads/main              ← Current branch pointer               │
│     refs/remotes/origin/main     ← GitHub's version                     │
│                                                                         │
│   Working Directory:                                                    │
│     js/semio/semio.ts            ← Domain logic (TypeScript)            │
│     py/engine/engine.py          ← Backend (Python)                     │
│     net/Semio/Semio.cs           ← Grasshopper (C#)                     │
│     go/repo/main.go              ← CLI tools (Go)                       │
│                                                                         │
└────────────────────────────────────────────────────────────────────────┘
```

**semio commit structure**:

```bash
# ============================================================
# EXAMPLE 1: MULTI-LANGUAGE COMMIT
# ============================================================
# Purpose: Show how a single feature change touches multiple languages
#
# When you add a new property like "mandatory" to Connector,
# you update TypeScript, Python, C#, AND regenerate schemas.
# Git tracks ALL of these as ONE atomic change.

# Example: Adding a new Connector property

git add js/semio/semio.ts             # TypeScript: add mandatory field
git add py/engine/engine.py           # Python: add mandatory to model
git add net/Semio/Semio.cs            # C#: add Mandatory property
git add jsonschema/kit.json           # Regenerated JSON Schema

git commit -m "feat(connector): add mandatory flag for required connections"
#             │    │           │
#             │    │           └── Description of what changed
#             │    └── Scope: which part of codebase (connector)
#             └── Type: feat = new feature, fix = bug fix, etc
```

**A semio commit includes**:

| Element         | Example                                          |
|-----------------|--------------------------------------------------|
| SHA-1 hash      | `a7f3b2c...` (unique identifier)                 |
| Parent(s)       | Previous commit(s) this builds on                |
| Author          | `usalu <ueli@semio.design>`                      |
| Timestamp       | `2025-01-15T14:30:00Z`                           |
| Message         | `feat(connector): add mandatory flag...`         |
| Tree            | Snapshot of all files at this point              |

**semio commit history (excerpt)**:

```
* a7f3b2c feat(connector): add mandatory flag for required connections
* 8d2e1a9 fix(validation): handle empty connector list gracefully
* 5c4b3a2 feat(quality): add benchmark system for performance metrics
* 2b1a0f8 refactor(diff): consolidate diff types across languages
|
| (merge commit from feature branch)
|\
| * 9f8e7d6 feat(diagram): add force-directed layout for kit view
| * 6e5d4c3 feat(diagram): node dragging with connection preservation
|/
* 1a2b3c4 feat(type): add Interface for connector compatibility
```

**Viewing semio history**:

```bash
# ============================================================
# EXAMPLE 2: GIT HISTORY COMMANDS
# ============================================================
# Purpose: Show how to explore the project's history
#
# Git stores EVERY change ever made. You can time-travel,
# find who changed what, and understand why decisions were made.

# What changed in semio.ts recently?
git log --oneline js/semio/semio.ts
#       │         └── Only show commits that touched this file
#       └── One commit per line (compact view)

# Who added the Quality type? (git blame shows line-by-line authorship)
git blame js/semio/semio.ts | grep "Quality"
# Output: a7f3b2c (usalu 2025-01-10) export interface Quality {
#         └── commit hash    └── author and date

# What was Kit like 6 months ago? (HEAD~100 = 100 commits ago)
git show HEAD~100:js/semio/semio.ts | grep "interface Kit"
#       │         │
#       │         └── The file path AT that old commit
#       └── "show" displays file content at that point in time

# All changes affecting connectors (search commit messages)
git log --all --oneline --grep="connector"
#       │      │         └── Filter by message content
#       │      └── Compact output
#       └── Search all branches, not just current
```

**semio diff example**:

```diff
# ============================================================
# EXAMPLE 3: READING A DIFF
# ============================================================
# Purpose: Understand how Git shows changes between versions
#
# A "diff" shows the difference between two versions.
# Lines starting with + are ADDED, lines with - are REMOVED.
# Lines without +/- are CONTEXT (unchanged, shown for reference).

# git diff HEAD~1 js/semio/semio.ts
# "What changed in the last commit?"

 export interface Connector {
   id: string;
   point: Point;
   direction: Vector;
+  mandatory?: boolean;     // ← ADDED: This line is new
   interface?: InterfaceId;
 }

# The + at the start means this line was ADDED.
# If there was a - line, that would mean REMOVED.
# No prefix = unchanged context line.
```

**Why version control for semio**

Without Git, semio development would be impossible:

- "Which version of Kit schema is deployed?"
- "Who changed the validation logic?"
- "Can we revert the broken diff system?"
- "How do 3 developers work simultaneously?"

**What it enables for semio**

- Complete history of every model evolution
- Fearless refactoring (always can revert)
- Multiple developers on same files
- Accountability for breaking changes
- Time travel (how did Connector work before Interface?)
- Documentation through commit messages

**What it limits**

- Large 3D model files don't diff well
- Merge conflicts in schema files need manual resolution
- Learning curve for Git commands
- History can become messy without discipline
- Rewriting history breaks collaboration

---

#### 8.2 Git: The Standard Version Control System

**Plain explanation**

Git is the version control system used by almost everyone. Created by Linus Torvalds (who also created Linux), Git is distributed—every developer has the complete history. You can work offline, commit locally, and sync when ready.

semio uses Git exclusively. Every developer has the full 4+ years of semio history locally.

**Technical explanation**

**semio's Git structure**:

```
semio/
├── .git/                    ← Git's internal storage
│   ├── objects/             ← Every file version ever committed
│   ├── refs/
│   │   ├── heads/main       ← Local main branch
│   │   └── remotes/origin/  ← GitHub's branches
│   ├── config               ← Repository settings
│   └── hooks/               ← Automated scripts
│       └── pre-commit       ← Runs before every commit
├── .gitignore               ← Files Git should ignore
└── [all source files]
```

**Common semio Git workflow**:

```bash
# ============================================================
# EXAMPLE 1: DAILY GIT WORKFLOW
# ============================================================
# Purpose: Show the basic commands developers use every day
#
# This is the typical flow: check status → stage changes → commit → push

# 1. Check current state - what files have changed?
git status
#   Output:
#   modified: js/semio/semio.ts      ← Changed but not staged
#   modified: py/engine/engine.py    ← Changed but not staged
#   untracked: new-feature.ts        ← New file Git doesn't know about

# 2. Stage specific changes - mark what goes in the next commit
git add js/semio/semio.ts py/engine/engine.py
#       └── Only add these two files; ignore new-feature.ts for now

# 3. Commit with conventional message - save the staged changes
git commit -m "feat(validation): add constraint for unique connector names"
#           └── -m = message follows in quotes
#               Format: type(scope): description

# 4. Push to GitHub - upload to the shared repository
git push origin main
#              │    └── Branch name (main = primary branch)
#              └── Remote name (origin = GitHub)
```

**semio-specific Git commands**:

```bash
# ============================================================
# EXAMPLE 2: ADVANCED GIT EXPLORATION
# ============================================================
# Purpose: More sophisticated history exploration commands

# Clone semio repository - get a complete local copy
git clone https://github.com/semio/semio.git
cd semio
# Now you have 4+ years of history locally!

# See what files were touched in last 10 commits
git log --oneline --name-only -10
#       │          │           └── Show 10 commits
#       │          └── List changed files for each commit
#       └── Compact one-line format

# Find when Kit interface was last modified
git log -1 --format="%H %s" -- js/semio/semio.ts
#       │   │                   └── Only look at this file
#       │   └── Format: full hash + subject line
#       └── Only show 1 (most recent) commit

# View specific commit details
git show a7f3b2c
#       └── First 7 chars of commit hash is enough

# Compare current with last release (tag v1.0.0)
git diff v1.0.0..HEAD -- js/semio/semio.ts
#       │       │        └── Only this file
#       │       └── HEAD = current commit
#       └── v1.0.0 = the tagged release

# Find all commits by author
git log --author="usalu" --oneline
#       └── Only commits by this person

# Search commit messages for "connector"
git log --grep="connector" --oneline
#       └── Only commits with "connector" in message
```

**semio's .gitignore**:

```gitignore
# ============================================================
# EXAMPLE 3: GITIGNORE - FILES GIT SHOULD NOT TRACK
# ============================================================
# Purpose: Tell Git which files to ignore
#
# Some files should NOT be in version control:
# - Downloaded dependencies (reinstall from package.json)
# - Build outputs (regenerate from source)
# - Secrets (passwords, API keys)
# - OS-specific files (Mac .DS_Store, Windows Thumbs.db)

# ── DEPENDENCIES ────────────────────────────────────────────
# Don't track downloaded packages - reinstall with npm install / pip install
node_modules/           # JavaScript packages (can be 500MB+)
.venv/                  # Python virtual environment

# ── BUILD OUTPUTS ───────────────────────────────────────────
# These are GENERATED from source - no need to track
dist/                   # Vite/webpack build output
build/                  # Generic build folder
*.pyc                   # Compiled Python files
__pycache__/            # Python bytecode cache
bin/                    # .NET compiled binaries
obj/                    # .NET intermediate objects

# ── GENERATED FILES ─────────────────────────────────────────
# Scripts regenerate these from source
reports/*.json          # Linting/validation reports
jsonschema/*.json       # Generated from TypeScript types

# ── IDE SETTINGS ────────────────────────────────────────────
# Personal editor preferences shouldn't be shared
.idea/                  # JetBrains IDE
.vscode/settings.json   # VS Code settings (but .vscode/tasks.json is tracked)

# ── SECRETS ─────────────────────────────────────────────────
# NEVER commit passwords, API keys, or tokens
.env                    # Environment variables
.env.local              # Local overrides

# ── OS FILES ────────────────────────────────────────────────
# Operating system creates these automatically
.DS_Store               # macOS folder metadata
Thumbs.db               # Windows image thumbnails

# ── TEMPORARY ───────────────────────────────────────────────
# Working files that shouldn't persist
temp/                   # Temporary working folder
*.log                   # Log files
```

**Distributed model in semio**:

```
┌───────────────────┐    ┌───────────────────┐    ┌───────────────────┐
│  Developer 1 PC   │    │      GitHub       │    │  Developer 2 PC   │
│                   │    │   (origin)        │    │                   │
│  Full semio       │←──→│  Full semio       │←──→│  Full semio       │
│  history          │    │  history          │    │  history          │
│  (4+ years)       │    │  (canonical)      │    │  (4+ years)       │
└───────────────────┘    └───────────────────┘    └───────────────────┘
       │                                                  │
       │                                                  │
       └──────────────────────────────────────────────────┘
                 Can sync directly (if needed)
```

**Key Git concepts for semio**:

| Concept    | semio Usage                                      |
|------------|--------------------------------------------------|
| Clone      | Get complete semio repo: `git clone ...`         |
| Commit     | Save changes with message: `git commit -m "..."` |
| Branch     | Parallel work: `git branch feature-quality`      |
| Merge      | Combine branches: `git merge feature-quality`    |
| Push       | Upload to GitHub: `git push origin main`         |
| Pull       | Download from GitHub: `git pull origin main`     |
| Diff       | Compare versions: `git diff HEAD~1`              |
| Log        | View history: `git log --oneline`                |
| Stash      | Save work temporarily: `git stash`               |
| Reset      | Undo commits: `git reset HEAD~1`                 |

**Why Git for semio**

Git's design matches semio's needs:

- Fast (local operations for exploring history)
- Distributed (work offline on trains, planes)
- Cheap branching (experiment without fear)
- Cryptographic integrity (commits can't be secretly modified)
- GitHub ecosystem (issues, PRs, Actions)

**What it enables**

- Offline development (commit without internet)
- Parallel feature development (branches)
- Complete history locally (instant blame/log)
- Data integrity (SHA-1 hashes detect corruption)
- Speed (most operations are local)
- Flexibility (many workflow options)

**What it limits**

- Learning curve (many commands)
- Merge conflicts require understanding
- History can get messy (rebase vs merge debates)
- Large binary files are inefficient (3D models)
- Requires discipline for good messages

---

#### 8.3 Branches: Parallel Development

**Plain explanation**

A branch is an independent line of development. The main branch has production code. You create a feature branch to work on something new without affecting main. When done, you merge the branch back.

semio uses a **compressed main branch** strategy—history is squashed for clean reading while development happens in tickets.

**Technical explanation**

**semio's branching strategy (from AGENTS.md)**:

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    semio COMPRESSED MAIN STRATEGY                       │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│   main (compressed/squashed history):                                    │
│   ────●────────●────────●────────●────────●────────                     │
│       │        │        │        │        │                              │
│       │        │        │        │        └─ "feat: add Quality system" │
│       │        │        │        └─ "fix: connector validation"         │
│       │        │        └─ "feat: Y.js collaboration"                   │
│       │        └─ "feat: XState state machine"                          │
│       └─ "initial: semio framework"                                     │
│                                                                          │
│   Development work (tracked via tickets, not branches):                  │
│   ┌───────────────────────────────────────────────────────────────┐     │
│   │ tickets/2025/01/15/QUALITY-BENCHMARKS/                        │     │
│   │   plan.md        ← Planned work                               │     │
│   │   log.md         ← Progress notes                             │     │
│   │   summary.md     ← Completion summary                         │     │
│   └───────────────────────────────────────────────────────────────┘     │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

**semio branch rules**:

| Rule                          | Reason                                        |
|-------------------------------|-----------------------------------------------|
| Never `git stash`             | Multiple agents work concurrently             |
| Never `git checkout` to switch| Would mess up others' work                    |
| Work directly on main         | Tickets track parallel work instead           |
| Squash on merge to main       | Keep history clean and readable               |
| Release branches allowed      | `release/r25.01-1` for parallel release fixes |

**Creating branches (when needed)**:

```bash
# ============================================================
# EXAMPLE 1: BRANCH COMMANDS
# ============================================================
# Purpose: Show how to create and merge branches
#
# Branches are like parallel universes - you can make changes
# without affecting the main timeline, then merge back later.

# For releases that need fixes after main progressed:
git checkout -b release/r25.01-1
#            │   └── Branch name: release for version 25.01, patch 1
#            └── -b = create new branch AND switch to it

# For experimental features (rare):
git checkout -b experiment/wasm-backend
#                └── Naming convention: experiment/ for risky work

# Merge back to main (squashed):
git checkout main                     # Switch to main branch
git merge --squash experiment/wasm-backend
#         └── --squash = combine ALL commits from branch into ONE
#             This keeps main's history clean (one commit per feature)
git commit -m "feat: add WASM backend for browser performance"
#             └── Write a single, clear commit message for all the work
```

**semio branch visualization**:

```
main:  ─────●─────●─────●─────●─────●───── (canonical, clean)
                              ↖
release/r25.01-1:              ●───●───  (parallel fixes for deployed release)
```

**Why semio uses this strategy**:

- **Multiple agents**: Copilot, Cursor, human devs work simultaneously
- **Ticket-based work**: Development is tracked in `tickets/` not branches
- **Clean history**: Squashed commits are readable
- **No context switching**: No stashing, no branch switching

**What branches enable**

- Parallel development (via tickets in semio)
- Feature isolation (until squash-merged)
- Release management (`release/rYY.MM-V` branches)
- Experimentation without risk
- Clean main branch history

**What it limits**

- Long-lived branches diverge (semio avoids these)
- Merge conflicts increase over time
- Stale branches accumulate (semio deletes after merge)

---

#### 8.4 Commits: Snapshots in Time

**Plain explanation**

A commit is a saved snapshot of your project at a moment in time. It records exactly what changed, who made the change, when, and includes a message explaining why.

semio uses **conventional commits** with a specific format that includes a work symbol indicating effort level.

**Technical explanation**

**semio commit message format (from AGENTS.md)**:

```
MAIN-TASK-SYMBOL SUMMARY WORK-SYMBOL

Where WORK-SYMBOL is one of: 🪛 < 🔨 < 🛠️ < 🏗️
(screwdriver < hammer < tools < construction = increasing effort)
```

**semio commit examples**:

```bash
# ============================================================
# EXAMPLE 1: CONVENTIONAL COMMITS WITH WORK SYMBOLS
# ============================================================
# Purpose: Show semio's commit message format
#
# Format: type(scope): summary WORK-SYMBOL
#
# Types: feat (feature), fix (bug fix), refactor, docs, test, chore
# Scope: which part of codebase (connector, validation, quality, etc.)
# Summary: what changed (imperative mood: "add" not "added")
# Work Symbol: effort level (🪛 small → 🏗️ large)

# Small fix (screwdriver 🪛 = quick fix, few lines)
git commit -m "fix(validation): handle null connector list 🪛"

# Medium feature (hammer 🔨 = moderate work, one component)
git commit -m "feat(connector): add mandatory flag for required connections 🔨"

# Large feature (tools 🛠️ = significant work, multiple files)
git commit -m "feat(quality): add benchmark system with benchmarks 🛠️"

# Major refactor (construction 🏗️ = major effort, cross-cutting changes)
git commit -m "refactor(diff): consolidate diff system across all languages 🏗️"
```

**Anatomy of a semio commit**:

```
# ============================================================
# EXAMPLE 2: FULL COMMIT STRUCTURE
# ============================================================
# Purpose: Show what Git stores for each commit
#
# A commit contains: unique ID, author, date, message, and the actual changes

commit 3b4f5a6d8e2c1f0a9b8c7d6e5f4a3b2c1d0e9f8a  ← Unique SHA-1 hash (40 chars)
Author: usalu <ueli@semio.design>                 ← Who made the commit
Date:   Mon Jan 13 10:00:00 2025                  ← When (timestamp)

    feat(connector): add Interface for connector compatibility 🔨
    
    ← First line: summary (shows in git log --oneline)
    ← Blank line separates summary from body
    
    Connectors now reference an Interface for explicit compatibility
    control instead of implicit name matching.
    
    ← Body: detailed explanation of WHY (not just what)
    
    - Add Interface entity to Kit with compatible_interfaces list
    - Add interface field to Connector
    - Update validation to check Interface compatibility
    - Sync schema across TypeScript, Python, C#, Go
    
    ← Bullet list of specific changes
    
    Closes #234
    
    ← Footer: references GitHub issue number
```

**Commit scope conventions in semio**:

| Scope           | Affects                                    |
|-----------------|--------------------------------------------|
| `connector`     | Connector model, validation                |
| `validation`    | Constraint checks, error messages          |
| `diff`          | Diff/patch system                          |
| `quality`       | Quality, Benchmark entities                |
| `sketchpad`     | React UI components                        |
| `engine`        | Python backend                             |
| `grasshopper`   | C# Rhino plugin                            |
| `schema`        | JSON/GraphQL/SQL schema generation         |

**Multi-file atomic commits in semio**:

```bash
# ============================================================
# EXAMPLE 3: ATOMIC CROSS-LANGUAGE COMMITS
# ============================================================
# Purpose: Show how one feature change touches multiple languages
#
# When you add a new entity like "Interface", you must update:
# - TypeScript (domain logic)
# - Python (backend)
# - C# (Grasshopper plugin)
# - Go (CLI tools)
# - JSON Schema (API contract)
# - SQL (database schema)
#
# ALL of these go in ONE commit so the codebase stays consistent.
# If you committed TypeScript alone, Python would be out of sync!

git add js/semio/semio.ts           # TypeScript Interface type
git add py/engine/engine.py         # Python Pydantic model
git add net/Semio/Semio.cs          # C# class
git add go/semio/semio.go           # Go struct
git add jsonschema/kit.json         # JSON schema update
git add sql/sqlite/schema.sql       # SQLite table

git commit -m "feat(interface): add Interface for connector compatibility 🛠️"
# ↑ Single atomic commit keeps all languages in sync
# If someone checks out this commit, everything matches
```

**Finding semio commits**:

```bash
# ============================================================
# EXAMPLE 4: SEARCHING COMMIT HISTORY
# ============================================================
# Purpose: Find specific commits by message, content, or author

# Find all commits mentioning "validation" in message
git log --grep="validation" --oneline
#       └── Search commit MESSAGES for this text

# Find when "Interface" type was first added (search code changes)
git log --all -S "interface Interface" -- js/semio/semio.ts
#             │   └── Search for this STRING in the diff itself
#             └── -S = "pickaxe" - find when string was added/removed

# Who last changed the Connector section?
git blame js/semio/semio.ts | grep "Connector"
#       └── blame shows author for each line

# Find Quality-related changes in semio.ts
git log --oneline -- js/semio/semio.ts | grep "quality"
#                    └── Only commits touching this file
```

**Why commits matter for semio**

Commits are the historical record of every design decision:

- Why was mandatory added to Connector?
- When did we switch from name matching to Interface?
- Who introduced the Quality benchmark system?
- What was the Piece model like before MirrorPlane?

**What commits enable**

- Complete history (git log)
- Blame (who changed this line?)
- Bisect (find when bug introduced)
- Revert (undo specific changes)
- Cherry-pick (move specific commits)
- Understanding past decisions

**What it limits**

- Requires discipline (semio enforces via templates)
- Bad commits clutter history (squash helps)
- Hard to fix bad commits once pushed
- Multi-language changes must be atomic

---

#### 8.5 Pull Requests: Proposing Changes

**Plain explanation**

A pull request (PR) is a way to propose changes and request review. You push your branch, open a PR, describe your changes, and teammates review the code before it's merged. PRs are where code review happens.

semio uses **ticket-based development** instead of traditional PR branches, but still uses PRs for external contributions and major features.

**Technical explanation**

**semio's PR/ticket workflow**:

```
# ============================================================
# EXAMPLE 1: TWO CONTRIBUTION WORKFLOWS
# ============================================================
# Purpose: Show how semio uses DIFFERENT workflows for internal
#          team members vs external community contributors
# Relates to: PRs as a gateway for outside contributors

┌─────────────────────────────────────────────────────────────────────────┐
│                    semio CONTRIBUTION WORKFLOW                           │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  Internal Development (ticket-based, no PR):                            │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  1. Open ticket: repo ticket open FEATURE-NAME                  │    │
│  │     └── Creates tickets/2026/01/14/FEATURE-NAME/ticket.md       │    │
│  │  2. Work on main branch directly                                │    │
│  │     └── No separate branch needed - trusted team member         │    │
│  │  3. Commit with conventional message                            │    │
│  │     └── 🏗️ FEATURE-NAME Add Interface entity 🔨                 │    │
│  │  4. Close ticket: repo ticket close FEATURE-NAME                │    │
│  │     └── Computes git diff stats, archives ticket                │    │
│  │  5. Push squashed commit to main                                │    │
│  │     └── History stays clean (one commit per feature)            │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
│  External Contribution (PR-based):                                       │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  1. Fork semio repository                                       │    │
│  │     └── Creates YOUR-NAME/semio copy on GitHub                  │    │
│  │  2. Create feature branch                                       │    │
│  │     └── git checkout -b add-interface-support                   │    │
│  │  3. Make changes, push to fork                                  │    │
│  │     └── git push origin add-interface-support                   │    │
│  │  4. Open Pull Request to semio/main                             │    │
│  │     └── GitHub UI: "Compare & pull request" button              │    │
│  │  5. CI runs preflight checks                                    │    │
│  │     └── GitHub Actions verifies all tests pass                  │    │
│  │  6. Maintainer reviews                                          │    │
│  │     └── Human checks domain logic, schema sync                  │    │
│  │  7. Squash merge to main                                        │    │
│  │     └── All commits become one clean commit on main             │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘

# Key Insight: PRs exist for TRUST and GATEKEEPING
# - Internal team = trusted, can commit directly
# - External contributors = need review before merging
# - Both paths result in clean, squashed commits on main
```

**semio PR template**:

```markdown
# ============================================================
# EXAMPLE 2: PULL REQUEST TEMPLATE
# ============================================================
# Purpose: Show the STRUCTURE of a good PR description
#          that helps reviewers understand changes quickly
# Relates to: PRs as documentation of what/why/how changed
# File: .github/PULL_REQUEST_TEMPLATE.md

## Description

<!-- What does this PR do? -->
<!-- ^ This is a markdown comment - guides the writer -->
Adds Interface entity for explicit connector compatibility control.
<!-- ^ One sentence summary of the entire PR -->

## Type of Change

<!-- Checkboxes help categorize the change -->
- [ ] Bug fix (non-breaking change that fixes an issue)
- [x] New feature (non-breaking change that adds functionality)
      <!-- ^ [x] means checked/selected -->
- [ ] Breaking change (fix or feature that breaks existing APIs)
      <!-- ^ Breaking = existing code will stop working -->
- [ ] Documentation update

## Changes

<!-- Bullet list of specific changes made -->
- Added `Interface` entity to Kit with `compatible_interfaces` list
  <!-- ^ New data model added to the domain -->
- Added `interface` field to `Connector`
  <!-- ^ Existing model extended with new optional field -->
- Updated validation to check Interface compatibility
  <!-- ^ Business rule: connectors must be compatible -->
- Synced schema across TypeScript, Python, C#, Go
  <!-- ^ Multi-language sync is CRITICAL for semio -->

## Testing

<!-- Checklist of verification steps -->
- [ ] Added/updated unit tests
      <!-- ^ Did you write tests for new code? -->
- [ ] Ran `npm run preflight` successfully
      <!-- ^ Did automated checks pass? -->
- [ ] Tested in Sketchpad manually
      <!-- ^ Did you click around in the UI? -->

## Schema Changes

<!-- Multi-language schema sync checklist -->
- [ ] Updated `js/semio/semio.ts`
      <!-- ^ TypeScript source of truth -->
- [ ] Updated `py/engine/engine.py`
      <!-- ^ Python models match TS -->
- [ ] Updated `net/Semio/Semio.cs`
      <!-- ^ C# models match TS -->
- [ ] Regenerated `jsonschema/kit.json`
      <!-- ^ JSON schema generated from TS -->

## Related Issues

Closes #234
<!-- ^ Magic keyword! GitHub will auto-close issue #234 when PR merges -->
<!-- Other keywords: Fixes, Resolves, References -->
```

<!-- Key Insight: PR templates STANDARDIZE contributions -->
<!-- Every PR has the same structure = faster review -->
<!-- Checklists prevent forgetting steps (like schema sync) -->

**semio PR checks (CI)**:

```yaml
# ============================================================
# EXAMPLE 3: AUTOMATED PR VERIFICATION
# ============================================================
# Purpose: Show how GitHub Actions AUTOMATICALLY tests PRs
#          before humans even look at them
# Relates to: CI as the first line of defense
# File: .github/workflows/ci.yml (simplified)

name: CI
# ^ Name shown in GitHub Actions tab

on: [pull_request]
# ^ Trigger: run this workflow when PR is opened/updated

jobs:
  preflight:
    # First job: run all quality checks
    runs-on: ubuntu-latest
    # ^ Use a Linux virtual machine (GitHub provides free)
    
    steps:
      - uses: actions/checkout@v4
        # ^ Download the PR's code into the VM
        
      - uses: actions/setup-node@v4
        # ^ Install Node.js (needed for npm commands)
        
      - run: npm ci
        # ^ Install dependencies (like npm install, but stricter)
        
      - run: npm run preflight
        # ^ THE KEY LINE: runs fix + analyze + test
        # If this fails, PR gets a red X and can't merge
      
  schema-sync:
    # Second job: verify schemas are in sync
    runs-on: ubuntu-latest
    
    steps:
      - run: npm run schema
        # ^ Regenerate all schemas
        
      - run: |
          if [ -n "$(git status --porcelain)" ]; then
            # ^ Check if regenerating changed any files
            # If so, someone forgot to run npm run schema locally
            echo "Schema out of sync!"
            echo "Run 'npm run schema' and commit the changes"
            exit 1
            # ^ Exit with error = PR fails
          fi

# Key Insight: CI provides CONFIDENCE before human review
# If CI passes: "At least the basics work"
# If CI fails: "Don't bother reviewing yet, fix the basics first"
```

**PR review focus for semio**:

```
# ============================================================
# EXAMPLE 4: WHAT HUMAN REVIEWERS CHECK
# ============================================================
# Purpose: Show what a human reviewer looks for in a semio PR
#          (things automation CAN'T catch)
# Relates to: Human judgment complements automation
```

| Focus Area              | What Reviewers Check                           |
|-------------------------|------------------------------------------------|
| Domain correctness      | Does Kit/Type/Design model make sense?         |
|                         | *Does Interface belong in Kit or Type?*        |
| Schema sync             | All 4 languages updated consistently?          |
|                         | *Did they update C# but forget Go?*            |
| Diff system             | Can changes be diffed and undone?              |
|                         | *If I add a Piece, can I undo that?*           |
| Validation              | New constraints for new fields?                |
|                         | *Interface names must be unique - is that validated?* |
| i18n                    | New UI text in both en.json and de.json?       |
|                         | *"Create Interface" button needs translation*  |
| Test coverage           | New validation constraints have tests?         |
|                         | *Is port-name-unique tested in semio.test.ts?* |

```
# Key Insight: Reviewers focus on JUDGMENT calls
# - "Is this the right design?" (can't automate)
# - "Will this scale?" (requires experience)
# - "Did they miss an edge case?" (domain knowledge)
#
# Automation handles MECHANICAL checks:
# - "Is the code formatted?" (Prettier)
# - "Do types match?" (TypeScript)
# - "Are translations complete?" (i18n.ts)
```

**Why PRs for semio**

PRs provide external contributors a way to:

- Propose changes without direct commit access
- Get feedback before merge
- Run CI checks automatically
- Document the decision to accept/reject

**What it enables**

- External contributions (community PRs)
- Automated quality checks (CI on every PR)
- Code review for major changes
- Documentation of decisions
- Safe main branch (must pass checks)

**What it limits**

- Slower than direct commits (review time)
- Review bottlenecks (maintainer availability)
- Large PRs are hard to review
- CI can take 5-10 minutes

---

#### 8.6 Code Review: Human Verification

**Plain explanation**

Code review is having other developers read your code before it goes into production. They check for bugs, style issues, design problems, and opportunities to improve. It's a second (or third) set of eyes on every change.

semio uses **automated review** (linting, type checking) plus **AI-assisted review** (Copilot, Claude) rather than traditional human PR review for most changes.

**Technical explanation**

**semio's code review layers**:

```
# ============================================================
# EXAMPLE 1: FOUR LAYERS OF CODE REVIEW
# ============================================================
# Purpose: Show how semio uses MULTIPLE review layers
#          instead of relying on slow human PR review
# Relates to: Automation speeds up development velocity

┌─────────────────────────────────────────────────────────────────────────┐
│                    semio CODE REVIEW LAYERS                              │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  Layer 1: Automated Checks (runs on every commit)                       │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  npm run preflight                                              │    │
│  │  ├── fix (Prettier, Ruff)          → Auto-format code           │    │
│  │  │   └── Nobody argues about tabs vs spaces anymore             │    │
│  │  ├── analyze (i18n, code, ts, eslint) → Detect problems        │    │
│  │  │   └── Catches errors BEFORE commit                           │    │
│  │  └── test                          → Run unit tests             │    │
│  │      └── Verify behavior hasn't changed                         │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
│  Layer 2: Type System (compile-time checks)                             │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  TypeScript: Catches type mismatches across js/                 │    │
│  │  └── "You can't assign string to Guid"                          │    │
│  │  Pydantic: Validates Python models at runtime                   │    │
│  │  └── "Missing required field 'name' in Kit"                     │    │
│  │  C# compiler: Catches .NET type errors                          │    │
│  │  └── "Cannot convert Type to Design"                            │    │
│  │  Go compiler: Catches Go type errors                            │    │
│  │  └── "undefined: Interface"                                     │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
│  Layer 3: AI Review (development-time)                                  │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  Copilot/Claude reviews code as you write                       │    │
│  │  └── "Consider using optional chaining here"                    │    │
│  │  MCP tools check policy compliance                              │    │
│  │  └── "This file is missing SPDX license header"                 │    │
│  │  AGENTS.md provides context for AI reviewers                    │    │
│  │  └── "NEVER use inline comments" (AI learns project rules)      │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
│  Layer 4: Human Review (external PRs, major features)                   │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  External contributors get human review                         │    │
│  │  └── Trust boundary: unknown code needs human eyes              │    │
│  │  Breaking schema changes get maintainer review                  │    │
│  │  └── Architecture decisions need human judgment                 │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘

# Key Insight: Layer 1-3 are AUTOMATIC and INSTANT
# Only Layer 4 (human review) requires waiting for someone
# This means 95% of review happens in seconds, not hours
```

**semio code policy checks (automated review)**:

```typescript
// ============================================================
// EXAMPLE 2: POLICY-BASED AUTOMATED REVIEW
// ============================================================
// Purpose: Show specific code POLICIES that are automatically
//          enforced - no human reviewer needed
// Relates to: Codifying team standards into automation
// File: hooks/code.ts

// ❌ Inline comments (forbidden by policy)
const x = 5; // This is a comment - VIOLATION
//           └── Why forbidden? Comments become outdated.
//               Document in AGENTS.md instead.

// ❌ Block comments (forbidden)
/* This is a block comment */ // VIOLATION
// └── Same reason: use AGENTS.md for documentation

// ❌ Empty regions (forbidden)
//#region Empty // VIOLATION - nothing inside
//#endregion
//       └── Why? Regions organize code. Empty region = clutter.

// ✅ License header (required)
// SPDX-License-Identifier: AGPL-3.0-or-later  // OK
// └── Every source file MUST have license header
//     This is a legal requirement for open source

// ✅ DEBUG logs (allowed, but flagged for cleanup)
console.log("[DEBUG] [TICKET-123] piece position:", piece.center);
//           │         │
//           │         └── Ticket identifier for later cleanup
//           └── [DEBUG] prefix = temporary diagnostic

// Key Insight: Policies are MACHINE-READABLE rules
// Instead of: "Please remember to add license headers"
// We have: hooks/code.ts that FAILS if header is missing
// Humans forget. Machines don't.
```
//#region Empty // VIOLATION - nothing inside
//#endregion

// ✅ License header (required)
// SPDX-License-Identifier: AGPL-3.0-or-later  // OK

// ✅ DEBUG logs (allowed, but flagged for cleanup)
console.log("[DEBUG] [TICKET-123] piece position:", piece.center);
```

**Review focus areas for semio**:

```
# ============================================================
# EXAMPLE 3: AUTOMATED VS HUMAN REVIEW RESPONSIBILITIES
# ============================================================
# Purpose: Show EXACTLY what automation handles vs what
#          requires human judgment
# Relates to: Knowing when to trust automation
```

| Area                   | Automated Check                              | Human Check         |
|------------------------|----------------------------------------------|---------------------|
| Code formatting        | Prettier, Ruff (auto-fixed)                  | Never               |
|                        | *Tabs vs spaces? Machine decides.*           |                     |
| Type correctness       | TypeScript, Pydantic, C#, Go compilers       | Never               |
|                        | *"x is number not string" - compiler knows.* |                     |
| Comment policy         | hooks/code.ts                                 | Never               |
|                        | *"No inline comments" - detected automatically.* |                  |
| i18n completeness      | hooks/i18n.ts                                 | Never               |
|                        | *"Missing de.json key" - script detects.*   |                     |
| Schema sync            | npm run schema (CI failure if out of sync)   | Never               |
|                        | *"C# doesn't match TS" - auto-checked.*      |                     |
| Domain design          | Cannot automate                              | Yes (major changes) |
|                        | *"Should Interface be in Kit or Type?"*      |                     |
| API breaking changes   | Cannot automate                              | Yes (maintainer)    |
|                        | *"This removes a field - will it break users?"* |                  |
| UX decisions           | Cannot automate                              | Yes (design review) |
|                        | *"Is this button in the right place?"*       |                     |

```
# Key Insight: Automate the OBJECTIVE, review the SUBJECTIVE
# - "Is the code formatted correctly?" → Objective → Automate
# - "Is this a good API design?" → Subjective → Human judgment
```

**AI-assisted review examples**:

```typescript
// ============================================================
// EXAMPLE 4: AI AS DEVELOPMENT-TIME REVIEWER
// ============================================================
// Purpose: Show how AI (Copilot/Claude) reviews code
//          AS YOU WRITE IT - not after PR is opened
// Relates to: Shifting review LEFT (earlier in process)

// Original code (written by developer):
const connector = type.connectors.find(c => c.id === id);
//                                       └── Find connector with matching id
if (connector) {
  // do something with connector
}
// ^ This works, but there's a better pattern...

// AI suggestion (Copilot/Claude during development):
// "Consider using optional chaining and early return for cleaner code"

// Improved code (after accepting AI suggestion):
const connector = type.connectors.find(c => c.id === id);
//                                       └── Same search logic
if (!connector) return;
//  └── Early return if NOT found (fail fast)
//      This is called the "guard clause" pattern

// do something with connector (now definitely defined)
// ^ TypeScript knows connector is Connector, not undefined
// ^ No nested if statement needed

// Why AI review during development is better than PR review:
// 1. IMMEDIATE feedback (seconds, not hours)
// 2. CONTEXTUAL (AI sees what you're trying to do)
// 3. EDUCATIONAL (you learn the better pattern instantly)
// 4. NO WAITING (no "waiting for reviewer" bottleneck)

// Key Insight: AI shifts review from GATE to GUIDE
// Old: Write code → Open PR → Wait → Get feedback → Fix → Wait → Merge
// New: Write code → AI suggests → Accept/reject → Commit → Push
```

**Why automated review for semio**

Human code review bottlenecks don't scale:

- Small team (1-3 developers)
- High velocity development
- AI agents work 24/7
- Comprehensive type systems catch most bugs

Automated checks + AI assistance > slow human PRs for internal work.

**What it enables**

- Instant feedback (no waiting for reviewer)
- Consistent enforcement (machines don't forget)
- 24/7 review (AI always available)
- Focus human review on design, not formatting
- Scale review with AI agents

**What it limits**

- Cannot catch domain logic errors automatically
- AI may miss subtle bugs
- Breaking changes need human judgment
- New patterns need explicit policy updates

---

#### 8.7 Continuous Integration: Automated Verification

**Plain explanation**

Continuous Integration (CI) automatically builds and tests your code every time you push changes. If tests fail, you know immediately. If the build breaks, you find out before it affects others.

semio uses **Husky pre-commit hooks** for local CI and **GitHub Actions** for remote CI.

**Technical explanation**

**semio's CI pipeline**:

```
# ============================================================
# EXAMPLE 1: LOCAL AND REMOTE CI STAGES
# ============================================================
# Purpose: Show the TWO stages of CI - local (your machine)
#          and remote (GitHub servers)
# Relates to: Catching errors early (local) and for everyone (remote)

┌─────────────────────────────────────────────────────────────────────────┐
│                    semio CI PIPELINE                                     │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  LOCAL CI (pre-commit, runs before every commit):                       │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  .husky/pre-commit                                              │    │
│  │  ├── Prettier       → Format JS/TS/JSON/YAML/MD                 │    │
│  │  │   └── Runs in ~2 seconds                                     │    │
│  │  ├── Ruff           → Format + lint Python                      │    │
│  │  │   └── Much faster than Black + Flake8                        │    │
│  │  ├── i18n check     → Validate translations                     │    │
│  │  │   └── Did you add de.json for new UI text?                   │    │
│  │  ├── TypeScript     → Type check (tsc --noEmit)                 │    │
│  │  │   └── --noEmit = check types but don't produce JS files      │    │
│  │  ├── ESLint         → JS/TS linting                             │    │
│  │  │   └── Catches code quality issues                            │    │
│  │  └── Code policies  → Comments, headers, regions                │    │
│  │      └── semio-specific rules from AGENTS.md                    │    │
│  │                                                                 │    │
│  │  ⏱️ Total time: ~10 seconds (blocks commit if fails)            │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
│  REMOTE CI (GitHub Actions, runs on push/PR):                           │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  .github/workflows/ci.yml                                       │    │
│  │  ├── Build          → npm run build                             │    │
│  │  │   └── Compile all packages in monorepo                       │    │
│  │  ├── Test           → npm run test (Vitest, Playwright)         │    │
│  │  │   └── Unit tests + E2E tests                                 │    │
│  │  ├── Schema sync    → Verify generated files match              │    │
│  │  │   └── Did someone regenerate schemas locally?                │    │
│  │  └── Cross-platform → Run on Windows, macOS, Linux              │    │
│  │      └── Catch platform-specific bugs                           │    │
│  │                                                                 │    │
│  │  ⏱️ Total time: ~5 minutes (blocks merge if fails)              │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘

# Key Insight: TWO GATES protect the codebase
# Gate 1 (Local): Catches YOUR mistakes before commit
# Gate 2 (Remote): Catches EVERYONE's mistakes before merge
# Both must pass for code to reach main branch
```

**semio pre-commit hook** (`.husky/pre-commit`):

```bash
#!/bin/sh
# ============================================================
# EXAMPLE 2: LOCAL PRE-COMMIT HOOK
# ============================================================
# Purpose: Show the ACTUAL script that runs before every commit
#          This is the first line of defense against bad code
# Relates to: Git hooks as automated quality gates
# File: .husky/pre-commit

. "$(dirname "$0")/_/husky.sh"
# ^ Load Husky's helper functions
#   Husky is a tool that makes git hooks easy to manage

# ============================================================
# STEP 1: Run formatters (auto-fix)
# ============================================================
# These CHANGE files to fix formatting issues automatically
# If they change anything, git will include those changes in the commit

npx tsx hooks/prettier.ts
# ^ Run Prettier via TypeScript wrapper
#   Formats: .ts, .tsx, .js, .json, .yaml, .md files
#   This is why semio code always looks consistent

npx tsx hooks/ruff.ts
# ^ Run Ruff (Python formatter + linter)
#   Formats: .py files
#   Ruff is MUCH faster than Black + Flake8 combined

# ============================================================
# STEP 2: Run linters (generate reports)
# ============================================================
# These DON'T change files - they just report problems
# Results go into reports/*.json for later analysis

npx tsx hooks/i18n.ts
# ^ Check that all UI text has translations
#   Reports: Missing keys, unused keys, incomplete translations

npx tsx hooks/code.ts
# ^ Check semio-specific code policies
#   Reports: Inline comments, missing headers, empty regions

npx tsx hooks/typescript.ts
# ^ Run TypeScript compiler in check mode
#   Reports: Type errors across all .ts/.tsx files

npx tsx hooks/eslint.ts
# ^ Run ESLint for JavaScript/TypeScript
#   Reports: Code quality issues, potential bugs

# ============================================================
# STEP 3: Check if any linter reported errors
# ============================================================
if [ -s reports/typescript.json ] || [ -s reports/eslint.json ]; then
  # ^ -s = "file exists AND is not empty"
  #   If either report has content, there are errors
  
  echo "❌ Linting errors found. Check reports/ folder."
  exit 1
  # ^ exit 1 = FAILURE
  #   Git will ABORT the commit
  #   You must fix the errors before committing
fi

# If we get here, all checks passed!
# Git will proceed with the commit
```

**semio GitHub Actions workflow**:

```yaml
# ============================================================
# EXAMPLE 3: REMOTE CI WITH GITHUB ACTIONS
# ============================================================
# Purpose: Show how GitHub AUTOMATICALLY runs tests when you push
#          This runs on GitHub's servers, not your machine
# Relates to: CI as a shared quality gate for the whole team
# File: .github/workflows/ci.yml

name: CI
# ^ Display name in GitHub Actions tab

on:
  push:
    branches: [main]
    # ^ Run when code is pushed directly to main
  pull_request:
    branches: [main]
    # ^ Run when PR is opened or updated against main

jobs:
  # ============================================================
  # JOB 1: PREFLIGHT - Basic quality checks
  # ============================================================
  preflight:
    runs-on: ubuntu-latest
    # ^ Use a fresh Ubuntu Linux virtual machine
    #   GitHub provides these for free (for public repos)
    
    steps:
      - uses: actions/checkout@v4
        # ^ Download the repository code into the VM
        #   Without this, there's no code to test!
      
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'
          # ^ Install Node.js version 20 (LTS)
          cache: 'npm'
          # ^ Cache node_modules between runs (faster)
      
      - name: Install dependencies
        run: npm ci
        # ^ npm ci = "clean install"
        #   Faster and stricter than npm install
        #   Uses exact versions from package-lock.json
      
      - name: Run preflight
        run: npm run preflight
        # ^ THE MAIN CHECK
        #   Runs: fix + analyze + test
        #   If this fails, the PR gets a red X
      
      - name: Upload reports
        uses: actions/upload-artifact@v4
        with:
          name: reports
          path: reports/
        # ^ Save the reports so we can download and inspect them
        #   Useful for debugging CI failures

  # ============================================================
  # JOB 2: BUILD - Compile all packages
  # ============================================================
  build:
    needs: preflight
    # ^ Only run if preflight passes
    #   No point building broken code
    
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
      - run: npm ci
      - run: npm run build
        # ^ Compile TypeScript, bundle packages, etc.
      
  # ============================================================
  # JOB 3: TEST - Run tests on multiple platforms
  # ============================================================
  test:
    needs: build
    # ^ Only run if build passes
    
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        # ^ Run the SAME tests on THREE operating systems
        #   This catches platform-specific bugs
    
    runs-on: ${{ matrix.os }}
    # ^ Use the OS from the matrix
    #   Creates 3 parallel jobs (Linux, Windows, macOS)
    
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
      - run: npm ci
      - run: npm test
        # ^ Run all tests
        #   Vitest for unit tests
        #   Playwright for E2E tests

# Key Insight: CI creates a SHARED QUALITY GATE
# - Everyone's code is tested the same way
# - Platform-specific bugs are caught early
# - Green checkmark = "safe to merge"
```

**semio CI commands**:

```
# ============================================================
# EXAMPLE 4: CI COMMAND HIERARCHY
# ============================================================
# Purpose: Show how CI commands BUILD ON each other
#          Each command includes the previous ones
# Relates to: Progressive verification (fix → analyze → test → build)
```

| Command               | What It Does                                 |
|-----------------------|----------------------------------------------|
| `npm run fix`         | Prettier + Ruff formatting                   |
|                       | *Fastest - just auto-fix formatting*         |
| `npm run analyze`     | i18n + code + TypeScript + ESLint checks     |
|                       | *Medium - find problems without fixing*      |
| `npm run preflight`   | fix + analyze (runs both)                    |
|                       | *Standard - format + check (pre-commit)*     |
| `npm run test`        | preflight + nx run-many -t test              |
|                       | *Thorough - everything + unit tests*         |
| `npm run build`       | test + nx run-many -t build                  |
|                       | *Complete - everything + compile packages*   |

```
# Visual: Each command is a SUPERSET of the previous
#
# fix
# └── analyze
#     └── preflight = fix + analyze
#         └── test = preflight + tests
#             └── build = test + compile
#
# So "npm run build" does EVERYTHING
```

**Skip mechanism for development**:

```bash
# ============================================================
# EXAMPLE 5: SKIPPING CHECKS DURING RAPID DEVELOPMENT
# ============================================================
# Purpose: Show how to BYPASS slow checks when iterating quickly
#          (Use sparingly - CI exists for a reason!)
# Relates to: Developer experience vs quality tradeoffs

# Skip all pre-checks during rapid iteration
npm run test -- --skip=preflight
#            │   └── Skip the preflight step (fix + analyze)
#            └── Everything after -- is passed to the script

# Skip specific checks
npm run test -- --skip=fix,analyze
#                      └── Comma-separated list of steps to skip

# Pass arguments directly to Nx (monorepo task runner)
npm run test -- --nx --projects=@semio/js
#               │    └── Only test the @semio/js package
#               └── --nx means "pass remaining args to Nx"

# Why would you skip?
# - You're debugging one test, don't need full preflight
# - You're iterating on TypeScript, Prettier is slowing you down
# - You know your change only affects one package

# ⚠️ WARNING: Don't skip habitually!
# The checks exist for a reason. CI will still run everything.
# Skipping locally just delays finding the problems.
```

**Generated reports**:

```bash
# ============================================================
# EXAMPLE 6: CI REPORT FILES
# ============================================================
# Purpose: Show WHERE CI stores its findings
#          These files are checked by the pre-commit hook
# Relates to: Machine-readable CI output

reports/
├── i18n.json        # Translation validation
│                    # Missing keys, unused keys, etc.
├── eslint.json      # ESLint problems
│                    # Code quality issues, potential bugs
├── code.json        # Policy violations
│                    # Inline comments, missing headers
├── typescript.json  # TypeScript errors
│                    # Type mismatches, undefined variables
└── ruff.json        # Python linting
                     # Python code quality issues

# Key Insight: Reports are JSON (machine-readable)
# This means OTHER tools can consume them:
# - VS Code extension shows violations as squiggly lines
# - MCP tools can read and act on violations
# - CI can check "is any report non-empty?" to fail
```

**Why CI for semio**

Without CI:

- "Did you run preflight?" "I forgot"
- TypeScript errors on main branch
- Broken i18n missing translations
- Schema sync issues discovered late

CI ensures every commit is verified automatically.

**What it enables**

- Immediate feedback on changes
- Confidence in main branch
- Consistent verification across languages
- Catch cross-platform issues (Windows, macOS, Linux)
- Enforce quality gates before merge

**What it limits**

- Pre-commit adds ~10 seconds per commit
- GitHub Actions adds ~3-5 minutes for full pipeline
- Flaky tests undermine trust
- Complex multi-language pipeline to maintain
- False sense of security (tests aren't everything)
- Setup overhead

---

#### 8.8 Continuous Deployment: From Code to Production

**Plain explanation**

Continuous Deployment (CD) automatically deploys code to production when it passes all tests. Push code, tests pass, it's live. No manual steps, no waiting for a release.

semio has **multiple deployment targets** with different CD strategies for each platform.

**Technical explanation**

**semio deployment targets**:

```
# ============================================================
# EXAMPLE 1: MULTIPLE DEPLOYMENT TARGETS
# ============================================================
# Purpose: Show how semio deploys to DIFFERENT platforms
#          Each has its own deployment strategy and tooling
# Relates to: CD adapts to target platform requirements

┌─────────────────────────────────────────────────────────────────────────┐
│                    semio DEPLOYMENT TARGETS                              │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  Documentation (js/docs) → GitHub Pages                                 │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  Push to main → Build Astro → Deploy to gh-pages branch         │    │
│  │  URL: https://semio.design                                      │    │
│  │  Strategy: Instant replacement (static site)                    │    │
│  │  └── Fully automatic: push code → site updates in ~2 minutes    │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
│  VS Code Extension (js/vscode) → Marketplace                            │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  npm run publish:vscode → Build VSIX → Upload to marketplace    │    │
│  │  Strategy: Manual trigger (breaking changes need changelog)     │    │
│  │  └── Requires: version bump, CHANGELOG update, human decision   │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
│  Grasshopper Plugin (net/Semio.Grasshopper) → Yak (Rhino package)       │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  npm run publish:yak → Build DLL → Upload to yak.rhino3d.com    │    │
│  │  Strategy: Manual trigger (Rhino version compatibility)        │    │
│  │  └── Requires: Rhino 7/8 compatibility testing, signing        │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
│  Desktop App (js/desktop) → GitHub Releases                             │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  npm run publish:desktop → electron-builder → Upload release    │    │
│  │  Strategy: Manual trigger (platform signing, notarization)     │    │
│  │  └── Requires: Windows code signing, macOS notarization        │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘

# Key Insight: NOT everything is fully automatic
# - Docs: Safe to auto-deploy (static, easy to rollback)
# - Extensions/Apps: Need human decision (breaking changes, signing)
# CD = "automatically CAN deploy", not "automatically ALWAYS deploy"
```

**GitHub Pages deployment (docs)**:

```yaml
# ============================================================
# EXAMPLE 2: AUTOMATIC DOCUMENTATION DEPLOYMENT
# ============================================================
# Purpose: Show FULLY AUTOMATIC deployment of documentation
#          Push to main → site updates automatically
# Relates to: CD for low-risk deployments
# File: .github/workflows/gh-pages.yml

name: Deploy Documentation
# ^ Name shown in GitHub Actions tab

on:
  push:
    branches: [main]
    # ^ Trigger: only when pushing to main branch
    paths:
      - 'js/docs/**'
      # ^ Only when docs files change
      - 'README.md'
      # ^ Or when README changes (it's used in docs)
      # This prevents unnecessary deploys when only TypeScript changes

jobs:
  deploy:
    runs-on: ubuntu-latest
    # ^ Use GitHub's Linux VM
    
    steps:
      - uses: actions/checkout@v4
        # ^ Download repository code
      
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'
          # ^ Need Node.js to build Astro site
      
      - name: Install and build
        run: |
          npm ci
          # ^ Install all dependencies
          npm run build -w @semio/docs
          # ^ Build only the docs workspace
          #   -w = workspace flag for npm
          #   This runs: cd js/docs && astro build
      
      - name: Deploy to GitHub Pages
        uses: peaceiris/actions-gh-pages@v4
        # ^ Third-party action that handles gh-pages deployment
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          # ^ GitHub automatically provides this secret
          #   No manual configuration needed
          publish_dir: ./js/docs/dist
          # ^ Deploy the built files from dist/

# Key Insight: This runs on EVERY push that touches docs
# No manual step needed - write docs, push, site updates
# Rollback = revert commit and push again
```

**Yak (Grasshopper) deployment**:

```typescript
// ============================================================
// EXAMPLE 3: MANUAL GRASSHOPPER PLUGIN DEPLOYMENT
// ============================================================
// Purpose: Show MANUAL deployment to Rhino's package manager
//          This requires human decision due to compatibility risks
// Relates to: CD for high-risk deployments (third-party ecosystem)
// File: yak/publish.ts

import { execSync } from 'child_process';
// ^ Node.js built-in for running shell commands

// Step 1: Build the Grasshopper plugin (.gha DLL)
execSync('dotnet build net/Semio.Grasshopper -c Release');
//        │                                    └── Release configuration
//        │                                        (optimized, no debug info)
//        └── Build the C# project
//            Output: net/Semio.Grasshopper/bin/Release/Semio.Grasshopper.gha

// Step 2: Create Yak package (.yak file)
execSync('yak build', { cwd: 'yak' });
//        │           └── Run in yak/ directory
//        └── yak CLI tool from Rhino
//            Reads manifest.yml, creates Semio-0.1.0.yak

// Step 3: Publish to Yak repository
execSync('yak push *.yak', { cwd: 'yak' });
//        │                └── Run in yak/ directory
//        └── Upload to yak.rhino3d.com
//            Users can now install via Rhino Package Manager

// Why manual?
// 1. Rhino 7 vs Rhino 8 compatibility must be tested
// 2. Breaking changes affect architect workflows
// 3. Grasshopper definitions might break
// 4. No automatic rollback - must "yank" published version
```

**semio deployment strategies by target**:

```
# ============================================================
# EXAMPLE 4: DEPLOYMENT STRATEGY COMPARISON
# ============================================================
# Purpose: Compare HOW each target is deployed and rolled back
# Relates to: Matching CD strategy to platform constraints
```

| Target           | Strategy            | Trigger       | Rollback                    |
|------------------|---------------------|---------------|------------------------------|
| Docs (gh-pages)  | Instant replacement | Auto on push  | Revert commit, redeploy      |
|                  | *Just overwrite*    | *Every push*  | *Git handles history*        |
| VS Code          | Marketplace upload  | Manual        | Publish previous version     |
|                  | *Submit to store*   | *Human runs*  | *Upload old .vsix*           |
| Grasshopper (Yak)| Package upload      | Manual        | Yank version, upload previous|
|                  | *Rhino pkg mgr*     | *Human runs*  | *yak yank, then yak push*    |
| Desktop          | GitHub Release      | Manual        | Delete release, create new   |
|                  | *Binary download*   | *Human runs*  | *GitHub API*                 |
| Engine (Docker)  | Rolling update      | Manual        | kubectl rollout undo         |
|                  | *Replace containers*| *Human runs*  | *K8s keeps previous version* |

```
# Key Insight: Rollback difficulty varies
# - Docs: Easy (git revert)
# - Published packages: Hard (users already downloaded)
# - Desktop: Medium (users can download old release)
```

**Feature flags in Sketchpad**:

```typescript
// ============================================================
// EXAMPLE 5: FEATURE FLAGS FOR SAFE DEPLOYMENT
// ============================================================
// Purpose: Show how to deploy code that's OFF by default
//          Turn features on gradually without redeploying
// Relates to: Decoupling deployment from release
// File: js/semio/sketchpad/Sketchpad.tsx

const featureFlags = {
  // Environment variable flags (set at build time)
  enableAIChat: import.meta.env.VITE_ENABLE_AI_CHAT === 'true',
  //            └── Vite reads VITE_* env vars
  //                Set in .env file or CI environment
  
  enableCollaboration: import.meta.env.VITE_ENABLE_COLLAB === 'true',
  //                   └── Real-time multiplayer editing
  //                       Requires backend, so off by default
  
  // Hard-coded flags (change requires redeploy)
  enableExperimentalQuality: false,
  //                         └── Quality system not ready
  //                             Flip to true when stable
};

// Usage in component (conditional rendering):
{featureFlags.enableAIChat && <AIChatPanel />}
// │                          └── Render chat panel
// └── Short-circuit: if flag is false, don't render

// Why feature flags?
// 1. Deploy code to production but keep it hidden
// 2. Enable for internal testing before public release
// 3. Quick disable if something breaks (no redeploy)
// 4. Gradual rollout (enable for 10% of users)

// Key Insight: DEPLOYMENT ≠ RELEASE
// Deployment: Code reaches production servers
// Release: Users can access the feature
// Feature flags let you deploy now, release later
```

**Why CD for semio**

Different targets have different CD needs:

- **Docs**: Fast iteration, low risk → auto-deploy
- **Extensions**: Breaking changes possible → manual release
- **Desktop**: Platform signing required → manual release
- **Plugins**: Third-party ecosystem → careful versioning

**What it enables**

- Docs updates live in minutes
- Extension updates available same day
- Desktop releases with proper signing
- Plugin compatibility tracking
- User feedback loop via GitHub issues

**What it limits**

- Multiple deployment pipelines to maintain
- Platform-specific signing requirements
- Marketplace review delays (VS Code)
- Can't auto-rollback easily for published packages

---

### Chapter 9: How Data Is Managed

#### 9.1 Databases: Structured Data Storage

**Plain explanation**

A database is organized storage for data. Unlike files where you manage everything yourself, databases provide structure, searching, relationships, and guarantees. Your bank account balance, your social media posts, your shopping cart—all live in databases.

semio uses **SQLite** for kit storage—a file-based relational database that's embedded directly in the application.

**Technical explanation**

**semio's database choice: SQLite**

```
# ============================================================
# EXAMPLE 1: DATABASE ARCHITECTURE FOR KIT STORAGE
# ============================================================
# Purpose: Show how semio stores kit data in DIFFERENT contexts
#          Static files use SQLite, browser uses IndexedDB
# Relates to: Choosing the right database for each use case

┌─────────────────────────────────────────────────────────────────────────┐
│                    semio DATABASE ARCHITECTURE                           │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  Static Kit (zip file):                                                 │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  my-kit.zip                                                     │    │
│  │  ├── .semio/                                                    │    │
│  │  │   └── kit.db     ← SQLite database with all kit data        │    │
│  │  │       └── Types, Designs, Connectors, Pieces, etc.          │    │
│  │  ├── models/                                                    │    │
│  │  │   ├── wall.glb   ← 3D model files (binary, not in DB)       │    │
│  │  │   └── beam.glb       (too large for database)               │    │
│  │  └── images/                                                    │    │
│  │      └── preview.png    (referenced by DB, stored as files)    │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
│  Browser (dynamic kit):                                                  │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  IndexedDB (browser storage)                                    │    │
│  │  ├── Y.js documents for collaborative editing                   │    │
│  │  │   └── CRDT-based, syncs across users                        │    │
│  │  └── Kit snapshots for persistence                              │    │
│  │      └── Survives page refresh                                  │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘

# Key Insight: Different storage for different contexts
# - Sharing kits: SQLite in zip (portable, no server)
# - Live editing: IndexedDB with Y.js (real-time, collaborative)
# - Both can represent the same Kit - just different persistence
```

**Why SQLite for semio**:

```
# ============================================================
# EXAMPLE 2: WHY SQLITE IS PERFECT FOR SEMIO
# ============================================================
# Purpose: Show how SQLite matches semio's specific requirements
# Relates to: Database selection based on use case constraints
```

| Requirement              | SQLite Solution                              |
|--------------------------|----------------------------------------------|
| Portable kit files       | Single .db file inside .zip                  |
|                          | *Email a kit = email a file*                 |
| No server needed         | Embedded, runs in-process                    |
|                          | *No PostgreSQL or MySQL to install*          |
| Offline support          | File-based, no network required              |
|                          | *Architects can work on planes*              |
| Cross-platform           | Works on Windows, macOS, Linux, browser      |
|                          | *Same kit.db opens everywhere*               |
| Relational queries       | Full SQL support for complex joins           |
|                          | *"All pieces connected to this type"*        |
| ACID transactions        | Kit saves are atomic                         |
|                          | *Either ALL changes save or NONE*            |

```
# Key Insight: SQLite is the most-deployed database in the world
# - Every iPhone has SQLite
# - Every Android has SQLite
# - Every web browser has SQLite
# It's not "lesser than" PostgreSQL - it's PURPOSE-BUILT for embedding
```

**semio SQLite schema** (`sql/sqlite/schema.sql`):

```sql
-- ============================================================
-- EXAMPLE 3: SEMIO'S DATABASE SCHEMA
-- ============================================================
-- Purpose: Show the STRUCTURE of semio's SQLite database
--          This defines WHAT data is stored and HOW it relates
-- Relates to: Relational database design
-- File: sql/sqlite/schema.sql

-- ============================================================
-- CORE KIT METADATA
-- ============================================================
-- The kit table stores TOP-LEVEL information about the kit itself
CREATE TABLE kit (
    guid TEXT PRIMARY KEY,    -- Globally Unique ID (like 'abc-123-def-456')
    name TEXT NOT NULL,       -- Human-readable name (like "Metabolism Kit")
    version TEXT,             -- Semantic version (like "1.0.0")
    description TEXT,         -- What this kit is for
    remote_url TEXT,          -- Where to download updates
    homepage_url TEXT,        -- Documentation link
    license TEXT,             -- SPDX license (like "AGPL-3.0-or-later")
    icon TEXT,                -- Path to icon file
    image TEXT                -- Path to preview image
);
-- ^ ONE row per kit (a kit.db file has exactly ONE kit)

-- ============================================================
-- TYPES (reusable building blocks)
-- ============================================================
-- Types are like "classes" in OOP - templates for pieces
CREATE TABLE type (
    guid TEXT PRIMARY KEY,                    -- Unique ID for this type
    kit_guid TEXT REFERENCES kit(guid),       -- Which kit owns this type
    parent_guid TEXT REFERENCES type(guid),   -- Parent for inheritance (subtype)
    name TEXT NOT NULL,                       -- Name like "Wall" or "Column"
    variant TEXT,                             -- Variation like "Loadbearing"
    is_virtual BOOLEAN DEFAULT FALSE,         -- Can't be instantiated directly?
    can_scale BOOLEAN DEFAULT TRUE,           -- Allowed to resize?
    can_mirror BOOLEAN DEFAULT TRUE,          -- Allowed to flip?
    unit TEXT,                                -- Unit for dimensions
    available_count INTEGER,                  -- Stock quantity
    description TEXT,                         -- What this type represents
    icon TEXT,                                -- Path to icon
    image TEXT                                -- Path to preview
);
-- ^ Many types per kit

-- ============================================================
-- CONNECTORS (connection points on types)
-- ============================================================
-- Connectors define WHERE two pieces can connect
CREATE TABLE connector (
    guid TEXT PRIMARY KEY,                        -- Unique ID
    type_guid TEXT REFERENCES type(guid),         -- Which type has this connector
    id TEXT NOT NULL,                             -- Local ID within type (like "top")
    name TEXT,                                    -- Human name (like "Top Connection")
    -- Point in 3D space (where connector is located)
    point_x REAL DEFAULT 0,                       -- X coordinate
    point_y REAL DEFAULT 0,                       -- Y coordinate
    point_z REAL DEFAULT 0,                       -- Z coordinate
    -- Direction vector (which way it faces)
    direction_x REAL DEFAULT 0,                   -- X component of direction
    direction_y REAL DEFAULT 1,                   -- Y component (default: forward)
    direction_z REAL DEFAULT 0,                   -- Z component
    t REAL DEFAULT 0,                             -- Diagram ring position (0-1)
    mandatory BOOLEAN DEFAULT FALSE,              -- Must be connected?
    interface_guid TEXT REFERENCES interface(guid) -- Compatibility group
);
-- ^ Many connectors per type

-- ============================================================
-- DESIGNS (compositions of pieces)
-- ============================================================
-- Designs are arrangements of pieces - the actual "building"
CREATE TABLE design (
    guid TEXT PRIMARY KEY,                      -- Unique ID
    kit_guid TEXT REFERENCES kit(guid),         -- Which kit owns this design
    parent_guid TEXT REFERENCES design(guid),   -- Parent for hierarchy (subdesign)
    name TEXT NOT NULL,                         -- Name like "Tower A"
    variant TEXT,                               -- Variation like "With Courtyard"
    description TEXT,                           -- What this design represents
    icon TEXT,                                  -- Path to icon
    image TEXT                                  -- Path to preview
);
-- ^ Many designs per kit

-- ============================================================
-- PIECES (instances of types in designs)
-- ============================================================
-- A piece is a TYPE placed in a DESIGN at a specific location
CREATE TABLE piece (
    guid TEXT PRIMARY KEY,                          -- Unique ID
    design_guid TEXT REFERENCES design(guid),       -- Which design contains this
    type_guid TEXT REFERENCES type(guid),           -- What type is this piece
    subdesign_guid TEXT REFERENCES design(guid),    -- Or it's a subdesign
    id TEXT NOT NULL,                               -- Local ID within design
    name TEXT,                                      -- Human name
    -- Plane defines position and orientation in 3D space
    -- (9 values: origin xyz, x-axis xyz, y-axis xyz)
    plane_origin_x REAL, plane_origin_y REAL, plane_origin_z REAL,
    plane_x_axis_x REAL, plane_x_axis_y REAL, plane_x_axis_z REAL,
    plane_y_axis_x REAL, plane_y_axis_y REAL, plane_y_axis_z REAL,
    scale REAL DEFAULT 1.0,                         -- Size multiplier
    is_hidden BOOLEAN DEFAULT FALSE,                -- Visible in viewport?
    is_locked BOOLEAN DEFAULT FALSE,                -- Can be edited?
    color TEXT                                      -- Override color (#RRGGBB)
);
-- ^ Many pieces per design

-- ============================================================
-- CONNECTIONS (links between pieces)
-- ============================================================
-- Connections define HOW pieces relate spatially
CREATE TABLE connection (
    guid TEXT PRIMARY KEY,                                -- Unique ID
    design_guid TEXT REFERENCES design(guid),             -- Which design
    connected_piece_guid TEXT REFERENCES piece(guid),     -- "From" piece
    connected_connector_id TEXT,                          -- Which connector on "from"
    connecting_piece_guid TEXT REFERENCES piece(guid),    -- "To" piece
    connecting_connector_id TEXT,                         -- Which connector on "to"
    -- Translation parameters (offset between connectors)
    gap REAL DEFAULT 0,    -- Y offset (forward/backward)
    shift REAL DEFAULT 0,  -- X offset (left/right)
    rise REAL DEFAULT 0,   -- Z offset (up/down)
    -- Rotation parameters (how "to" piece is rotated relative to "from")
    rotation REAL DEFAULT 0,  -- Around Y axis
    turn REAL DEFAULT 0,      -- Around Z axis
    tilt REAL DEFAULT 0       -- Around X axis
);
-- ^ Many connections per design

-- Key Insight: This is a RELATIONAL schema
-- Tables REFERENCE each other via FOREIGN KEYS
-- connector.type_guid → type.guid (connector belongs to type)
-- piece.design_guid → design.guid (piece belongs to design)
-- This allows EFFICIENT queries like "all connectors for type X"
```

**Database types comparison**:

```
# ============================================================
# EXAMPLE 4: DATABASE TYPES AND WHEN TO USE THEM
# ============================================================
# Purpose: Compare different database technologies
# Relates to: Choosing the right tool for the job
```

| Type          | semio Usage               | Example                   |
|---------------|---------------------------|---------------------------|
| SQLite        | Kit storage (primary)     | `.semio/kit.db`           |
|               | *Embedded, file-based*    | *Portable, offline*       |
| IndexedDB     | Browser persistence       | Y.js documents            |
|               | *Browser-native storage*  | *Survives page refresh*   |
| PostgreSQL    | (Not used yet)            | Future cloud backend      |
|               | *Server database*         | *Multi-user, scalable*    |
| Key-Value     | (Not used)                | Redis for caching         |
|               | *Fast lookups by key*     | *Session storage*         |

```
# Key Insight: Different databases for different needs
# - SQLite: "I need portable, single-user, embedded"
# - PostgreSQL: "I need multi-user, server, scalable"
# - IndexedDB: "I need browser storage, offline-capable"
# - Redis: "I need fast, temporary, shared cache"
#
# semio uses SQLite because kits are FILES that users share
```

**Why databases for semio**

Without SQLite:

- Query types by name → scan entire JSON
- Find all pieces using type → scan all designs
- Update one connector → rewrite entire type
- Concurrent edits → corruption risk

SQLite provides efficient queries and atomic updates.

**What it enables**

- Efficient queries (find type by guid in O(log n))
- Relationships (piece → type, connection → pieces)
- Atomic transactions (kit save is all-or-nothing)
- Portable files (kit.db inside .zip)
- Cross-language support (SQLite in Python, C#, Go)

**What it limits**

- Schema migrations needed for new fields
- SQL learning curve
- Binary files (models) stored separately
- Browser requires IndexedDB (no native SQLite)

---

#### 9.2 SQL: The Language of Relational Data

**Plain explanation**

SQL (Structured Query Language) is how you talk to relational databases. You write queries to read data, statements to modify it, and commands to define structure. SQL has been around since the 1970s and remains the standard.

semio uses SQL in the Engine (Python) to query kit.db files for efficient data access.

**Technical explanation**

**semio SQL examples**:

```sql
-- ============================================================
-- EXAMPLE 1: BASIC SQL QUERIES FOR KIT DATA
-- ============================================================
-- Purpose: Show how SQL lets you ASK QUESTIONS about kit data
--          Each query is like asking "give me all X where Y"
-- Relates to: SQL as a declarative query language

-- Find a type by guid
SELECT * FROM type WHERE guid = 'abc123-def456';
--     │              └── Filter: only rows where guid matches
--     └── Select ALL columns from the type table

-- Get all connectors for a type
SELECT c.* 
FROM connector c               -- "c" is an alias (short name for table)
WHERE c.type_guid = 'abc123-def456';
--    └── Filter: only connectors belonging to this type

-- Find all pieces using a specific type
SELECT p.id, p.name, d.name as design_name
--     │     │        └── Rename column in output
--     │     └── piece.name
--     └── piece.id
FROM piece p
JOIN design d ON p.design_guid = d.guid
--        │      └── Match condition: piece's design = design's guid
--        └── Combine piece and design tables
WHERE p.type_guid = 'abc123-def456';
--    └── Filter: only pieces of this type
-- This tells you: "Where is this type used across all designs?"

-- Get connection graph for a design
SELECT 
    c.connected_piece_guid,    -- "From" piece
    c.connecting_piece_guid,   -- "To" piece
    c.gap, c.shift, c.rise,    -- Translation parameters
    c.rotation, c.turn, c.tilt -- Rotation parameters
FROM connection c
WHERE c.design_guid = 'xyz789-uvw012';
-- This returns all edges in the design's connection GRAPH
-- You could visualize this as nodes (pieces) and edges (connections)
```

**Engine SQL queries** (`py/engine/engine.py`):

```python
# ============================================================
# EXAMPLE 2: PYTHON + SQL FOR KIT LOADING
# ============================================================
# Purpose: Show how Python code uses SQL to load kit data
#          This bridges the gap between SQL and Python objects
# Relates to: Object-Relational Mapping (ORM) concepts
# File: py/engine/engine.py

import sqlite3
# ^ Python's built-in SQLite library (no install needed!)

from pathlib import Path
# ^ Modern path handling (cross-platform)

def load_kit_from_db(kit_path: Path) -> Kit:
    """Load a Kit from SQLite database."""
    
    # Step 1: Connect to the database file
    conn = sqlite3.connect(kit_path / '.semio' / 'kit.db')
    #                      └── Path to kit.db inside the kit folder
    
    conn.row_factory = sqlite3.Row
    # ^ Magic setting: lets us access columns by NAME not index
    #   row['name'] instead of row[1]
    
    cursor = conn.cursor()
    # ^ Cursor is like a "pointer" for executing queries
    
    # Step 2: Load kit metadata (only one row)
    cursor.execute("SELECT * FROM kit LIMIT 1")
    #              └── SQL query as a string
    kit_row = cursor.fetchone()
    #         └── Get the first (and only) result row
    
    # Step 3: Load all types belonging to this kit
    cursor.execute("SELECT * FROM type WHERE kit_guid = ?", (kit_row['guid'],))
    #                                                │      └── Parameters tuple
    #                                                └── ? = placeholder (prevents SQL injection!)
    
    types = []
    for type_row in cursor.fetchall():
        #           └── Iterate through ALL matching rows
        
        # Step 4: For each type, load its connectors
        cursor.execute(
            "SELECT * FROM connector WHERE type_guid = ?", 
            (type_row['guid'],)
        )
        
        # Step 5: Convert SQL rows to Python objects
        connectors = [
            Connector(
                id=c['id'],                    # Access by column name
                point=Point(c['point_x'], c['point_y'], c['point_z']),
                #          └── Reconstruct Point from 3 columns
                direction=Vector(c['direction_x'], c['direction_y'], c['direction_z']),
                mandatory=c['mandatory']       # Boolean column
            )
            for c in cursor.fetchall()
            # ^ List comprehension: transform each row into Connector
        ]
        
        types.append(Type(
            guid=type_row['guid'],
            name=type_row['name'],
            connectors=connectors
        ))
    
    # Step 6: Clean up
    conn.close()
    #    └── Always close connections when done!
    
    return Kit(guid=kit_row['guid'], name=kit_row['name'], types=types)
    #          └── Return complete Kit object with all Types and Connectors

# Key Insight: SQL query, Python transform
# 1. SQL efficiently fetches data from disk
# 2. Python transforms rows into rich objects
# 3. Result: Kit object with all relationships loaded
```

**SQL operations in semio**:

```
# ============================================================
# EXAMPLE 3: SQL OPERATIONS REFERENCE
# ============================================================
# Purpose: Map SQL commands to semio use cases
# Relates to: CRUD operations (Create, Read, Update, Delete)
```

| Operation       | SQL Statement            | semio Use Case              |
|-----------------|--------------------------|------------------------------|
| Create table    | `CREATE TABLE`           | Schema initialization        |
|                 |                          | *Run once when creating kit* |
| Insert row      | `INSERT INTO`            | Add new type/design/piece    |
|                 |                          | *User creates a new Type*    |
| Update row      | `UPDATE SET WHERE`       | Modify connector position    |
|                 |                          | *User drags connector*       |
| Delete row      | `DELETE FROM WHERE`      | Remove piece from design     |
|                 |                          | *User deletes piece*         |
| Query single    | `SELECT WHERE guid =`    | Load specific type           |
|                 |                          | *Open type in editor*        |
| Query join      | `SELECT JOIN ON`         | Get pieces with type info    |
|                 |                          | *Show pieces with type names*|
| Aggregate       | `SELECT COUNT GROUP BY`  | Count pieces per type        |
|                 |                          | *Statistics panel*           |

```
# Key Insight: SQL is DECLARATIVE
# You say WHAT you want, not HOW to get it
# "Give me all pieces where type = Wall" (SQL figures out how)
```

**Complex semio queries**:

```sql
-- ============================================================
-- EXAMPLE 4: ADVANCED SQL QUERIES FOR ANALYSIS
-- ============================================================
-- Purpose: Show powerful queries for analyzing kit data
--          These answer complex questions about designs
-- Relates to: SQL for data analysis and validation

-- Find all "orphan" pieces (not connected to anything)
-- Why? These might be errors - pieces floating in space
SELECT p.id, p.name
FROM piece p
-- LEFT JOIN includes pieces even if they have NO connections
LEFT JOIN connection c1 ON p.guid = c1.connected_piece_guid
LEFT JOIN connection c2 ON p.guid = c2.connecting_piece_guid
-- If both joins found nothing, the piece is orphaned
WHERE c1.guid IS NULL AND c2.guid IS NULL
  AND p.design_guid = 'xyz789';
-- Result: List of pieces that aren't connected to anything

-- Count types used in each design
-- Why? Shows which designs use the most variety
SELECT d.name, COUNT(DISTINCT p.type_guid) as type_count
--             │     └── DISTINCT = don't count same type twice
--             └── COUNT = count how many
FROM design d
JOIN piece p ON p.design_guid = d.guid
GROUP BY d.guid
--       └── Group results by design (one row per design)
ORDER BY type_count DESC;
--       └── Sort by type count, highest first
-- Result: Design leaderboard by type variety

-- Find types with mandatory connectors that are never connected
-- Why? Validation - mandatory means "must be connected"
SELECT t.name, c.id as connector_id
FROM type t
JOIN connector c ON c.type_guid = t.guid
WHERE c.mandatory = TRUE
--    └── Only connectors marked as required
  AND NOT EXISTS (
    -- Subquery: check if this connector is ever used
    SELECT 1 FROM connection conn
    WHERE (conn.connected_connector_id = c.id 
           AND conn.connected_piece_guid IN 
               (SELECT guid FROM piece WHERE type_guid = t.guid))
       OR (conn.connecting_connector_id = c.id 
           AND conn.connecting_piece_guid IN 
               (SELECT guid FROM piece WHERE type_guid = t.guid))
  );
-- Result: List of mandatory connectors that are NEVER used
--         This is likely a design error!
```

**Why SQL for semio**

SQL enables:

- Complex queries (joins across tables) in one statement
- Atomic transactions (save entire kit or nothing)
- Indexing for fast lookups by guid
- Cross-platform compatibility (SQLite everywhere)

**What it enables**

- Efficient data access (indexed queries)
- Complex relationships (piece → type → connector)
- Aggregation (count, sum, average)
- Data integrity (foreign key constraints)
- Portable across languages (Python, C#, Go all use SQL)

**What it limits**

- Schema changes require migrations
- Object-relational mapping overhead
- Complex hierarchical data (tree structures) awkward
- Learning curve for advanced SQL

---

#### 9.3 NoSQL: Beyond Tables

**Plain explanation**

NoSQL databases store data differently than traditional tables. Some store documents (JSON), some store key-value pairs, some store graphs. They trade some of SQL's features for flexibility, speed, or scale.

semio uses **IndexedDB** (a browser NoSQL database) for Y.js document persistence in Sketchpad.

**Technical explanation**

**semio's NoSQL usage**:

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    semio NoSQL USAGE                                     │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  Browser (IndexedDB):                                                   │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  IndexedDB Database: "semio-sketchpad"                          │    │
│  │  ├── Object Store: "y-docs"                                     │    │
│  │  │   ├── Key: "kit:abc123" → Y.Doc (binary)                    │    │
│  │  │   ├── Key: "kit:def456" → Y.Doc (binary)                    │    │
│  │  │   └── ...                                                    │    │
│  │  └── Object Store: "settings"                                   │    │
│  │      └── Key: "user-preferences" → { theme, language, ... }    │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
│  Y.js (CRDT document store):                                            │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  Y.Doc contains:                                                │    │
│  │  ├── Y.Map("kit")     → Kit metadata                            │    │
│  │  ├── Y.Array("types") → Type[] with nested Y.Maps              │    │
│  │  ├── Y.Array("designs") → Design[] with nested structures       │    │
│  │  └── Y.Map("meta")    → Collaboration metadata                  │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

**IndexedDB in Sketchpad**:

```typescript
// ============================================================
// EXAMPLE 1: NOSQL DOCUMENT PERSISTENCE IN THE BROWSER
// ============================================================
// Purpose: Store collaborative documents in browser's IndexedDB
// Relates to: NoSQL key-value storage pattern - no tables, just keys and values
// IndexedDB is like a simple filing cabinet where each document has a label (key)
// and contains complex nested data (value). No rigid columns like SQL.
//
// js/semio/sketchpad/Sketchpad.tsx

import { IndexeddbPersistence } from 'y-indexeddb';  // Library that bridges Y.js ↔ IndexedDB
                                                     // Y.js is a CRDT library for real-time collaboration
                                                     // IndexedDB is the browser's built-in NoSQL database

function createKitStore(kitGuid: Guid): KitStore {   // Function to create a persistent kit store
                                                     // kitGuid is the unique identifier for this kit
  
  const yDoc = new Y.Doc();                          // Create a new Y.js document (CRDT document)
                                                     // Y.Doc is like a JSON object that can be edited
                                                     // by multiple users simultaneously without conflicts
  
  // ------------------------------------------------------------
  // KEY-VALUE STORAGE: The heart of NoSQL
  // ------------------------------------------------------------
  // Unlike SQL with tables/rows/columns, IndexedDB stores:
  //   Key: "kit:abc123"  →  Value: <entire Y.Doc binary data>
  // This is like storing a whole filing folder under one label
  
  const persistence = new IndexeddbPersistence(      // Create persistence layer
    `kit:${kitGuid}`,                                // KEY: "kit:abc123" - unique identifier in database
                                                     // Template string builds key like "kit:abc123-def456-..."
    yDoc                                             // VALUE: The entire Y.js document to persist
                                                     // This contains all types, designs, pieces, connections
  );
  
  // ------------------------------------------------------------
  // ASYNC NATURE OF NoSQL: Callback-based, not blocking
  // ------------------------------------------------------------
  // SQL: SELECT * FROM table  (blocks until done)
  // NoSQL: "Tell me when ready" → callback fires later
  
  persistence.on('synced', () => {                   // Event listener: fires when data loads from disk
                                                     // 'synced' means IndexedDB data merged into yDoc
    console.log('[DEBUG] Kit loaded from IndexedDB'); // Confirm data is available
                                                     // User can now see their types and designs
  });
  
  return new KitStore(yDoc, persistence);            // Return the store with both document and persistence
                                                     // KitStore wraps yDoc for convenient access methods
}
```

**NoSQL types comparison**:

| Type        | Structure       | semio Use              | Example           |
|-------------|-----------------|------------------------|-------------------|
| Document    | JSON objects    | Y.js documents         | Y.Map, Y.Array    |
| Key-Value   | Key → Value     | IndexedDB persistence  | kit:guid → Y.Doc  |
| Graph       | Nodes + Edges   | (not used)             | Neo4j             |
| Column      | Column families | (not used)             | Cassandra         |

**When semio uses NoSQL vs SQL**:

| Scenario                      | Storage        | Why                           |
|-------------------------------|----------------|-------------------------------|
| Browser kit editing           | IndexedDB      | No SQLite in browser          |
| Real-time collaboration       | Y.js + CRDT    | Conflict-free concurrent edits|
| Static kit file               | SQLite         | Portable, efficient queries   |
| User preferences              | IndexedDB      | Simple key-value is enough    |

**Why NoSQL for browser**

SQLite doesn't run natively in browsers. IndexedDB provides:

- Asynchronous access (non-blocking)
- Large storage capacity (50MB+)
- Indexed queries (by key)
- Transaction support

**What it enables**

- Flexible schema (Y.js structures)
- Offline persistence (kit survives browser refresh)
- Fast key-value access
- Large data storage in browser

**What it limits**

- No complex queries (can't JOIN)
- Browser-only (not portable to desktop .zip)
- Different API than SQL (async, callback-based)

---

#### 9.4 CRUD: Create, Read, Update, Delete

**Plain explanation**

CRUD is the four basic operations you can do with any stored data: **C**reate (add new data), **R**ead (get existing data), **U**pdate (change data), **D**elete (remove data). Almost every interaction with a database boils down to one of these four operations.

semio kits support CRUD for all entities: types, designs, pieces, connections, connectors, files, and more.

**Technical explanation**

**semio CRUD operations**:

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    semio CRUD OPERATIONS                                 │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  CREATE:                                                                │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  TypeScript: kit.types.push(newType)                            │    │
│  │  SQL:        INSERT INTO type (...) VALUES (...)                │    │
│  │  Y.js:       yTypes.push([yMapFromType(newType)])               │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
│  READ:                                                                  │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  TypeScript: kit.types.find(t => t.guid === guid)               │    │
│  │  SQL:        SELECT * FROM type WHERE guid = ?                  │    │
│  │  Y.js:       yTypes.toArray().find(...)                         │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
│  UPDATE:                                                                │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  TypeScript: type.name = "New Name"                             │    │
│  │  SQL:        UPDATE type SET name = ? WHERE guid = ?            │    │
│  │  Y.js:       yType.set("name", "New Name")                      │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
│  DELETE:                                                                │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  TypeScript: kit.types = kit.types.filter(t => t.guid !== guid) │    │
│  │  SQL:        DELETE FROM type WHERE guid = ?                    │    │
│  │  Y.js:       yTypes.delete(index)                               │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

**semio CRUD in TypeScript** (`js/semio/semio.ts`):

```typescript
// ============================================================
// EXAMPLE 1: CRUD OPERATIONS IN PURE TYPESCRIPT
// ============================================================
// Purpose: Show the four fundamental data operations using plain JavaScript arrays
// Relates to: CRUD is a universal pattern - Create, Read, Update, Delete
// These operations work the same whether you use arrays, databases, or APIs
// Understanding CRUD helps you work with ANY data system
//
// js/semio/semio.ts

// ------------------------------------------------------------
// CREATE: Add new data to the collection
// ------------------------------------------------------------
// Like adding a new card to a recipe box
// The kit gets a new type added to its types array
// We use spread operator (...) to create NEW objects (immutability)

function createType(kit: Kit, typeData: Partial<Type>): Kit {   // Function to add a type to a kit
                                                                 // Partial<Type> means some fields optional
  
  const newType: Type = {                          // Build the new Type object
    guid: crypto.randomUUID(),                     // Generate unique ID for this type
                                                   // Every type needs a guid for identification
    name: typeData.name ?? "New Type",             // Use provided name or default "New Type"
                                                   // ?? is nullish coalescing - use default if null/undefined
    connectors: typeData.connectors ?? [],         // Use provided connectors or empty array
    models: typeData.models ?? [],                 // Use provided models or empty array
    ...typeData                                    // Spread remaining provided properties
                                                   // This adds any other fields from typeData
  };
  
  return { ...kit, types: [...kit.types, newType] }; // Return NEW kit with type added
                                                      // ...kit copies all existing kit properties
                                                      // types: [...kit.types, newType] adds new type to array
                                                      // Immutable pattern: never modify original kit
}

// ------------------------------------------------------------
// READ: Find and retrieve existing data
// ------------------------------------------------------------
// Like looking up a specific card in the recipe box
// We search by GUID because names can change or duplicate

function readType(kit: Kit, guid: Guid): Type | undefined { // Function to find a type by guid
                                                             // Returns Type if found, undefined if not
  
  return kit.types.find(t => t.guid === guid);     // Search the types array for matching guid
                                                   // .find() returns first match or undefined
                                                   // t => t.guid === guid is the search condition
                                                   // Arrow function tests each type in the array
}

// ------------------------------------------------------------
// UPDATE: Modify existing data (using diff system)
// ------------------------------------------------------------
// Like editing a recipe card with correction tape
// semio uses "diffs" to track WHAT changed, not just the new value
// Diffs enable undo/redo and synchronization

function updateType(kit: Kit, guid: Guid, diff: TypeDiff): Kit { // Function to update a type
                                                                  // diff contains the changes to apply
  return {
    ...kit,                                        // Copy all existing kit properties
    types: kit.types.map(t =>                      // Map over all types in the array
                                                   // .map() transforms each element
      t.guid === guid ? applyTypeDiff(t, diff) : t // If this is the type to update...
                                                   // ...apply the diff to get new version
                                                   // Otherwise keep the type unchanged
                                                   // Ternary: condition ? if_true : if_false
    )
  };
}

// ------------------------------------------------------------
// DELETE: Remove data from the collection
// ------------------------------------------------------------
// Like removing a card from the recipe box
// We filter OUT the item to delete, keeping everything else

function deleteType(kit: Kit, guid: Guid): Kit {   // Function to remove a type by guid
  return {
    ...kit,                                        // Copy all existing kit properties
    types: kit.types.filter(t => t.guid !== guid)  // Filter keeps items where condition is TRUE
                                                   // t.guid !== guid means "keep if NOT the target"
                                                   // This effectively removes the matching type
                                                   // Original array unchanged (filter returns new array)
  };
}
```

**semio CRUD in SQL** (Engine):

```sql
-- ============================================================
-- EXAMPLE 2: CRUD OPERATIONS IN SQL (RELATIONAL DATABASE)
-- ============================================================
-- Purpose: Show the four CRUD operations using SQL syntax
-- Relates to: SQL is the universal language for relational databases
-- These same four operations exist in every SQL database: SQLite, PostgreSQL, MySQL, etc.
-- SQL keywords are typically UPPERCASE by convention (but not required)
--
-- py/engine/engine.py (uses these queries internally)

-- ------------------------------------------------------------
-- CREATE: Add new data to a table (INSERT)
-- ------------------------------------------------------------
-- INSERT adds a NEW ROW to an existing table
-- You specify which columns get which values
-- The row becomes permanent in the database

INSERT INTO connector (            -- INSERT INTO tells SQL which table to add to
  guid,                            -- Column 1: unique identifier for this connector
  type_guid,                       -- Column 2: which type this connector belongs to (foreign key)
  id,                              -- Column 3: the connector's id (like "top", "bottom")
  name,                            -- Column 4: human-readable name
  point_x, point_y, point_z        -- Columns 5-7: the 3D position coordinates
)
VALUES (                           -- VALUES provides the actual data to insert
  'new-guid',                      -- Value for guid column
  'type-guid',                     -- Value for type_guid column (links to parent type)
  'top',                           -- Value for id column
  'Top Connector',                 -- Value for name column
  0, 1, 0                          -- Values for point_x, point_y, point_z (position in 3D space)
);

-- ------------------------------------------------------------
-- READ: Retrieve data from a table (SELECT)
-- ------------------------------------------------------------
-- SELECT is the most common SQL operation
-- It READS data without changing anything
-- WHERE clause filters which rows to return

SELECT * FROM connector            -- SELECT * means "get all columns" FROM connector table
WHERE type_guid = 'type-guid';     -- WHERE filters: only rows where type_guid matches
                                   -- This returns ALL connectors that belong to this type
                                   -- Result is a table-like structure with all matching rows

-- ------------------------------------------------------------
-- UPDATE: Modify existing data (UPDATE...SET)
-- ------------------------------------------------------------
-- UPDATE changes existing rows in the table
-- SET specifies which columns to change and their new values
-- WHERE is CRITICAL: without it, ALL rows would be updated!

UPDATE connector                   -- UPDATE specifies which table to modify
SET                                -- SET begins the list of changes
  point_x = 0.5,                   -- Change point_x to 0.5
  point_y = 1.0,                   -- Change point_y to 1.0
  point_z = 0.0                    -- Change point_z to 0.0
WHERE guid = 'connector-guid';     -- WHERE ensures only ONE connector is updated
                                   -- The connector with this exact guid gets modified
                                   -- Without WHERE, ALL connectors would move!

-- ------------------------------------------------------------
-- DELETE: Remove data from a table (DELETE)
-- ------------------------------------------------------------
-- DELETE removes rows from the table permanently
-- WHERE clause specifies which rows to delete
-- WITHOUT WHERE, DELETE removes ALL rows (dangerous!)

DELETE FROM connector              -- DELETE FROM specifies which table to remove from
WHERE guid = 'connector-guid';     -- WHERE specifies exactly which row(s) to delete
                                   -- Only the connector with matching guid is removed
                                   -- The row is gone permanently (unless you have backups)
```

**semio CRUD in Y.js** (Sketchpad):

```typescript
// ============================================================
// EXAMPLE 3: CRUD OPERATIONS IN Y.JS (COLLABORATIVE NOSQL)
// ============================================================
// Purpose: Show CRUD with real-time collaboration support
// Relates to: Y.js is a CRDT (Conflict-free Replicated Data Type) library
// When multiple users edit simultaneously, Y.js merges changes automatically
// This is more complex than SQL because changes must sync between users
//
// js/semio/sketchpad/Sketchpad.tsx

// ------------------------------------------------------------
// CREATE: Add new piece to design (collaborative)
// ------------------------------------------------------------
// When you add a piece in Sketchpad, other users see it instantly
// Y.js broadcasts the change to all connected clients
// yMapFromPiece converts plain object to Y.js-compatible map

function createPiece(designStore: DesignStore, pieceData: Piece): void { // Add piece to store
                                                                          // void = no return value
  
  const yPiece = yMapFromPiece(pieceData);  // Convert Piece object to Y.Map
                                            // Y.Map is Y.js's collaborative map structure
                                            // Like a JavaScript object but syncs across clients
  
  designStore.yPieces.push([yPiece]);       // Add to Y.Array (like array.push)
                                            // yPieces is a Y.Array containing all pieces
                                            // The [yPiece] syntax is required by Y.js
                                            // This triggers sync to other clients automatically
}

// ------------------------------------------------------------
// READ: Get piece from Y.js document
// ------------------------------------------------------------
// Reading from Y.js is similar to reading from arrays
// We convert from Y.js format back to plain JavaScript objects
// toArray() gives us a regular JavaScript array to search

function readPiece(designStore: DesignStore, guid: Guid): Piece | undefined { // Find piece by guid
  
  const yPiece = designStore.yPieces        // Start with the Y.Array of pieces
    .toArray()                              // Convert Y.Array to regular JavaScript array
                                            // Now we can use standard array methods
    .find(yp => yp.get("guid") === guid);   // Find first match by guid
                                            // yp.get("guid") reads from Y.Map (not yp.guid)
                                            // Y.Map uses .get() method, not dot notation
  
  return yPiece ? pieceFromYMap(yPiece) : undefined; // Convert Y.Map back to plain Piece object
                                                      // pieceFromYMap extracts all properties
                                                      // Return undefined if not found
}

// ------------------------------------------------------------
// UPDATE: Modify piece (triggers Y.js sync to all clients)
// ------------------------------------------------------------
// Updating in Y.js requires finding the item first
// transact() groups changes into a single sync operation
// Other users see the update in real-time

function updatePiece(                                // Function to modify an existing piece
  designStore: DesignStore,                          // The store containing all pieces
  guid: Guid,                                        // Which piece to update (by guid)
  updates: Partial<Piece>                            // What to change (partial = some fields)
): void {
  
  const index = designStore.yPieces.toArray().findIndex( // Find the INDEX of the piece
    yp => yp.get("guid") === guid                        // Match by guid
  );                                                     // index = position in array (0, 1, 2, ...)
  
  if (index !== -1) {                                // If found (findIndex returns -1 if not found)
    const yPiece = designStore.yPieces.get(index);   // Get the actual Y.Map at that index
                                                     // .get(index) retrieves item from Y.Array
    
    yPiece.doc?.transact(() => {                     // Wrap changes in a transaction
                                                     // transact() batches changes into one sync
                                                     // ?. is optional chaining (doc might be undefined)
      
      Object.entries(updates).forEach(([key, value]) => { // Loop through each update
                                                          // Object.entries gives [key, value] pairs
                                                          // forEach processes each pair
        yPiece.set(key, value);                           // Set the property on Y.Map
                                                          // This triggers sync to other clients
      });
    });
  }
}

// ------------------------------------------------------------
// DELETE: Remove piece from Y.js document
// ------------------------------------------------------------
// Deletion also syncs - other users see the piece disappear
// We find by index then delete at that position
// Y.Array.delete() removes the item and triggers sync

function deletePiece(designStore: DesignStore, guid: Guid): void { // Remove piece by guid
  
  const index = designStore.yPieces.toArray().findIndex( // Find the index of the piece
    yp => yp.get("guid") === guid                        // Match by guid
  );
  
  if (index !== -1) {                                // If found (not -1)
    designStore.yPieces.delete(index);               // Delete at that index
                                                     // Y.Array.delete(index) removes the item
                                                     // This triggers sync - other users see removal
                                                     // The piece is gone from all connected clients
  }
}
```

**CRUD operations by platform**:

| Operation | Sketchpad (Y.js) | Engine (SQLite) | Grasshopper (C#) |
|-----------|------------------|-----------------|-------------------|
| Create    | `yArray.push()`  | `INSERT INTO`   | `list.Add()`      |
| Read      | `yArray.find()`  | `SELECT WHERE`  | `list.Find()`     |
| Update    | `yMap.set()`     | `UPDATE SET`    | Property assign   |
| Delete    | `yArray.delete()`| `DELETE WHERE`  | `list.Remove()`   |

**Why CRUD is universal**

Almost all data operations are CRUD:

- Create a new type → CREATE
- View a design → READ
- Move a piece → UPDATE
- Remove a connection → DELETE

Understanding CRUD helps reason about any data system.

**What it enables**

- Simple mental model for data operations
- Consistent API design patterns
- Map directly to SQL/HTTP/REST
- Easy to test (four operations per entity)

**What it limits**

- Complex operations don't fit neatly (batch updates, transactions)
- Doesn't capture relationships well
- No concept of undo/redo
- semio uses diffs instead of raw CRUD for change tracking

---

#### 9.5 Schema Migrations: Evolving Data Structures

**Plain explanation**

When your data structure changes—adding a new field, renaming a column, changing types—you need to update existing data to match. Schema migration is the process of transforming old data to fit new structures.

semio handles migrations when the Kit schema evolves (adding new properties to Connector, Type, or Design).

**Technical explanation**

**semio schema evolution**:

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    semio SCHEMA MIGRATION                                │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  Schema v1 (old):                    Schema v2 (new):                   │
│  ┌─────────────────────────┐        ┌─────────────────────────────────┐ │
│  │  interface Connector {  │   →    │  interface Connector {          │ │
│  │    id: string;          │        │    id: string;                  │ │
│  │    point: Point;        │        │    point: Point;                │ │
│  │    direction: Vector;   │        │    direction: Vector;           │ │
│  │  }                      │        │    mandatory?: boolean;   ← NEW │ │
│  └─────────────────────────┘        │    interface?: InterfaceId; NEW │ │
│                                     │  }                              │ │
│                                     └─────────────────────────────────┘ │
│                                                                          │
│  Migration Strategy:                                                     │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  1. New fields are optional (backward compatible)               │    │
│  │  2. Old kits load without new fields                            │    │
│  │  3. Defaults applied when missing                               │    │
│  │  4. Saving adds new fields                                      │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

**semio migration via optional fields** (`js/semio/semio.ts`):

```typescript
// ============================================================
// EXAMPLE 1: BACKWARD-COMPATIBLE SCHEMA EVOLUTION
// ============================================================
// Purpose: How semio handles schema changes without breaking old data
// Relates to: Optional fields allow new features without breaking old kits
// Old kits can still load even if they don't have new fields
// This is "additive migration" - we only add, never remove or rename
//
// js/semio/semio.ts

// ------------------------------------------------------------
// VERSIONED SCHEMA: Each version adds optional fields
// ------------------------------------------------------------
// Zod is a validation library that defines the expected shape of data
// Optional fields (marked with .optional()) can be missing from old data
// This allows old kits to load without the new fields

export const ConnectorSchema = z.object({          // Define the Connector schema using Zod
                                                   // z.object() creates a validator for object shape
  
  id: z.string(),                                  // REQUIRED: connector id (always present)
                                                   // Old kits always had this field
  
  name: z.string().optional(),                     // OPTIONAL: Added in v2 (2023)
                                                   // Old kits from v1 won't have name
                                                   // .optional() means field can be undefined
  
  point: PointSchema,                              // REQUIRED: 3D position (always present)
                                                   // PointSchema validates {x, y, z} structure
  
  direction: VectorSchema,                         // REQUIRED: direction vector (always present)
                                                   // VectorSchema validates {x, y, z} structure
  
  t: z.number().optional(),                        // OPTIONAL: Added in v3 (2024)
                                                   // Position on connector ring for diagrams
                                                   // Old kits don't have this, defaults to undefined
  
  mandatory: z.boolean().optional(),               // OPTIONAL: Added in v4 (2024)
                                                   // Whether connector must be connected
                                                   // Undefined means false (not mandatory)
  
  interface: GuidSchema.optional(),                // OPTIONAL: Added in v5 (2025)
                                                   // Links to Interface for compatibility rules
                                                   // Old connectors work with any other connector
  
  props: z.array(PropSchema).optional(),           // OPTIONAL: Added in v6 (2025)
                                                   // Measurable properties on connector
                                                   // Old connectors have no props
  
  description: z.string().optional(),              // OPTIONAL: Always been optional
                                                   // Human-readable description
  
  attributes: z.array(AttributeSchema).optional()  // OPTIONAL: Always been optional
                                                   // Key-value metadata pairs
});

// ------------------------------------------------------------
// LOADING OLD DATA: Missing fields become undefined
// ------------------------------------------------------------
// When loading a kit from an old version, new fields are simply missing
// Zod's parse() accepts the data because new fields are optional
// Code must handle undefined values with defaults or null checks

const oldKitJson = {                               // Example: Kit from version 1 (2022)
  types: [{                                        // Contains one type
    connectors: [{                                 // With one connector
      id: "top",                                   // Has required id field
      point: { x: 0, y: 0, z: 1 },                 // Has required point field
      direction: { x: 0, y: 0, z: 1 }              // Has required direction field
      // ↑ Notice: No name, mandatory, interface, props, t fields
      // These were added in later versions
    }]
  }]
};

// ------------------------------------------------------------
// PARSING: Zod validates and fills in structure
// ------------------------------------------------------------

const kit = KitSchema.parse(oldKitJson);           // Parse validates against schema
                                                   // Returns typed Kit object if valid
                                                   // Throws error if required fields missing

// After parsing, optional fields are undefined:
// kit.types[0].connectors[0].name === undefined      (v2 field missing)
// kit.types[0].connectors[0].mandatory === undefined (v4 field missing)
// kit.types[0].connectors[0].interface === undefined (v5 field missing)
// kit.types[0].connectors[0].props === undefined     (v6 field missing)

// Code must handle undefined with defaults:
const isMandatory = kit.types[0].connectors[0].mandatory ?? false; // Use false if undefined
const connectorName = kit.types[0].connectors[0].name ?? kit.types[0].connectors[0].id; // Fallback to id
```
```

**SQL schema migrations**:

```sql
-- ============================================================
-- EXAMPLE 2: SQL DATABASE SCHEMA MIGRATION
-- ============================================================
-- Purpose: How to add new columns to existing database tables
-- Relates to: ALTER TABLE is SQL's migration command
-- Unlike TypeScript where optional fields "just work", SQL requires explicit changes
-- Each column must be added to the table structure before it can be used
--
-- sql/sqlite/migrations/v4_to_v5.sql

-- ------------------------------------------------------------
-- STEP 1: Add new column with default value
-- ------------------------------------------------------------
-- ALTER TABLE modifies an existing table's structure
-- ADD COLUMN creates a new column in the table
-- DEFAULT FALSE means existing rows get FALSE for this column
-- Without DEFAULT, existing rows would have NULL

ALTER TABLE connector                        -- Modify the connector table
ADD COLUMN mandatory BOOLEAN DEFAULT FALSE;  -- Add mandatory column
                                             -- BOOLEAN stores true/false
                                             -- DEFAULT FALSE = existing connectors are not mandatory
                                             -- New connectors will also default to FALSE unless specified

-- ------------------------------------------------------------
-- STEP 2: Add foreign key column
-- ------------------------------------------------------------
-- Foreign keys link one table to another
-- REFERENCES creates a constraint that validates the link
-- interface(guid) means this column references the guid column in interface table

ALTER TABLE connector                        -- Still modifying connector table
ADD COLUMN interface_guid TEXT               -- Add interface_guid column
REFERENCES interface(guid);                  -- REFERENCES creates the foreign key constraint
                                             -- TEXT type stores the GUID as a string
                                             -- Linking to interface table for compatibility rules
                                             -- NULL is allowed (no interface = compatible with all)

-- ------------------------------------------------------------
-- STEP 3: Record the migration (version tracking)
-- ------------------------------------------------------------
-- Track which schema version the database is at
-- This lets code know what columns/tables exist
-- Future migrations can check this before running

UPDATE meta                                  -- Update the meta table (stores metadata)
SET value = '5'                              -- Set value to version 5
WHERE key = 'schema_version';                -- Only update the schema_version row
                                             -- Now code knows this database has v5 schema
                                             -- Can use mandatory and interface_guid columns
```

**C# migration handling** (`net/Semio/Semio.cs`):

```csharp
// ============================================================
// EXAMPLE 3: C# NULLABLE PROPERTIES FOR MIGRATION
// ============================================================
// Purpose: How C# handles optional/new properties from JSON
// Relates to: Nullable types (?) allow properties to be missing
// When old JSON doesn't have a property, C# sets it to null
// We then apply defaults after deserialization
//
// net/Semio/Semio.cs

public class Connector                       // Class definition for Connector
{                                            // C# uses classes instead of TypeScript interfaces
    
    // --------------------------------------------------------
    // REQUIRED PROPERTIES: Always present in all versions
    // --------------------------------------------------------
    
    public string Id { get; set; }           // Property with getter and setter
                                             // string type (not nullable) = always required
                                             // { get; set; } is auto-property syntax
    
    public Point Point { get; set; }         // 3D position of the connector
                                             // Point is another class with X, Y, Z
                                             // Required = JSON must have this field
    
    public Vector Direction { get; set; }    // Direction the connector faces
                                             // Vector is another class with X, Y, Z
                                             // Required for connector orientation
    
    // --------------------------------------------------------
    // OPTIONAL PROPERTIES: May be missing in old JSON
    // --------------------------------------------------------
    // The ? after the type makes it nullable
    // Nullable means the value can be null (missing/unknown)
    // We assign defaults using = which applies if not in JSON
    
    public bool? Mandatory { get; set; } = false;  // Nullable bool with default
                                                    // bool? can be: true, false, or null
                                                    // = false sets default for new instances
                                                    // Old JSON without this → null after parse
    
    public Guid? Interface { get; set; }     // Nullable GUID (no default = null)
                                             // Guid? can hold a GUID or null
                                             // null means "no interface specified"
                                             // Old connectors had no interface concept
    
    // --------------------------------------------------------
    // POST-DESERIALIZATION: Apply defaults to nulls
    // --------------------------------------------------------
    // After JSON is parsed, some values might be null
    // ApplyDefaults() replaces nulls with sensible defaults
    // Called after loading old kit files
    
    public void ApplyDefaults()              // Method to fill in missing values
    {
        Mandatory ??= false;                 // ??= is null-coalescing assignment
                                             // If Mandatory is null, set it to false
                                             // If already true/false, leave it alone
                                             // Old connectors become non-mandatory by default
    }
}
```

**semio migration strategies**:

| Strategy            | semio Usage                           | When Used              |
|---------------------|---------------------------------------|------------------------|
| Optional fields     | New Zod properties with `.optional()` | Adding new properties  |
| Default values      | Provide defaults in schema            | New required fields    |
| Transform on load   | Parse and normalize old data          | Renaming properties    |
| Version field       | `schemaVersion` in kit metadata       | Breaking changes       |

**Cross-language schema sync**:

```bash
# Schema changes require sync across all languages
# 1. Update TypeScript (js/semio/semio.ts)
# 2. Run schema generation
npx tsx py/engine/generate-schemas.ts

# Generated files:
# - jsonschema/kit.json        (JSON Schema)
# - sql/sqlite/schema.sql      (SQLite DDL)
# - graphql/schema.graphql     (GraphQL SDL)
```

**Why migrations matter for semio**

Users have existing kits with old schemas. Without migration:

- Old kits fail to load
- Users lose their work
- Breaking changes are impossible

With migration:

- Old kits load successfully
- Missing fields get defaults
- Schema evolves safely

**What it enables**

- Backward compatibility with old kits
- Schema evolution without data loss
- Gradual feature rollout
- Cross-platform compatibility

**What it limits**

- Optional fields complicate logic
- Can't easily remove fields
- Migration code grows over time
- Testing all schema versions is hard

---

#### 9.6 Caching: Speed Through Remembering

**Plain explanation**

Caching stores frequently-accessed data in fast storage to avoid recalculating or refetching it. It's like keeping a frequently-used book on your desk instead of walking to the library each time.

semio uses caching at multiple levels: Y.js caches snapshots, the Sketchpad caches computed piece metadata, and Nx caches build outputs.

**Technical explanation**

**semio caching layers**:

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    semio CACHING LAYERS                                  │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  Application Cache (Sketchpad):                                         │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  Piece Metadata Cache:                                          │    │
│  │    Map<PieceGuid, PieceMetadata>                                │    │
│  │    - Computed planes (expensive calculation)                    │    │
│  │    - Connection hierarchy                                       │    │
│  │    - Invalidated when pieces/connections change                 │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
│  Store Snapshot Cache:                                                  │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  KitStore._cachedSnapshot: Kit | null                           │    │
│  │    - Full kit object                                            │    │
│  │    - Invalidated on any Y.js change                             │    │
│  │    - Hash-based cache key                                       │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
│  Build Cache (Nx):                                                      │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  node_modules/.cache/nx/                                        │    │
│  │    - Build outputs per package                                  │    │
│  │    - Hash based on inputs                                       │    │
│  │    - Shared across developers (remote cache)                    │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

**Snapshot caching in KitStore** (`js/semio/sketchpad/Sketchpad.tsx`):

```typescript
// ============================================================
// EXAMPLE 1: SNAPSHOT CACHING WITH HASH-BASED INVALIDATION
// ============================================================
// Purpose: Avoid rebuilding the same Kit object repeatedly
// Relates to: Caching trades memory for speed - store result to skip work
// The hash is a fingerprint of the data - same hash means same data
// If data hasn't changed (same hash), return cached version
//
// js/semio/sketchpad/Sketchpad.tsx

export class Store<TState> {                           // Generic Store class
                                                       // TState is the type of data it stores (Kit, Design, etc.)
  
  protected _cachedSnapshot: TState | null = null;     // Cached result of buildSnapshot()
                                                       // null means cache is empty/invalid
                                                       // protected = accessible in subclasses
  
  protected _cachedHash: string | null = null;         // Hash of the cached data
                                                       // Used to detect if data changed
                                                       // If new hash matches, cache is still valid

  // ------------------------------------------------------------
  // CACHE HIT/MISS: Check before expensive computation
  // ------------------------------------------------------------
  
  snapshot(): TState {                                 // Get the current state snapshot
    
    const currentHash = this.hash(this.buildSnapshot()); // Compute hash of current data
                                                         // this.hash() generates fingerprint
                                                         // this.buildSnapshot() builds the object
    
    // CACHE HIT: Hash matches, return cached version
    if (this._cachedSnapshot && this._cachedHash === currentHash) { // If cache exists AND hash matches
      return this._cachedSnapshot;                     // Return cached snapshot (fast path!)
                                                       // Skips expensive buildSnapshot() call
                                                       // This is the performance win
    }
    
    // CACHE MISS: Data changed, rebuild and cache
    this._cachedSnapshot = this.buildSnapshot();       // Build fresh snapshot (expensive)
                                                       // This traverses Y.js data structures
    this._cachedHash = currentHash;                    // Store the hash for future comparisons
    return this._cachedSnapshot;                       // Return the fresh snapshot
  }

  // ------------------------------------------------------------
  // CACHE INVALIDATION: Force rebuild on next access
  // ------------------------------------------------------------
  
  invalidateCache(): void {                            // Clear the cache
    this._cachedSnapshot = null;                       // Remove cached snapshot
    this._cachedHash = null;                           // Remove cached hash
                                                       // Next snapshot() call will rebuild
                                                       // Called when Y.js data changes
  }
}
```

**DerivedStore for computed values** (`js/semio/sketchpad/Sketchpad.tsx`):

```typescript
// ============================================================
// EXAMPLE 2: DEPENDENCY-BASED CACHING FOR COMPUTED VALUES
// ============================================================
// Purpose: Cache expensive computations that depend on other data
// Relates to: Derived values are computed FROM source data
// When source changes, derived value must recompute
// When source is unchanged, return cached derived value
//
// js/semio/sketchpad/Sketchpad.tsx

// ------------------------------------------------------------
// CREATING A DERIVED CACHE: Define dependencies and compute function
// ------------------------------------------------------------
// derivedStore manages cached computed values
// getOrCreate either returns existing cache or creates new one
// Dependencies define WHEN to invalidate the cache

const piecesMetadataNode = derivedStore.getOrCreate(   // Create or get cached computation
  
  "piecesMetadata",                                    // CACHE KEY: unique identifier for this cache
                                                       // Used to look up the cached value later
  
  [{ store: designStore, path: [yPathMapKey("pieces")] }], // DEPENDENCIES: what this cache depends on
                                                       // store: which Y.js store to watch
                                                       // path: which part of the store to watch
                                                       // yPathMapKey("pieces") = watch the pieces array
                                                       // When pieces change, cache is invalidated
  
  () => computePiecesMetadata(designStore.snapshot())  // COMPUTE FUNCTION: how to build the cached value
                                                       // Arrow function called only when cache invalid
                                                       // computePiecesMetadata is expensive calculation
                                                       // Computes planes, hierarchy, connections for all pieces
                                                       // Only runs when pieces actually change
);

// ------------------------------------------------------------
// USING THE CACHED VALUE: Get result from cache
// ------------------------------------------------------------
// snapshot() returns the cached value
// Only recomputes if dependencies changed
// Multiple components can share the same cache

const metadata = piecesMetadataNode.snapshot();        // Get the cached piece metadata
                                                       // If pieces unchanged, instant return
                                                       // If pieces changed, recomputes first
                                                       // Shared across all components needing piece metadata
                                                       // Huge performance win for diagram/scene rendering
```

**Nx build caching** (`nx.json`):

```json
// ============================================================
// EXAMPLE 3: BUILD SYSTEM CACHING (NX MONOREPO)
// ============================================================
// Purpose: Cache build outputs to avoid rebuilding unchanged packages
// Relates to: Build caching is like caching at the project level
// If source files haven't changed, use cached build output
// This makes "npm run build" fast when only one package changed
//
// nx.json (Nx monorepo configuration)

{
  "tasksRunnerOptions": {                    // Configure how Nx runs tasks
    "default": {                             // Default runner configuration
      "runner": "nx/tasks-runners/default",  // Use Nx's built-in task runner
                                             // The runner executes build, test, lint commands
      "options": {
        "cacheableOperations": [             // Which operations can be cached
          "build",                           // Cache build outputs (compiled JS, bundles)
          "lint",                            // Cache lint results (ESLint checks)
          "test"                             // Cache test results (Vitest output)
        ],                                   // Only cache operations that are deterministic
                                             // Same inputs should always produce same outputs
        "parallel": 3                        // Run up to 3 tasks simultaneously
                                             // Speeds up when multiple packages need work
      }
    }
  },
  "namedInputs": {                           // Define sets of input files
                                             // Used to determine cache key (hash)
    "default": [                             // Default input set
      "{projectRoot}/**/*"                   // All files in the project
                                             // {projectRoot} is like "js/semio" for @semio/js
    ],
    "production": [                          // Production input set (for builds)
      "default",                             // Include all default inputs
      "!{projectRoot}/**/*.test.ts"          // EXCLUDE test files (! means exclude)
                                             // Test files don't affect build output
                                             // Changing tests shouldn't invalidate build cache
    ]
  }
}
// When you run "npx nx build @semio/js":
// 1. Nx hashes all input files
// 2. Checks if cached output exists for that hash
// 3. If cache hit: copies cached files (fast!)
// 4. If cache miss: runs actual build, stores in cache
```

**Cache invalidation strategies**:

| Strategy         | semio Usage                        | Trigger              |
|------------------|------------------------------------|----------------------|
| Time-based       | (Not used)                         | Expires after N sec  |
| Event-based      | Y.js observer invalidates cache   | On data change       |
| Hash-based       | Nx cache keys                      | Input files change   |
| Manual           | `invalidateCache()` call          | Explicit invalidation|

**Why caching for semio**

Without caching:

- Every piece position recalculated on render (slow)
- Kit snapshot rebuilt on every access (CPU waste)
- Build times repeated for unchanged packages (slow CI)

With caching:

- Piece metadata computed once per change
- Snapshots reused until invalidated
- Builds skip unchanged packages

**What it enables**

- Fast UI updates (60 fps rendering)
- Efficient memory usage
- Quick build times
- Responsive interactions

**What it limits**

- Stale data if invalidation fails
- Memory overhead for cached values
- Cache debugging is tricky
- Complexity in cache key management

---

#### 9.7 Data Validation: Ensuring Correctness

**Plain explanation**

Validation checks that data meets required rules before accepting it. Invalid data—duplicate GUIDs, orphan references, out-of-range values—causes bugs and crashes. Validation catches problems early, at the point of entry.

semio has a comprehensive validation system that checks kits for structural correctness and generates diff-based fixes.

**Technical explanation**

**semio validation architecture**:

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    semio VALIDATION SYSTEM                               │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  Validation Flow:                                                       │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  Kit → ValidationContext → Constraints → Problems → Fixes       │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
│  Constraints:                                                           │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  guid-unique        → All GUIDs must be unique                  │    │
│  │  type-name-unique   → Type names unique among siblings          │    │
│  │  design-name-unique → Design names unique among siblings        │    │
│  │  piece-name-unique  → Piece names unique within design          │    │
│  │  connector-name-unique → Connector names unique within type     │    │
│  │  ...                                                            │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
│  Problem Structure:                                                     │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  {                                                              │    │
│  │    constraintId: "type-name-unique",                            │    │
│  │    severity: "error",                                           │    │
│  │    message: "Duplicate type name 'Wall' among siblings",        │    │
│  │    location: { entityKind: "Type", entityGuid: "abc123" },      │    │
│  │    fixes: [{ title: "Rename to 'Wall 2'", diff: {...} }]       │    │
│  │  }                                                              │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

**semio validation** (`js/semio/semio.ts`):

```typescript
// ============================================================
// EXAMPLE 1: DOMAIN VALIDATION WITH FIX GENERATION
// ============================================================
// Purpose: Check kit data for errors and generate automatic fixes
// Relates to: Validation catches problems before they cause bugs
// Each constraint checks one rule and produces fixable problems
// Fixes are KitDiffs that can be applied to correct the problem
//
// js/semio/semio.ts

// ------------------------------------------------------------
// TYPES: Define the shape of validation results
// ------------------------------------------------------------

type Severity = "error" | "warning";         // How serious is the problem?
                                             // "error" = must fix before use
                                             // "warning" = should fix but works

type SemioEntityKind =                       // What kind of entity has the problem?
  "Kit" | "Type" | "Design" | "Piece" | "Connector" | /* ... */; // All entity types

interface SemioDomainLocation {              // WHERE is the problem in the kit?
  entityKind: SemioEntityKind;               // What type of entity (Type, Design, etc.)
  entityGuid?: Guid;                         // Which specific entity (by GUID)
  field?: string;                            // Which field on that entity (optional)
}

interface Fix {                              // HOW to fix the problem
  title: string;                             // Human-readable description of fix
  diff: KitDiff;                             // The actual change to apply
                                             // KitDiff is semio's change format
                                             // Applying this diff fixes the problem
}

interface Problem {                          // A validation error or warning
  constraintId: string;                      // Which constraint was violated
                                             // Like "type-name-unique" or "guid-unique"
  severity: Severity;                        // Is it error or warning?
  message: string;                           // Human-readable explanation
  location: SemioDomainLocation;             // Where is the problem?
  fixes: Fix[];                              // Available automatic fixes
                                             // User can click to apply
}

// ------------------------------------------------------------
// CONSTRAINT PATTERN: Function that finds problems
// ------------------------------------------------------------
// Constraints are pure functions: Kit in → Problems out
// Each constraint checks ONE rule
// Returns empty array if no problems found

type Constraint = (ctx: ValidationContext) => Problem[]; // Function signature

// ------------------------------------------------------------
// EXAMPLE CONSTRAINT: Unique type names
// ------------------------------------------------------------
// Two types can't have the same name within the same parent
// This constraint finds duplicates and generates rename fixes

const typeNameUniqueConstraint: Constraint = (ctx) => {  // Constraint function
  const problems: Problem[] = [];                        // Accumulate problems found
  
  const nameGroups = groupBy(ctx.kit.types, t => t.name); // Group types by name
                                                          // groupBy returns { name: [types...] }
                                                          // If multiple types have same name, array length > 1
  
  for (const [name, types] of Object.entries(nameGroups)) { // Check each name group
    if (types.length > 1) {                              // More than one type with this name?
                                                         // That's a duplicate! Create a problem
      
      // Generate automatic fix: rename the duplicates
      const fix = semioMakeFix(ctx.kit, {                // Create a KitDiff to fix this
        types: {
          updated: types.slice(1).map((t, i) => ({       // For each duplicate (skip first)
            guid: t.guid,                                // Target this specific type
            diff: { name: `${name} ${i + 2}` }           // Rename to "Wall 2", "Wall 3", etc.
          }))
        }
      });
      
      problems.push({                                    // Add problem to results
        constraintId: "type-name-unique",                // Constraint that was violated
        severity: "error",                               // This is an error (must fix)
        message: `Duplicate type name "${name}"`,        // What went wrong
        location: {                                      // Where is the problem?
          entityKind: "Type",                            // It's a Type entity
          entityGuid: types[1].guid                      // Specifically the second duplicate
        },
        fixes: [{                                        // Available fixes
          title: `Rename to "${name} 2"`,                // What the fix does
          diff: fix                                      // The KitDiff to apply
        }]
      });
    }
  }
  return problems;                                       // Return all problems found
};

// ------------------------------------------------------------
// RUNNING VALIDATION: Check all constraints
// ------------------------------------------------------------

function validateSemioKit(kit: Kit): ValidationResult {  // Main validation function
  const ctx = buildValidationContext(kit);               // Build context with indexes
                                                         // Context has maps for fast lookup
  
  const problems = defaultConstraints.flatMap(c => c(ctx)); // Run ALL constraints
                                                            // flatMap flattens arrays
                                                            // Each constraint returns Problem[]
  
  return { problems };                                   // Return validation result
                                                         // problems.length === 0 means kit is valid
}
```

**Zod schema validation**:

```typescript
// ============================================================
// EXAMPLE 2: PARSE-TIME VALIDATION WITH ZOD
// ============================================================
// Purpose: Validate data structure when loading from JSON
// Relates to: Schema validation ensures data matches expected shape
// Zod validates AND provides TypeScript types from the same schema
// Invalid data throws an error instead of causing bugs later
//
// js/semio/semio.ts

// ------------------------------------------------------------
// DEFINING A VALIDATED SCHEMA
// ------------------------------------------------------------
// z.object() defines what properties an object must have
// Each property has a type and optional constraints

const TypeSchema = z.object({                  // Define Type schema
  
  guid: GuidSchema,                            // Must have a valid GUID
                                               // GuidSchema validates UUID format
  
  name: z.string().min(1),                     // Must have non-empty name
                                               // z.string() = must be string
                                               // .min(1) = at least 1 character
                                               // Empty string "" would fail
  
  connectors: z.array(ConnectorSchema).default([]), // Array of connectors
                                                     // .default([]) = use empty array if missing
                                                     // Each element validated by ConnectorSchema
  
  models: z.array(ModelSchema).default([])    // Array of models
                                              // Same pattern: array of validated elements
  
}).refine(                                    // Additional validation beyond structure
  
  t => t.connectors.every(c => c.id),         // Custom validation function
                                              // Check that every connector has an id
                                              // every() returns true if ALL pass the test
  
  "Every connector must have an id"           // Error message if validation fails
                                              // User sees this message on invalid data
);

// ------------------------------------------------------------
// USING THE SCHEMA TO VALIDATE DATA
// ------------------------------------------------------------
// parse() validates AND returns typed data
// Throws ZodError if validation fails

const type = TypeSchema.parse(jsonData);       // Validate jsonData against schema
                                               // If valid: returns typed Type object
                                               // If invalid: throws ZodError with details
                                               // After this line, TypeScript knows type is valid
```

**Cross-platform validation**:

| Platform     | Validation                  | Output                    |
|--------------|-----------------------------|-----------------------------|
| TypeScript   | `validateSemioKit(kit)`     | `Problem[]` with fixes     |
| Python       | `validate_kit(kit)`         | `Problem[]` with fixes     |
| C#           | `SemioValidator.Validate()` | `Problem[]` with fixes     |
| VS Code      | Diagnostics from problems   | Squiggly lines + Quick Fix |

**Why validation for semio**

Invalid kits cause cascading failures:

- Duplicate GUIDs → wrong piece selected
- Orphan references → missing type on load
- Invalid connections → broken placement algorithm

Validation catches these at the boundary, before they propagate.

**What it enables**

- Early error detection
- Automatic fixes via diffs
- Cross-platform consistency
- User-friendly error messages
- Preventive quality control

**What it limits**

- Validation adds processing time
- Complex constraints are hard to express
- False positives frustrate users
- Validation logic duplicated per language

---

#### 9.8 Data Synchronization: Keeping Copies in Sync

**Plain explanation**

When data exists in multiple places—browser, server, other users' browsers—keeping it synchronized is challenging. Changes made by one user need to reach others. Conflicts need resolution. Offline changes need to merge when reconnecting.

semio uses Y.js CRDTs for automatic, conflict-free synchronization in collaborative editing.

**Technical explanation**

**semio sync architecture**:

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    semio DATA SYNCHRONIZATION                            │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  User A's Browser                   User B's Browser                    │
│  ┌─────────────────────┐           ┌─────────────────────┐              │
│  │  Y.Doc (local)      │           │  Y.Doc (local)      │              │
│  │  ├── yTypes         │           │  ├── yTypes         │              │
│  │  ├── yDesigns       │           │  ├── yDesigns       │              │
│  │  └── yMeta          │           │  └── yMeta          │              │
│  └─────────┬───────────┘           └─────────┬───────────┘              │
│            │                                  │                          │
│            │  Y.js Updates (binary)          │                          │
│            │  ┌───────────────────────┐      │                          │
│            └──│   WebSocket Server    │──────┘                          │
│               │   (Yjs-websocket)     │                                 │
│               └───────────────────────┘                                 │
│                                                                          │
│  CRDT Merge (automatic, conflict-free):                                 │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  User A: piece.name = "Wall A"                                  │    │
│  │  User B: piece.position = {x: 100, y: 200}                      │    │
│  │  Result: Both changes merge automatically (different fields)    │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

**Y.js synchronization** (`js/semio/sketchpad/Sketchpad.tsx`):

```typescript
// ============================================================
// EXAMPLE 1: REAL-TIME COLLABORATIVE SYNCHRONIZATION
// ============================================================
// Purpose: Keep multiple users' copies of a kit in sync
// Relates to: CRDTs (Conflict-free Replicated Data Types) merge automatically
// Y.js is a CRDT library that handles sync without manual conflict resolution
// Data syncs in real-time via WebSocket, persists locally via IndexedDB
//
// js/semio/sketchpad/Sketchpad.tsx

import * as Y from 'yjs';                                // Y.js CRDT library
                                                         // Provides Y.Doc, Y.Map, Y.Array
                                                         // Handles automatic conflict resolution

import { WebsocketProvider } from 'y-websocket';         // WebSocket sync provider
                                                         // Connects Y.Doc to WebSocket server
                                                         // Broadcasts changes to other users

import { IndexeddbPersistence } from 'y-indexeddb';      // Local persistence provider
                                                         // Saves Y.Doc to browser's IndexedDB
                                                         // Enables offline access

function createSyncedKitStore(kitGuid: Guid): KitStore { // Create kit store with sync
  
  const yDoc = new Y.Doc();                              // Create the Y.js document
                                                         // This is the root of all synced data
                                                         // Contains yTypes, yDesigns, etc.

  // ------------------------------------------------------------
  // LOCAL PERSISTENCE: Works offline
  // ------------------------------------------------------------
  // IndexedDB stores data in the browser permanently
  // User can close browser, come back, and data is there
  // Also serves as cache while offline
  
  const indexedDb = new IndexeddbPersistence(            // Create local persistence
    `kit:${kitGuid}`,                                    // Database key (unique per kit)
    yDoc                                                 // Document to persist
  );                                                     // Changes automatically saved locally

  // ------------------------------------------------------------
  // REMOTE SYNC: Shares with other users
  // ------------------------------------------------------------
  // WebSocket connection to sync server
  // Changes broadcast to all connected users in real-time
  // ~50ms latency for typical updates
  
  const wsProvider = new WebsocketProvider(              // Create WebSocket provider
    'wss://sync.semio.design',                           // WebSocket server URL
                                                         // wss:// = secure WebSocket
    `kit:${kitGuid}`,                                    // Room name (users editing same kit)
                                                         // Only users in same room see each other's changes
    yDoc                                                 // Document to sync
  );

  // ------------------------------------------------------------
  // CONNECTION STATUS: Track online/offline state
  // ------------------------------------------------------------
  
  wsProvider.on('status', (event) => {                   // Listen for connection changes
    console.log(`[DEBUG] Sync status: ${event.status}`); // Log status (connected/disconnected)
                                                         // UI can show online indicator
  });
  
  return new KitStore(yDoc, { indexedDb, wsProvider });  // Return store with both providers
                                                         // KitStore wraps yDoc for convenience
}
```

**CRDT operations**:

```typescript
// ============================================================
// EXAMPLE 2: AUTOMATIC CONFLICT-FREE CHANGES
// ============================================================
// Purpose: Show how changes sync automatically to all users
// Relates to: CRDTs merge concurrent changes without conflicts
// When User A and User B edit simultaneously, both changes apply
// No manual merge needed - Y.js handles it automatically
//
// js/semio/sketchpad/Sketchpad.tsx

// ------------------------------------------------------------
// TRANSACT: Group related changes together
// ------------------------------------------------------------
// transact() batches multiple changes into one sync operation
// Other users receive all changes atomically (all or nothing)
// Also improves performance by reducing network round-trips

yDoc.transact(() => {                                    // Start a transaction
                                                         // All changes inside are batched
  
  // Add new piece (visible to all connected users instantly)
  const yPiece = new Y.Map();                            // Create new Y.Map for the piece
                                                         // Y.Map is like a JavaScript object
                                                         // but syncs across all clients
  
  yPiece.set("guid", crypto.randomUUID());               // Set the piece's unique ID
                                                         // crypto.randomUUID() generates UUID
                                                         // All users will see this same GUID
  
  yPiece.set("name", "New Piece");                       // Set the piece's name
                                                         // Other users see "New Piece" appear
  
  yPiece.set("typeGuid", selectedTypeGuid);              // Link piece to its type
                                                         // selectedTypeGuid from UI selection
  
  yPieces.push([yPiece]);                                // Add piece to the pieces array
                                                         // yPieces is a Y.Array in the Y.Doc
                                                         // push() triggers sync to all clients
                                                         // ~50ms later, other users see the new piece
});

// ------------------------------------------------------------
// AUTOMATIC MERGE: No conflict resolution needed
// ------------------------------------------------------------
// Y.js handles concurrent edits automatically:
//
// Scenario: User A and User B edit the same piece at the same time
//   User A: piece.name = "Wall A"     (changes name field)
//   User B: piece.position = {x:100}  (changes position field)
//
// Result: BOTH changes apply automatically
//   piece = { name: "Wall A", position: {x:100}, ... }
//
// This is because Y.js tracks changes per-field
// Different fields = no conflict, both apply
// Same field = last-writer-wins (eventual consistency)
```

**Sync scenarios**:

| Scenario              | Y.js Behavior                              |
|-----------------------|--------------------------------------------|
| Same field, same time | Last-writer-wins per field                 |
| Different fields      | Both changes merge automatically           |
| Add + delete same     | Depends on timing (eventual consistency)   |
| Offline edits         | Merge on reconnect                         |

**Offline-first design**:

```typescript
// ============================================================
// EXAMPLE 3: OFFLINE-FIRST ARCHITECTURE
// ============================================================
// Purpose: Changes work even without internet connection
// Relates to: Offline-first means local storage is primary, sync is secondary
// Users can work on airplanes, in tunnels, or with poor connectivity
// When connection returns, changes automatically sync
//
// js/semio/sketchpad/Sketchpad.tsx

function handlePieceMove(pieceGuid: Guid, newPosition: Point): void { // Handle user moving a piece
  
  const yPiece = findYPiece(pieceGuid);                // Find the piece in Y.js document
                                                       // Returns Y.Map for that piece
  
  // ------------------------------------------------------------
  // LOCAL CHANGE: Works immediately, even offline
  // ------------------------------------------------------------
  // The change is applied to the local Y.Doc instantly
  // IndexedDB persistence saves it to disk
  // User sees the piece move immediately (no waiting for server)
  
  yDoc.transact(() => {                                // Wrap in transaction
    yPiece.set("center", newPosition);                 // Update the piece position
                                                       // This works 100% offline
                                                       // No network required at all
  });                                                  // Transaction complete
  
  // ------------------------------------------------------------
  // SYNC (when online): Automatic background operation
  // ------------------------------------------------------------
  // Y.js WebsocketProvider handles sync automatically:
  // - If online: change sent to server in ~50ms
  // - If offline: change queued, sent when reconnected
  // - Other users see the change when it arrives
  //
  // No code needed here - Y.js does it automatically!
  // The WebsocketProvider we configured earlier handles everything
}
```

**Why sync for semio**

Collaborative design requires real-time updates:

- Multiple architects editing same kit
- Changes visible instantly to all
- Offline work that merges later
- Conflict resolution without user intervention

**What it enables**

- Real-time collaboration
- Offline capability
- Automatic conflict resolution
- Eventual consistency guarantee

**What it limits**

- Last-writer-wins loses some changes
- Complex merge logic for arrays
- Sync overhead (bandwidth, CPU)
- Server infrastructure required

---

#### 9.9 Backups: Protecting Against Loss

**Plain explanation**

Backups are copies of data stored separately from the original. If the original is lost, corrupted, or accidentally deleted, backups allow recovery. The 3-2-1 rule: 3 copies, 2 different media, 1 offsite.

semio kits can be exported as portable `.zip` files for backup and sharing.

**Technical explanation**

**semio backup strategies**:

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    semio BACKUP STRATEGIES                               │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  Kit Export (.zip):                                                     │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  metabolism.zip                                                 │    │
│  │  ├── .semio/                                                    │    │
│  │  │   └── kit.db        ← SQLite with all data                  │    │
│  │  ├── models/                                                    │    │
│  │  │   ├── capsule.glb   ← 3D models                             │    │
│  │  │   └── beam.glb                                               │    │
│  │  └── images/                                                    │    │
│  │      └── preview.png   ← Kit preview                           │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
│  Git Version Control:                                                   │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  .git/objects/         ← Complete history                      │    │
│  │    Every commit is a recoverable state                          │    │
│  │    Push to GitHub for remote backup                             │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
│  IndexedDB (browser):                                                   │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  Y.js persistence survives browser refresh                      │    │
│  │  Export to .zip for true backup                                 │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

**Kit export** (`js/semio/semio.ts`):

```typescript
// ============================================================
// EXAMPLE 1: EXPORTING A KIT TO PORTABLE ZIP FILE
// ============================================================
// Purpose: Create a backup/sharable file containing complete kit
// Relates to: Portable formats ensure data survives system changes
// The .zip contains everything needed to restore the kit:
//   - SQLite database with all data
//   - 3D model files (GLB, GLTF, etc.)
//   - Image files (previews, icons)
//
// js/semio/semio.ts

import JSZip from 'jszip';                             // Library for creating ZIP files
                                                       // Works in browser (no server needed)

async function exportKitToZip(                         // Async function to create zip
  kit: Kit,                                            // The kit data to export
  files: Map<FileGuid, Blob>                           // Map of file GUIDs to file content
): Promise<Blob> {                                     // Returns a Blob (binary data)
  
  const zip = new JSZip();                             // Create new empty ZIP archive
  
  // ------------------------------------------------------------
  // ADD KIT DATABASE: The core data
  // ------------------------------------------------------------
  // SQLite database contains all types, designs, connections, etc.
  // This is the structured data that defines the kit
  
  const db = await createKitDatabase(kit);             // Convert kit to SQLite database
                                                       // Creates binary SQLite file in memory
  
  zip.folder('.semio')?.file('kit.db', db);            // Add database to .semio folder
                                                       // .semio is reserved folder for semio data
                                                       // ?.file() handles null folder case
  
  // ------------------------------------------------------------
  // ADD MODEL FILES: The 3D geometry
  // ------------------------------------------------------------
  // Loop through all types and their models
  // Copy each model file into the zip
  
  const modelsFolder = zip.folder('models');           // Create models folder in zip
  
  for (const type of kit.types) {                      // Loop through all types
    for (const model of type.models ?? []) {           // Loop through each type's models
                                                       // ?? [] handles missing models array
      
      const fileBlob = files.get(model.file);          // Get the file content by GUID
                                                       // files Map contains actual binary data
      
      if (fileBlob) {                                  // If file exists in our map
        modelsFolder?.file(model.file, fileBlob);      // Add to models folder in zip
                                                       // Uses file GUID as filename
      }
    }
  }
  
  // ------------------------------------------------------------
  // GENERATE ZIP: Create the final blob
  // ------------------------------------------------------------
  
  return await zip.generateAsync({ type: 'blob' });    // Generate zip as Blob
                                                       // 'blob' = binary large object
                                                       // Can be downloaded or stored
}

// ------------------------------------------------------------
// USAGE: Download zip file to user's computer
// ------------------------------------------------------------

const zipBlob = await exportKitToZip(kit, fileStore);  // Create the zip blob

const url = URL.createObjectURL(zipBlob);              // Create temporary URL for blob
                                                       // Browser can "download" from this URL

const a = document.createElement('a');                 // Create invisible <a> link
a.href = url;                                          // Point to blob URL
a.download = `${kit.name}.zip`;                        // Set download filename
a.click();                                             // Trigger download
                                                       // User sees "Save as" dialog
```

**Git as backup**:

```bash
# Every commit is a backup point
git log --oneline
# a1b2c3d feat: add Quality benchmarks
# d4e5f6g fix: connector validation
# g7h8i9j Initial commit

# Recover any previous state
git checkout d4e5f6g -- js/semio/semio.ts

# Push to remote for offsite backup
git push origin main
```

**Backup best practices for semio**:

| Strategy          | semio Implementation          | Frequency           |
|-------------------|-------------------------------|---------------------|
| Local export      | Download .zip from Sketchpad  | Before major changes|
| Git commits       | Commit after each change      | Continuous          |
| Remote push       | Push to GitHub                | Daily               |
| Cloud sync        | (Future) Liveblocks backup    | Real-time           |

**Why backups for semio**

Kits represent significant design work:

- Hours of type modeling
- Complex connection logic
- Carefully positioned pieces

Losing a kit means losing that work. Backups provide insurance.

**What it enables**

- Recovery from deletion/corruption
- Point-in-time restoration
- Sharing kits with others
- Archival for future reference

**What it limits**

- Storage space for backups
- Manual export discipline
- Large 3D models increase backup size
- Restore testing rarely done

---

#### 9.10 Data Integrity: Ensuring Consistency

**Plain explanation**

Data integrity means data remains accurate, consistent, and valid over time. Foreign keys ensure referenced entities exist. Constraints prevent invalid values. Transactions ensure changes are all-or-nothing.

semio maintains integrity through GUID references, validation constraints, and transactional Y.js updates.

**Technical explanation**

**semio integrity mechanisms**:

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    semio DATA INTEGRITY                                  │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  GUID References:                                                       │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  Piece → Type (typeGuid)                                        │    │
│  │  Connection → Piece (connectedPieceGuid, connectingPieceGuid)   │    │
│  │  Connector → Interface (interfaceGuid)                          │    │
│  │  Model → File (fileGuid)                                        │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
│  Referential Integrity Checks:                                          │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  validateReferences(kit):                                       │    │
│  │    - Every piece.typeGuid exists in kit.types                   │    │
│  │    - Every connection.pieceGuid exists in design.pieces         │    │
│  │    - Every connector.interfaceGuid exists in kit.interfaces     │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
│  Transactional Updates:                                                 │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  yDoc.transact(() => {                                          │    │
│  │    // All changes are atomic                                    │    │
│  │    yPieces.push([newPiece]);                                    │    │
│  │    yConnections.push([newConnection]);                          │    │
│  │    // Either both succeed or neither                            │    │
│  │  });                                                            │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

**Referential integrity validation**:

```typescript
// ============================================================
// EXAMPLE 1: CHECKING THAT ALL REFERENCES ARE VALID
// ============================================================
// Purpose: Find broken links between entities in a kit
// Relates to: Referential integrity means every reference points to something real
// If a piece references a type that doesn't exist, that's broken integrity
// This function finds all such problems
//
// js/semio/semio.ts

function validateReferences(kit: Kit): Problem[] {     // Check all references in kit
                                                       // Returns list of problems found
  
  const problems: Problem[] = [];                      // Accumulate problems here
  
  // ------------------------------------------------------------
  // BUILD LOOKUP SETS: Fast O(1) existence checking
  // ------------------------------------------------------------
  // Convert arrays to Sets for fast "does this exist?" checks
  // Set.has() is O(1) vs Array.find() which is O(n)
  
  const typeGuids = new Set(kit.types.map(t => t.guid)); // Set of all type GUIDs
                                                          // map() extracts guids into array
                                                          // new Set() converts to Set
  
  const interfaceGuids = new Set(                        // Set of all interface GUIDs
    kit.interfaces?.map(i => i.guid) ?? []               // Handle missing interfaces array
  );
  
  // ------------------------------------------------------------
  // CHECK PIECE → TYPE REFERENCES
  // ------------------------------------------------------------
  // Every piece.typeGuid must point to an existing type
  // If the type was deleted, this piece is now broken
  
  for (const design of kit.designs) {                   // Loop through all designs
    for (const piece of design.pieces ?? []) {          // Loop through pieces in design
      
      if (piece.typeGuid && !typeGuids.has(piece.typeGuid)) { // If piece has type AND type doesn't exist
                                                               // typeGuids.has() checks the Set
        problems.push({                                  // Report the problem
          constraintId: "piece-type-reference",          // Constraint ID for categorization
          severity: "error",                             // This is a serious error
          message: `Piece "${piece.name}" references non-existent type`, // What went wrong
          location: {                                    // Where is the problem?
            entityKind: "Piece",                         // It's on a Piece
            entityGuid: piece.guid                       // This specific piece
          },
          fixes: []                                      // No automatic fix (type is gone)
        });
      }
    }
  }
  
  // ------------------------------------------------------------
  // CHECK CONNECTOR → INTERFACE REFERENCES
  // ------------------------------------------------------------
  // Connectors can optionally reference an interface
  // If that interface was deleted, connection becomes broken
  
  for (const type of kit.types) {                       // Loop through all types
    for (const connector of type.connectors ?? []) {    // Loop through connectors
      
      if (connector.interface && !interfaceGuids.has(connector.interface)) { // If has interface ref AND doesn't exist
        problems.push({                                  // Report the problem
          constraintId: "connector-interface-reference", // Constraint ID
          severity: "error",                             // Serious error
          message: `Connector "${connector.id}" references non-existent interface`,
          location: {
            entityKind: "Connector",                     // It's on a Connector
            entityGuid: connector.id                     // Connector id (not guid)
          },
          fixes: []                                      // No automatic fix
        });
      }
    }
  }
  
  return problems;                                      // Return all problems found
}
```
```

**SQL foreign keys**:

```sql
-- ============================================================
-- EXAMPLE 2: SQL FOREIGN KEY CONSTRAINTS
-- ============================================================
-- Purpose: Let the database enforce referential integrity automatically
-- Relates to: Foreign keys are database-level protection
-- If you try to insert a piece with non-existent type, database rejects it
-- Much safer than relying on application code to check
--
-- sql/sqlite/schema.sql

-- Enable foreign key enforcement in SQLite
-- SQLite has foreign keys OFF by default for backwards compatibility
-- We must explicitly turn them ON
PRAGMA foreign_keys = ON;                    -- Enable foreign key constraints
                                             -- Without this, foreign keys are ignored!

-- ------------------------------------------------------------
-- TABLE WITH FOREIGN KEY CONSTRAINTS
-- ------------------------------------------------------------
-- This table definition declares the relationships
-- The database will enforce them automatically

CREATE TABLE piece (
    guid TEXT PRIMARY KEY,                   -- Every piece has unique GUID
                                             -- PRIMARY KEY = unique identifier
    
    design_guid TEXT NOT NULL                -- Which design this piece belongs to
    REFERENCES design(guid)                  -- Must match a guid in design table
    ON DELETE CASCADE,                       -- If design deleted, delete this piece too
                                             -- CASCADE = automatic cleanup
    
    type_guid TEXT                           -- Which type this piece is
    REFERENCES type(guid)                    -- Must match a guid in type table
    ON DELETE SET NULL                       -- If type deleted, set this to NULL
                                             -- SET NULL = orphan the piece (don't delete)
);

-- ------------------------------------------------------------
-- WHAT HAPPENS WITH INVALID DATA
-- ------------------------------------------------------------
-- Try to insert a piece that references a non-existent type:

INSERT INTO piece (guid, design_guid, type_guid) 
VALUES ('piece-1', 'design-1', 'non-existent-type');

-- Result:
-- Error: FOREIGN KEY constraint failed
--
-- The database REJECTED the insert because 'non-existent-type'
-- doesn't exist in the type table. This prevents broken data
-- from ever entering the database.
```

**Cascading deletes**:

```typescript
// ============================================================
// EXAMPLE 3: HANDLING DELETIONS WITH CASCADING EFFECTS
// ============================================================
// Purpose: When deleting an entity, handle other entities that reference it
// Relates to: Cascading is about what happens to dependent data
// If you delete a type, what happens to pieces that use that type?
// There are multiple strategies - this shows the decision points
//
// js/semio/semio.ts

function deleteTypeWithCascade(kit: Kit, typeGuid: Guid): Kit { // Delete type and handle dependents
  
  // ------------------------------------------------------------
  // STRATEGY 1: CASCADE DELETE (delete dependent pieces too)
  // ------------------------------------------------------------
  // Remove all pieces that use this type
  // Pro: No orphan pieces
  // Con: User might lose work unexpectedly
  
  const updatedDesigns = kit.designs.map(d => ({       // Map over all designs
    ...d,                                              // Keep all design properties
    pieces: d.pieces?.filter(                          // Filter the pieces array
      p => p.typeGuid !== typeGuid                     // Keep pieces NOT using this type
    )                                                  // Pieces using this type are removed
  }));
  
  // ------------------------------------------------------------
  // ALTERNATIVE STRATEGY 2: SET NULL (orphan the pieces)
  // ------------------------------------------------------------
  // Set piece.typeGuid to undefined instead of deleting
  // Pro: User doesn't lose pieces
  // Con: Orphan pieces can't render (no type)
  //
  // const updatedDesigns = kit.designs.map(d => ({
  //   ...d,
  //   pieces: d.pieces?.map(p => 
  //     p.typeGuid === typeGuid ? { ...p, typeGuid: undefined } : p
  //   )
  // }));
  
  // ------------------------------------------------------------
  // ALTERNATIVE STRATEGY 3: PREVENT (don't allow deletion)
  // ------------------------------------------------------------
  // Throw error if type is in use
  // Pro: User must consciously handle dependencies first
  // Con: More user steps required
  //
  // const inUse = kit.designs.some(d => 
  //   d.pieces?.some(p => p.typeGuid === typeGuid)
  // );
  // if (inUse) throw new Error("Type is in use, cannot delete");
  
  // ------------------------------------------------------------
  // APPLY THE DELETION
  // ------------------------------------------------------------
  
  return {
    ...kit,                                            // Keep all kit properties
    types: kit.types.filter(t => t.guid !== typeGuid), // Remove the type
    designs: updatedDesigns                            // Use designs with cascaded pieces
  };
}
```

**Why integrity for semio**

Broken references cause runtime errors:

- Piece with missing type → can't render model
- Connection with missing piece → placement algorithm fails
- Orphan data → wasted storage and confusion

**What it enables**

- Reliable data access
- Predictable behavior
- Safe deletions with cascades
- Trust in stored data

**What it limits**

- Constraints add complexity
- Cascade decisions are design choices
- Integrity checks have performance cost
- Orphan data can accumulate

---

## Part 4: Advanced Architecture

### Chapter 10: How Systems Are Designed

#### 10.1 Architecture: The Big Picture

**Plain explanation**

Architecture is the high-level structure of a system—how the major pieces fit together. Just as a building's architecture determines where the rooms are, how people flow through spaces, and where the plumbing goes, software architecture determines how components are organized, how data flows, and how parts communicate.

semio's architecture is a **modular monorepo** with multiple language runtimes sharing a unified data model.

**Technical explanation**

**semio's architecture**:

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    semio SYSTEM ARCHITECTURE                             │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  User Interfaces (Presentation):                                        │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  Sketchpad (React)  │  Grasshopper (C#)  │  Desktop (Electron)  │    │
│  │  js/semio/sketchpad │  net/Semio.Grasshopper  │  js/desktop     │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                           │                                              │
│  Domain Logic (Shared):   │                                              │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  TypeScript         │  Python           │  C#        │  Go       │   │
│  │  js/semio/semio.ts  │  py/engine/       │  net/Semio │  go/repo  │   │
│  │  (Kit, Type,        │  engine.py        │  /Semio.cs │  /main.go │   │
│  │   Design, Piece,    │  (validation,     │  (models,  │  (CLI,    │   │
│  │   Connection, Diff) │   computation)    │  Goo types)│  tools)   │   │
│  └─────────────────────────────────────────────────────────────────┘    │
│                           │                                              │
│  Data Layer:              │                                              │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  SQLite (kit.db)    │  IndexedDB (Y.js) │  Files (.zip, .glb)   │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

**semio architectural decisions**:

| Decision             | Choice                  | Rationale                      |
|----------------------|-------------------------|--------------------------------|
| Monorepo vs polyrepo | Monorepo                | Schema sync across languages   |
| Microservices vs mono| Modular monolith        | Small team, complexity tradeoff|
| State management     | Y.js CRDT + XState      | Collaborative + predictable UI |
| Data format          | JSON (runtime) + SQLite | Portable kits, efficient queries|
| Build system         | Nx                      | Language-agnostic, caching     |

**Component communication**:

```
┌─────────────────────────────────────────────────────────────────────────┐
│  Sketchpad ←→ Engine Communication                                      │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  Browser (Sketchpad)           Server (Engine)                          │
│  ┌─────────────────┐           ┌─────────────────┐                      │
│  │ React + XState  │  GraphQL  │ Python + SQLite │                      │
│  │ Y.js (CRDT)     │←─────────→│ Validation      │                      │
│  │ Three.js (3D)   │           │ Computation     │                      │
│  └─────────────────┘           └─────────────────┘                      │
│                                                                          │
│  Grasshopper (Rhino)           Go CLI (repo)                            │
│  ┌─────────────────┐           ┌─────────────────┐                      │
│  │ C# Components   │   stdio   │ Ticket system   │                      │
│  │ Goo wrappers    │←─────────→│ MCP tools       │                      │
│  │ .NET runtime    │           │ Policy checks   │                      │
│  └─────────────────┘           └─────────────────┘                      │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

**Architecture documentation in semio**:

| Document             | Location              | Content                         |
|----------------------|-----------------------|---------------------------------|
| Data architecture    | `dataarchitecture.pu` | Entity relationships            |
| Software architecture| `softwarearchitecture.pu` | Module dependencies         |
| Interface specs      | `interfacearchitecture.txt` | API contracts              |
| Agent instructions   | `AGENTS.md`           | Decision records + patterns     |

**Why architecture for semio**

semio's architecture enables:

- Single schema definition, multiple language implementations
- Web, desktop, and plugin interfaces from shared core
- Collaborative editing (Y.js) + computational backend (Python)
- Local-first with optional cloud sync

**What it enables**

- Cross-platform deployment (browser, desktop, Rhino)
- Schema changes propagate to all languages
- Independent development of UI and Engine
- Offline capability with sync

**What it limits**

- Schema sync overhead across languages
- Test matrix across all platforms
- Complexity in maintaining parity
- Build times for full monorepo

---

#### 10.2 Layers: Separation of Concerns

**Plain explanation**

Layering organizes code into horizontal stacks where each layer has a specific job and only talks to the layers directly above or below it. Think of it like a company hierarchy: customers talk to sales, sales talks to operations, operations talks to manufacturing. Customers don't walk onto the factory floor.

semio has clear layers: UI (React, Grasshopper), Domain (Kit/Type/Design logic), and Data (SQLite, IndexedDB).

**Technical explanation**

**semio's layered architecture**:

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    semio LAYERS                                          │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  PRESENTATION LAYER (UI):                                               │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  Sketchpad React Components:                                    │    │
│  │    Home.tsx, Kit.tsx, Design.tsx, Type.tsx, Quality.tsx         │    │
│  │    - Canvas, Navbar, Footer, Panels                             │    │
│  │    - Three.js 3D rendering                                      │    │
│  │    - User interactions                                          │    │
│  │                                                                 │    │
│  │  Grasshopper Components:                                        │    │
│  │    Semio.Grasshopper.cs                                         │    │
│  │    - Goo wrappers for visual programming                        │    │
│  │    - Parameter definitions                                      │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                           │ uses                                         │
│                           ▼                                              │
│  DOMAIN LAYER (Business Logic):                                         │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  js/semio/semio.ts:                                             │    │
│  │    - Kit, Type, Design, Piece, Connection schemas               │    │
│  │    - Diff system (getDiff, applyDiff, inverseDiff, mergeDiff)   │    │
│  │    - Validation constraints                                     │    │
│  │    - Placement algorithms                                       │    │
│  │                                                                 │    │
│  │  py/engine/engine.py:                                           │    │
│  │    - Computation (placement, optimization)                      │    │
│  │    - Advanced validation                                        │    │
│  │                                                                 │    │
│  │  net/Semio/Semio.cs:                                            │    │
│  │    - C# domain models                                           │    │
│  │    - Validation and transformation                              │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                           │ uses                                         │
│                           ▼                                              │
│  DATA LAYER (Persistence):                                              │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  Sketchpad (Browser):                                           │    │
│  │    - Y.js documents (yTypes, yDesigns, yPieces)                 │    │
│  │    - IndexedDB persistence                                      │    │
│  │    - WebSocket sync                                             │    │
│  │                                                                 │    │
│  │  Engine (Server):                                               │    │
│  │    - SQLite kit.db                                              │    │
│  │    - File system for .zip kits                                  │    │
│  │                                                                 │    │
│  │  Grasshopper:                                                   │    │
│  │    - In-memory + JSON serialization                             │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

**Layer dependencies in semio**:

```typescript
// ============================================================
// EXAMPLE 1: LAYERED ARCHITECTURE IN PRACTICE
// ============================================================
// Purpose: Show how layers only depend on layers below them
// Relates to: Layered architecture enforces clear boundaries
// Presentation → Domain → Data (never backwards)
// This makes each layer independently testable and changeable
//
// js/semio/sketchpad/Design.tsx + js/semio/semio.ts

// ============================================================
// PRESENTATION LAYER → DOMAIN LAYER (allowed)
// ============================================================
// UI components import domain logic but never data layer directly
// Design.tsx (presentation) imports from semio.ts (domain)

import { applyKitDiff, getPieceDiff, computePiecePlane } from '../semio';
                                                         // Import domain functions
                                                         // These are pure functions
                                                         // No UI or database knowledge

// ------------------------------------------------------------
// BAD PATTERN: Presentation accessing Data directly
// ------------------------------------------------------------
// This would couple UI directly to storage mechanism
// If we change Y.js to something else, all UI code breaks

// ❌ import { yTypes } from './KitStore';  // BAD: UI accessing data layer
                                            // Creates tight coupling
                                            // Makes testing harder
                                            // Can't change storage without changing UI

// ------------------------------------------------------------
// GOOD PATTERN: Presentation uses abstraction (hooks)
// ------------------------------------------------------------
// Hooks hide the data layer behind a clean interface
// UI doesn't know (or care) if data comes from Y.js, SQLite, or API

// ✅ const kit = useKit();                 // GOOD: through abstraction
                                            // Hook hides Y.js details
                                            // UI gets clean Kit object
                                            // Storage can change without UI changes

// ============================================================
// DOMAIN LAYER: No upward dependencies
// ============================================================
// Domain layer (semio.ts) has NO imports from UI or data
// Pure functions only - given inputs, produce outputs
// This is the core business logic

export function computePiecePlane(                       // Domain function
  piece: Piece,                                          // Input: piece data
  type: Type,                                            // Input: type data
  connections: Connection[]                              // Input: connections
): Plane {                                               // Output: computed plane
  
  // Pure calculation, no UI or DB access
  // This function doesn't know about:
  // - React components (UI)
  // - Y.js documents (data)
  // - IndexedDB (persistence)
  //
  // It only knows about domain types: Piece, Type, Connection, Plane
  // This makes it testable without mocking UI or database
}

// ============================================================
// DATA LAYER → DOMAIN LAYER (allowed)
// ============================================================
// Data layer imports domain types to build snapshots
// KitStore uses Kit type to define what snapshot() returns

import { Kit, Type, Design } from '../semio';            // Import domain types
                                                         // Data layer knows about domain
                                                         // Converts storage → domain objects

export class KitStore extends Store<Kit> {               // Store parameterized by domain type
  snapshot(): Kit {                                      // Return domain type
    return kitFromYDoc(this.yDoc);                       // Convert Y.js → Kit
                                                         // Transformation at boundary
                                                         // Y.js details stay inside store
  }
}
```

**Cross-cutting concerns**:

```typescript
// ============================================================
// EXAMPLE 2: CONCERNS THAT SPAN MULTIPLE LAYERS
// ============================================================
// Purpose: Show code that legitimately crosses layer boundaries
// Relates to: Some features like logging and i18n need to work everywhere
// These are "cross-cutting concerns" - they cut across the normal layers
// We handle these carefully to avoid breaking layer isolation
//
// Various files across the codebase

// ------------------------------------------------------------
// LOGGING: Crosses all layers
// ------------------------------------------------------------
// Every layer needs to log for debugging
// We use a consistent prefix pattern to identify which layer

console.log('[DEBUG] [Design.tsx] Piece dropped');       // PRESENTATION layer log
                                                         // Shows user interaction
                                                         // [Design.tsx] identifies the component

console.log('[DEBUG] [semio.ts] Computing placement');   // DOMAIN layer log
                                                         // Shows business logic execution
                                                         // [semio.ts] identifies the module

console.log('[DEBUG] [KitStore] Y.js transaction');      // DATA layer log
                                                         // Shows persistence operations
                                                         // [KitStore] identifies the store

// Note: All logs use [DEBUG] prefix so they can be easily removed
// grep for [DEBUG] to find and clean up before release

// ------------------------------------------------------------
// INTERNATIONALIZATION (i18n): Crosses presentation layer
// ------------------------------------------------------------
// All user-facing text needs translation
// i18n is a cross-cutting concern within the presentation layer

const { t } = useTranslation();                          // Get translation function
                                                         // t() looks up translated strings
                                                         // Based on user's language setting

const label = t('semio.sketchpad.navbar.back');          // Get translated label
                                                         // Key matches UI element ID
                                                         // Returns "Go back" (en) or "Zurück" (de)
                                                         // Same key used for tooltip content
```

**Why layering for semio**

Layers enable:

- Change UI (React → Vue) without touching domain logic
- Change persistence (Y.js → plain JSON) without touching UI
- Test domain logic in isolation (pure functions)
- Different UIs (Sketchpad, Grasshopper) share same domain

**What it enables**

- Multiple UIs share domain logic
- Engine can run headless (no UI)
- Domain tests don't need React/browser
- Persistence can be swapped (IndexedDB, SQLite, cloud)

**What it limits**

- Data must be transformed at boundaries (Y.Map → Kit)
- Abstraction overhead for simple operations
- Type definitions duplicated across layers
- Performance cost of conversion

---

#### 10.3 Abstraction: Hiding Complexity

**Plain explanation**

Abstraction is showing only what's necessary and hiding everything else. When you drive a car, you see a steering wheel, pedals, and gauges. You don't see the engine's internal combustion, the transmission's gear ratios, or the fuel injection timing. These details are abstracted away behind simple interfaces.

semio abstracts complex operations like piece placement behind simple function calls.

**Technical explanation**

**semio abstractions**:

```typescript
// ============================================================
// EXAMPLE 1: ABSTRACTION LAYERS FOR PIECE PLACEMENT
// ============================================================
// Purpose: Show how complex operations are hidden behind simple interfaces
// Relates to: Each level of abstraction hides the complexity below it
// User sees "drop piece" → multiple layers handle the actual work
// Each layer only knows about the layer directly below it
//
// js/semio/sketchpad/Design.tsx + Sketchpad.tsx

// ============================================================
// HIGH-LEVEL ABSTRACTION: User sees "place piece"
// ============================================================
// This is what the UI code looks like
// Extremely simple - just call a command with the data
// All complexity is hidden in the command system

function handlePieceDropped(typeGuid: Guid, position: Point) { // Called when user drops a type
  executeCommand(                                              // Execute a command
    "semio.designApp.addPiece",                                // Command name (what to do)
    "semio.sketchpad.canvas.drop",                             // Origin (where it came from)
                                                               // Used for logging, tutorials
    { typeGuid, center: position }                             // Data (the piece to add)
                                                               // Just the type and position
  );
  // That's it! One function call from the UI.
  // Everything else happens automatically:
  // - Piece is created with proper GUID
  // - Diff is generated
  // - Y.js is updated
  // - Other users see the new piece
  // - Undo history is recorded
}

// ============================================================
// MID-LEVEL ABSTRACTION: Command creates piece and updates state
// ============================================================
// Command handler knows more details but still doesn't touch Y.js directly
// It works with domain types and diffs

registerCommand("semio.designApp.addPiece", (ctx, args) => {   // Register command handler
                                                               // ctx has current state
                                                               // args has the piece data
  
  const newPiece = createPiece(args.typeGuid, args.center);    // Create domain object
                                                               // createPiece is pure function
                                                               // Returns Piece with new GUID
  
  return {                                                     // Return a diff (not Y.js operations)
    kitDiff: {                                                 // Changes to apply to kit
      designs: {                                               // Changes to designs
        updated: [{                                            // One design being updated
          guid: designGuid,                                    // Which design
          diff: {                                              // What's changing
            pieces: { added: [newPiece] }                      // Adding a piece
          }
        }]
      }
    }
  };
  // Command returns a DIFF - a description of what changed
  // It doesn't actually modify anything yet
  // The diff system handles the actual application
});

// ============================================================
// LOW-LEVEL: Diff application handles Y.js updates
// ============================================================
// This is where Y.js actually gets modified
// Only this layer knows about Y.js internals

function applyKitDiffToYDoc(yDoc: Y.Doc, diff: KitDiff): void { // Apply diff to Y.js
  
  yDoc.transact(() => {                                         // Start Y.js transaction
                                                                // All changes batched together
    
    // Navigate the diff structure
    for (const added of diff.designs?.updated?.[0]?.diff?.pieces?.added ?? []) {
      // diff.designs?.updated?.[0]?.diff?.pieces?.added
      //      ↑          ↑      ↑     ↑       ↑       ↑
      //      |          |      |     |       |       +-- array of new pieces
      //      |          |      |     |       +---------- pieces sub-diff
      //      |          |      |     +------------------ the design's diff
      //      |          |      +------------------------ first updated design
      //      |          +------------------------------- updated designs array
      //      +------------------------------------------ designs sub-diff
      
      const yPiece = yMapFromPiece(added);                      // Convert Piece → Y.Map
                                                                // Y.Map is Y.js data structure
      
      yPieces.push([yPiece]);                                   // Add to Y.js array
                                                                // This triggers sync to others
    }
  });
}
```

**Abstraction levels in semio**:

| Level        | What User Sees        | What's Hidden                        |
|--------------|-----------------------|--------------------------------------|
| UI           | Drag piece to canvas  | Event handling, coordinates          |
| Command      | `addPiece(type, pos)` | Diff generation, validation          |
| Diff         | `{ pieces: added }`   | Y.js operations, sync                |
| Y.js         | `yArray.push()`       | CRDT merging, persistence            |
| IndexedDB    | Automatic persistence | Binary storage, transactions         |

**Interface abstraction** (`js/semio/semio.ts`):

```typescript
// ============================================================
// EXAMPLE 2: HIDING COMPLEX COMPATIBILITY LOGIC
// ============================================================
// Purpose: Users work with simple interface, complexity hidden inside
// Relates to: Good interfaces are simple to use, complex to implement
// Connector users just see "is compatible?" - don't know the rules
// Compatibility rules are complex but hidden behind one function call
//
// js/semio/semio.ts

// ------------------------------------------------------------
// SIMPLE INTERFACE: What users of Connector see
// ------------------------------------------------------------
// Connector interface is simple - just data properties
// Users don't need to understand compatibility rules

interface Connector {
  id: string;                        // Connector identifier (like "top", "bottom")
  point: Point;                      // 3D position
  direction: Vector;                 // Which way it faces
  interface?: InterfaceId;           // Just a GUID reference
                                     // Optional - undefined means "default"
                                     // Users don't need to know what this does
}

// ------------------------------------------------------------
// HIDDEN COMPLEXITY: What's inside the compatibility function
// ------------------------------------------------------------
// This function hides all the complex logic
// Caller just asks "are these two connectors compatible?"
// Function handles all the edge cases internally

function areConnectorsCompatible(                        // Simple function signature
  a: Connector,                                          // First connector
  b: Connector,                                          // Second connector
  interfaces: Interface[]                                // Kit's interface definitions
): boolean {                                             // Returns simple yes/no
  
  // Complex compatibility logic hidden from callers:
  
  // Rule 1: No interface specified = compatible with everything
  if (!a.interface && !b.interface) return true;         // Both undefined → compatible
                                                         // This is the "default" behavior
  
  // Rule 2: Same interface = compatible
  if (a.interface === b.interface) return true;          // Same interface → compatible
                                                         // Simple equality check
  
  // Rule 3: Look up interface definitions for more complex rules
  const aInterface = interfaces.find(i => i.guid === a.interface); // Find a's interface
  const bInterface = interfaces.find(i => i.guid === b.interface); // Find b's interface
  
  // Rule 4: Check compatibility lists (complex bidirectional check)
  // ... more complex logic hidden here
  // - Check if a's interface lists b's as compatible
  // - Check if b's interface lists a's as compatible
  // - Handle empty compatibility lists (means "compatible with all")
  // - Handle missing interfaces gracefully
  
  return checkCompatibility(aInterface, bInterface);     // Final complex check
                                                         // All details hidden inside
}

// USAGE (simple):
const canConnect = areConnectorsCompatible(connectorA, connectorB, kit.interfaces);
// User doesn't need to understand ANY of the rules above
// Just gets true or false
```

**Data abstraction**: Hide representation behind operations

```typescript
// ============================================================
// EXAMPLE 3: DATA ABSTRACTION - HIDING INTERNAL REPRESENTATION
// ============================================================
// Purpose: Work with data through operations, not direct structure
// Relates to: Users don't need to know HOW data is stored
// Point stores x,y,z internally, but users just "translate" it
// They don't need to know Point is { x: number, y: number, z: number }
//
// js/semio/semio.ts

// ------------------------------------------------------------
// SIMPLE INTERFACE: Users just call translatePoint()
// ------------------------------------------------------------
// Translation is a simple concept: "move a point by a vector"
// User doesn't need to know Point has x, y, z fields
// Could be stored differently and function would still work

function translatePoint(p: Point, v: Vector): Point {        // Simple signature
                                                              // Takes a point and vector
                                                              // Returns new point
  
  // Internal implementation (users don't need to see this):
  return {                                                   // Create new point
    x: p.x + v.x,                                            // Add x components
    y: p.y + v.y,                                            // Add y components  
    z: p.z + v.z                                             // Add z components
  };
  
  // If Point's internal representation changed (to array, etc.)
  // only this function would need to change, not the callers
}

// ------------------------------------------------------------
// COMPLEX ABSTRACTION: Plane to Matrix4 conversion
// ------------------------------------------------------------
// Planes have origin and axes in semio's coordinate system
// Three.js uses a different coordinate system (Matrix4)
// This abstraction HIDES the coordinate system complexity
// Users just get a Matrix4 they can use in Three.js

function planeToMatrix4(plane: Plane): THREE.Matrix4 {       // Simple signature
                                                              // Takes semio Plane
                                                              // Returns Three.js Matrix4
  
  // HIDDEN COMPLEXITY: Coordinate system transformation
  // semio uses: X=right, Y=forward, Z=up (left-handed)
  // Three.js uses: X=right, Y=up, Z=backward (right-handed)
  // Users don't need to know about this conversion!
  
  const origin = new THREE.Vector3(                          // Convert origin
    plane.origin.x,                                          // X stays the same
    plane.origin.z,                                          // semio Z → Three.js Y
    -plane.origin.y                                          // semio Y → Three.js -Z
  );
  
  // Similar axis conversions...
  // ... more coordinate system transformation hidden here
  // - Convert X-axis from semio to Three.js coordinates
  // - Convert Y-axis from semio to Three.js coordinates
  // - Build rotation matrix from converted axes
  // - Combine translation and rotation into final matrix
  
  const matrix = new THREE.Matrix4();                        // Create matrix
  matrix.makeBasis(xAxis, yAxis, zAxis);                     // Set rotation
  matrix.setPosition(origin);                                // Set translation
  
  return matrix;                                             // Return complete transform
}

// USAGE (simple):
const pieceTransform = planeToMatrix4(piece.plane);
mesh.applyMatrix4(pieceTransform);
// User doesn't need to understand coordinate system differences!
```

**Why abstraction for semio**

Without abstraction:

- Every component would handle Y.js directly
- Every connection would compute coordinate transforms
- Every UI element would know about CRDT internals

With abstraction:

- Components use `useKit()` hook—don't know about Y.js
- Placement uses `computePiecePlane()`—don't know matrix math
- UI uses `applyDiff()`—don't know sync protocol

**What it enables**

- Simple APIs for complex operations
- Change internals without affecting users
- Focus on business logic, not plumbing

**What it limits**

- Leaky abstractions when debugging
- Performance overhead from layers
- Wrong abstraction boundaries cause problems

---

#### 10.4 Coupling: How Much Things Depend on Each Other

**Plain explanation**

Coupling is how much one piece of code depends on another. If you change module A and module B breaks, they're tightly coupled. If A and B can change independently, they're loosely coupled.

Think of a phone charger. A proprietary charger is tightly coupled to one phone model. A USB-C charger is loosely coupled—it works with any USB-C device. In semio, packages are loosely coupled—changing the TypeScript UI doesn't break the Python engine.

**Technical explanation**

**Coupling in semio monorepo**:

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    semio COUPLING DIAGRAM                                │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  LOOSELY COUPLED (communicate via JSON/GraphQL):                        │
│                                                                          │
│    @semio/js ─────JSON Kit────→ py/engine                               │
│    @semio/js ─────GraphQL─────→ py/engine                               │
│    @semio/net ────JSON Kit────→ py/engine                               │
│    @semio/vscode ─CLI JSON────→ go/repo                                 │
│                                                                          │
│  TIGHTLY COUPLED (share source code):                                   │
│                                                                          │
│    Sketchpad.tsx ←───imports────→ semio.ts                              │
│    Design.tsx ←───imports────→ semio.ts                                 │
│    elements.tsx ←───imports────→ @semio/js                              │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

**Coupling types in semio**:

```typescript
// ============================================================
// EXAMPLE 1: TIGHT VS LOOSE COUPLING PATTERNS
// ============================================================
// Purpose: Show different ways code can depend on other code
// Relates to: Loose coupling enables independent changes
// Tight coupling is OK within packages, risky between packages
// semio uses loose coupling for cross-language communication
//
// Multiple files across monorepo

// ------------------------------------------------------------
// TIGHT COUPLING: Direct imports (within same package)
// ------------------------------------------------------------
// This is OK! Files in same package SHOULD know about each other
// js/semio/sketchpad/Design.tsx

import {                                                     // Direct TypeScript import
  Kit,                                                       // Import Kit type
  Type,                                                      // Import Type type
  Piece,                                                     // Import Piece type
  applyKitDiff                                               // Import function directly
} from '../semio';                                           // From same JS package
                                                              // If semio.ts changes, this file may break
                                                              // That's OK - they're in same package
                                                              // Developer controls both files

// TIGHT COUPLING CHARACTERISTICS:
// ✓ Fast - no serialization overhead
// ✓ Type-safe - compiler catches errors
// ✗ Changes in imported module break importer
// ✗ Can only work with same language
// ✓ OK within a package
// ✗ Bad between different packages/languages

// ------------------------------------------------------------
// LOOSE COUPLING: JSON interface (between packages)
// ------------------------------------------------------------
// Different languages communicate via JSON
// py/engine/engine.py

def load_kit(kit_json: str) -> Kit:                          # Takes JSON string
    """Load a Kit from JSON - loosely coupled to TypeScript"""
    data = json.loads(kit_json)                              # Parse JSON to dict
                                                              # JSON is the "contract"
                                                              # TypeScript sends JSON
                                                              # Python parses JSON
                                                              # Neither imports the other
    return Kit.from_dict(data)                               # Convert to Python object

# LOOSE COUPLING CHARACTERISTICS:
# ✓ Languages can change independently
# ✓ Different deployment schedules
# ✓ Easy to version APIs
# ✗ Serialization overhead
# ✗ No compile-time type checking
# ✓ Good between packages/languages

// ------------------------------------------------------------
// DEPENDENCY INJECTION: Swappable implementations
// ------------------------------------------------------------
// Instead of creating dependencies directly, receive them
// js/semio/sketchpad/Sketchpad.tsx

interface RemoteProviders {                                  // Define INTERFACE
                                                              // Not a concrete class
  yProvider?: (guid: Guid) => YProvider;                     // Optional Y.js provider
  fileProvider?: FileProvider;                               // Optional file provider
}

export function Sketchpad({                                  // Component receives providers
  remoteProviders                                            // Passed in from outside
}: {
  remoteProviders?: RemoteProviders                          // Optional configuration
}) {
  // Uses injected providers, doesn't know concrete implementation
  // Could be MemoryProvider for testing
  // Could be CloudProvider for production
  // Sketchpad doesn't care - just uses the interface
  
  const files = remoteProviders?.fileProvider;               // Use what was injected
  if (files) {
    files.upload(kitId, fileId, path, blob);                 // Call interface method
                                                              // Don't know if memory/local/cloud
  }
}

// DEPENDENCY INJECTION CHARACTERISTICS:
// ✓ Easy to test with mocks
// ✓ Can swap implementations at runtime
// ✓ Reduces tight coupling
// ✗ More setup code required
// ✓ Highly recommended pattern
```

**Event-based decoupling** (XState):

```typescript
// ============================================================
// EXAMPLE 2: EVENT-BASED DECOUPLING
// ============================================================
// Purpose: Sender and receiver don't know about each other
// Relates to: Events act as contracts between components
// Component sends event, handler receives event
// Neither knows about the other directly
//
// js/semio/sketchpad/Sketchpad.tsx

// ------------------------------------------------------------
// TIGHT COUPLING (what we avoid): Direct function calls
// ------------------------------------------------------------
// If we used direct calls:
// designStore.setSelection(pieces);                         // TIGHT!
                                                              // Caller must import designStore
                                                              // Caller must know exact method name
                                                              // If method signature changes, caller breaks
                                                              // Hard to intercept for logging/undo

// ------------------------------------------------------------
// LOOSE COUPLING: Send events through actor
// ------------------------------------------------------------
// Instead of calling methods, send events
// Actor is the middleman - decouples sender from receiver

actor.send({                                                 // Send event to actor
  type: 'DESIGN.SELECT_PIECE',                               // Event type (contract)
                                                              // Sender doesn't know who handles this
  pieceGuid                                                  // Event payload (data)
});

// Benefits of event-based approach:
// ✓ Sender doesn't import handler
// ✓ Multiple handlers can respond to same event
// ✓ Easy to add logging, undo, replay
// ✓ Handler can change without affecting sender

// ------------------------------------------------------------
// EVENT HANDLER: Registered separately from sender
// ------------------------------------------------------------
// Handler doesn't know WHO sent the event
// Just knows HOW to handle it

registerEventHandler('DESIGN.SELECT_PIECE', {                // Register for event type
                                                              // Handler doesn't know sender
  action: (context, event) => ({                             // Receive event with payload
    designApp: {                                             // Return state update
      ...context.designApp,                                  // Keep existing state
      selection: {                                           // Update selection
        pieces: [event.pieceGuid]                            // Use payload from event
      }
    }
  })
});

// The "contract" between sender and handler is just the event type
// Both agree on 'DESIGN.SELECT_PIECE' and its payload shape
// Neither needs to import the other
// This is loose coupling via messaging
```

**Interface decoupling**:

```typescript
// ============================================================
// EXAMPLE 3: INTERFACE-BASED DECOUPLING
// ============================================================
// Purpose: Code depends on interfaces, not implementations
// Relates to: Swap implementations without changing consumers
// Testing uses MemoryFileProvider, production uses RemoteFileProvider
// Consumer code is identical - only provider changes
//
// js/semio/sketchpad/Sketchpad.tsx

// ------------------------------------------------------------
// THE INTERFACE: Contract that all implementations must follow
// ------------------------------------------------------------
// This is just a specification - no actual code
// Any class implementing this interface must have these methods

interface FileProvider {                                     // Define the contract
                                                              // All file providers must match this

  upload: (                                                  // Upload method signature
    kitId: string,                                           // Which kit
    fileId: string,                                          // Which file
    path: string,                                            // File path
    blob: Blob                                               // File contents
  ) => Promise<string>;                                      // Returns URL
  
  download: (                                                // Download method signature
    kitId: string,                                           // Which kit
    fileId: string,                                          // Which file
    path: string                                             // File path
  ) => Promise<Blob>;                                        // Returns file contents
  
  delete: (                                                  // Delete method signature
    kitId: string, fileId: string, path: string              // Same parameters
  ) => Promise<void>;                                        // Returns nothing
  
  getUrl: (                                                  // Get URL method signature
    kitId: string, fileId: string, path: string              // Same parameters
  ) => string;                                               // Returns URL string
}

// ------------------------------------------------------------
// MULTIPLE IMPLEMENTATIONS: Each fulfills the same contract
// ------------------------------------------------------------
// All three classes implement FileProvider interface
// Consumer doesn't know which one is being used

class MemoryFileProvider implements FileProvider {           // Testing implementation
  // Stores files in memory (Map)                            // Fast, no persistence
  // Perfect for unit tests                                  // No network/disk needed
  private files = new Map<string, Blob>();                   // In-memory storage
  
  async upload(kitId: string, fileId: string, path: string, blob: Blob) {
    this.files.set(`${kitId}/${fileId}/${path}`, blob);      // Store in memory
    return `memory://${kitId}/${fileId}/${path}`;            // Return fake URL
  }
  // ... other methods
}

class LocalFileProvider implements FileProvider {            // Browser implementation
  // Stores files in IndexedDB                               // Persists locally
  // Works offline                                           // No server needed
  // ... uses IndexedDB for storage
}

class RemoteFileProvider implements FileProvider {           // Production implementation
  // Stores files on cloud server                            // Persists remotely
  // Requires network                                        // Scales to many users
  // ... uses HTTP/REST for storage
}

// ------------------------------------------------------------
// CONSUMER CODE: Same regardless of implementation
// ------------------------------------------------------------
// This code works with ANY FileProvider
// Doesn't know or care which implementation is used

function uploadModelFile(                                    // Consumer function
  provider: FileProvider,                                    // Accepts any FileProvider
  kit: Kit,
  file: File
) {
  return provider.upload(                                    // Calls interface method
    kit.guid,                                                // Pass parameters
    generateGuid(),
    file.name,
    file
  );
  // This works with Memory, Local, OR Remote provider
  // Consumer code never changes
  // Only inject different provider at setup time
}

// USAGE:
// Test: uploadModelFile(new MemoryFileProvider(), kit, file)
// Dev:  uploadModelFile(new LocalFileProvider(), kit, file)
// Prod: uploadModelFile(new RemoteFileProvider(), kit, file)
// Same function, different behavior based on injected provider
```

**Why decoupling matters for semio**

- TypeScript, Python, C#, Go can evolve independently
- Sketchpad (browser) and Engine (server) deploy separately
- VS Code extension doesn't break if Grasshopper changes
- Tests can mock file providers

**What it enables**

- Independent package development
- Different deployment cycles
- Swappable backends
- Parallel team work

**What it limits**

- JSON boundaries require serialization
- Interface changes break multiple packages
- Debugging across process boundaries is harder

---

#### 10.5 Cohesion: How Much Things Belong Together

**Plain explanation**

Cohesion is how well the parts of a module fit together. A highly cohesive module does one thing well—all its pieces work toward a single purpose. A module with low cohesion is a grab-bag of unrelated functions that happen to be in the same file.

In semio, `semio.ts` is highly cohesive: everything relates to kit-of-parts domain logic. `Sketchpad.tsx` is cohesive for app orchestration.

**Technical explanation**

**Cohesion in semio packages**:

| Package | Purpose | Cohesion Level | Contents |
|---------|---------|----------------|----------|
| `semio.ts` | Domain logic | HIGH | Kit/Type/Design/Piece schemas, diffs, validation |
| `Sketchpad.tsx` | App orchestration | HIGH | State machine, providers, routing |
| `Design.tsx` | Design editing | HIGH | Piece/connection editing, selection, tools |
| `elements.tsx` | UI primitives | HIGH | Table, Window, Panel, Canvas |
| `Semio.cs` | C# domain | HIGH | Models, serialization, validation |

**Functional cohesion in semio.ts**:

```typescript
// ============================================================
// EXAMPLE 1: HIGH COHESION - SINGLE PURPOSE MODULE
// ============================================================
// Purpose: Show a highly cohesive file where everything relates
// Relates to: Good modules do ONE thing well
// semio.ts only contains kit-of-parts domain logic
// Nothing about UI, networking, or file storage
//
// js/semio/semio.ts

// ------------------------------------------------------------
// SCHEMA DEFINITIONS: All about Kit/Type/Design structure
// ------------------------------------------------------------
// Every interface here is about the kit-of-parts domain
// Nothing about React, databases, or networking

export interface Kit {                                       // Kit schema
  types: Type[];                                             // Contains types
  designs: Design[];                                         // Contains designs
  ...                                                        // Other kit properties
}

export interface Type {                                      // Type schema
  connectors: Connector[];                                   // Type's connectors
  models: Model[];                                           // Type's 3D models
  ...                                                        // Other type properties
}

export interface Design {                                    // Design schema
  pieces: Piece[];                                           // Design's pieces
  connections: Connection[];                                 // Design's connections
  ...                                                        // Other design properties
}

// COHESION: All schemas relate to the same domain

// ------------------------------------------------------------
// DIFF OPERATIONS: All work with Kit/Type/Design/Piece
// ------------------------------------------------------------
// Every function here transforms domain types
// Same concepts, same data types

export function getKitDiff(                                  // Calculate difference
  before: Kit,                                               // Kit before change
  after: Kit                                                 // Kit after change
): KitDiff { ... }                                           // Returns the diff

export function applyKitDiff(                                // Apply difference
  kit: Kit,                                                  // Original kit
  diff: KitDiff                                              // Diff to apply
): Kit { ... }                                               // Returns modified kit

export function inverseKitDiff(                              // Reverse a diff
  kit: Kit,                                                  // Reference kit
  diff: KitDiff                                              // Diff to invert
): KitDiff { ... }                                           // Returns inverted diff

export function mergeKitDiff(                                // Combine diffs
  a: KitDiff,                                                // First diff
  b: KitDiff                                                 // Second diff
): KitDiff { ... }                                           // Returns merged diff

// COHESION: All diff functions work on same domain types

// ------------------------------------------------------------
// VALIDATION: All validate domain types
// ------------------------------------------------------------
// Every function here checks domain data

export function validateKit(kit: Kit): ValidationResult { ... }         // Validate kit
export function areConnectorsCompatible(...): boolean { ... }           // Check connectors

// COHESION: All validation is about domain constraints

// ------------------------------------------------------------
// PLACEMENT: All compute piece positions
// ------------------------------------------------------------
// Every function here does geometry for the domain

export function computePiecePlane(...): Plane { ... }        // Compute piece position
export function computeConnectionPlane(...): Plane { ... }   // Compute via connection

// COHESION: All placement is about domain geometry

// WHY THIS FILE IS HIGHLY COHESIVE:
// ✓ Every definition relates to kit-of-parts domain
// ✓ No UI code (React, components)
// ✓ No storage code (databases, files)
// ✓ No network code (HTTP, WebSocket)
// ✓ Can be tested in isolation
// ✓ Easy to understand purpose: "domain logic"
```

**Low cohesion anti-pattern** (what semio avoids):

```typescript
// ============================================================
// EXAMPLE 2: LOW VS HIGH COHESION COMPARISON
// ============================================================
// Purpose: Contrast bad (low cohesion) with good (high cohesion)
// Relates to: Avoid mixing unrelated concerns in one file
// Low cohesion makes code hard to understand and maintain
// High cohesion keeps related code together
//
// Anti-pattern vs actual semio organization

// ------------------------------------------------------------
// BAD: Random utilities file mixing unrelated concerns
// ------------------------------------------------------------
// ❌ utils.ts (low cohesion) - DON'T DO THIS
// This file has no clear purpose - it's a "junk drawer"

export function formatDate(d: Date): string { ... }          // Date formatting
                                                              // What does this have to do with...

export function calculateKitStats(kit: Kit): Stats { ... }   // Kit statistics
                                                              // ...kit statistics?

export function throttle(fn: Function): Function { ... }     // Performance utility
                                                              // And this is completely different!

export function validateEmail(email: string): boolean { ... } // Email validation
                                                              // No connection to any above

// PROBLEMS WITH LOW COHESION:
// ✗ Hard to name the file (what IS it about?)
// ✗ Changes to dates might break kit stats
// ✗ Testing requires mocking unrelated things
// ✗ New code gets dumped here, file grows forever
// ✗ Multiple developers editing same file = conflicts

// ------------------------------------------------------------
// GOOD: Each concern in its own cohesive module
// ------------------------------------------------------------
// ✅ How semio actually organizes code

// semio.ts - kit domain ONLY
export interface Kit { ... }                                 // Domain schemas
export function applyKitDiff(...) { ... }                    // Domain operations
export function validateKit(...) { ... }                     // Domain validation
// COHESIVE: Everything about kit-of-parts domain

// i18n.ts - localization ONLY
export function initI18n() { ... }                           // Initialize translations
export function t(key: string) { ... }                       // Get translation
export function changeLanguage(lang: string) { ... }         // Switch language
// COHESIVE: Everything about internationalization

// elements.tsx - UI primitives ONLY
export function Table(...) { ... }                           // Table component
export function Panel(...) { ... }                           // Panel component
export function Button(...) { ... }                          // Button component
// COHESIVE: Everything about UI building blocks

// BENEFITS OF HIGH COHESION:
// ✓ Clear file purpose (easy naming)
// ✓ Changes are isolated to relevant file
// ✓ Tests focus on one concern
// ✓ Teams can own specific files
// ✓ Easy to find related code
```

**App cohesion pattern**:

```typescript
// ============================================================
// EXAMPLE 3: APP-LEVEL COHESION
// ============================================================
// Purpose: Show how each app file contains everything for that app
// Relates to: Keep related code in the same place
// Design.tsx has ALL design editing code
// Type.tsx has ALL type editing code
// No cross-file searching needed
//
// js/semio/sketchpad/Design.tsx (example structure)

// ------------------------------------------------------------
// ONE APP = ONE FILE (everything related together)
// ------------------------------------------------------------
// Design.tsx contains:
// 1. State interface for the design app
// 2. Hooks for reading/writing that state
// 3. Commands for actions users can take
// 4. Event handlers for state transitions
// 5. React components for the UI

// 1. STATE: Design app's data structure
interface DesignAppState {                                   // What the design app tracks
  selection: {                                               // Currently selected items
    pieces: Guid[];                                          // Selected piece IDs
    connections: Guid[];                                     // Selected connection IDs
  };
  hover: {                                                   // Currently hovered items
    pieces: Guid[];                                          // Hovered piece IDs
  };
  camera: Camera;                                            // 3D view camera position
  activeTool: ToolKind;                                      // Current editing tool
}

// 2. HOOKS: Access design app state from React
function useDesignAppSelection() {                           // Get/set selection
  // Returns [selection, setSelection, canSetSelection]
  ...
}

function useDesignAppHover() {                               // Get/set hover
  // Returns [hover, setHover, canSetHover]
  ...
}

// 3. COMMANDS: Actions users can perform
registerCommand("semio.designApp.addPiece", ...);            // Add piece command
registerCommand("semio.designApp.deletePiece", ...);         // Delete piece command
registerCommand("semio.designApp.connectPieces", ...);       // Connect pieces command

// 4. EVENT HANDLERS: State machine transitions
registerEventHandler("DESIGN.SELECT_PIECE", ...);            // Handle piece selection
registerEventHandler("DESIGN.DESELECT_PIECE", ...);          // Handle deselection
registerEventHandler("DESIGN.SET_TOOL", ...);                // Handle tool change

// 5. COMPONENTS: React UI for design editing
function DesignCanvas() { ... }                              // 3D canvas component
function DesignDiagram() { ... }                             // 2D diagram component
function DesignToolbar() { ... }                             // Toolbar component

// WHY THIS IS COHESIVE:
// ✓ Everything about design editing is in Design.tsx
// ✓ Developer looking for design code → look in Design.tsx
// ✓ Changes to design editing → only modify Design.tsx
// ✓ Easy to understand: "this file = design app"
// ✓ Same pattern for Type.tsx, Kit.tsx, Quality.tsx
```

**Why cohesion matters for semio**

- `semio.ts` is easy to test in isolation (pure domain logic)
- Each app file has clear purpose and ownership
- Changes to design editing stay in `Design.tsx`
- New developers find related code together

**What it enables**

- Clear module purposes
- Localized changes
- Independent testing
- Team ownership of modules

**What it limits**

- Cross-cutting concerns (logging, i18n) span modules
- Judgment required for boundaries
- Sometimes related code needs to be in different layers

---

#### 10.6 Interfaces: Contracts Between Components

**Plain explanation**

An interface is a contract that says "if you want to work with me, here's what I promise to provide." It's like a job description—it specifies what capabilities exist without describing how they're implemented.

In semio, `FileProvider` is an interface that promises `upload`, `download`, `delete`, `getUrl`. Any implementation (memory, IndexedDB, cloud) works if it fulfills the contract.

**Technical explanation**

**Interfaces in semio**:

```typescript
// ============================================================
// EXAMPLE 1: FILE PROVIDER INTERFACE AND IMPLEMENTATIONS
// ============================================================
// Purpose: Show how one interface can have multiple implementations
// Relates to: Interfaces define contracts, classes fulfill them
// MemoryFileProvider, LocalFileProvider, RemoteFileProvider
// all work because they fulfill the same contract
//
// js/semio/sketchpad/Sketchpad.tsx

// ------------------------------------------------------------
// THE CONTRACT: What every file provider MUST do
// ------------------------------------------------------------
// Interface = specification only (no actual code)
// Says "here's what you must provide" without "here's how"

interface FileProvider {                                     // Contract definition
                                                              // Any file provider must have:
  
  upload: (                                                  // 1. Upload method
    kitId: string,                                           //    Takes kit identifier
    fileId: string,                                          //    Takes file identifier
    path: string,                                            //    Takes file path
    blob: Blob                                               //    Takes file data
  ) => Promise<string>;                                      //    Returns URL when done
  
  download: (                                                // 2. Download method
    kitId: string,                                           //    Same parameters
    fileId: string,
    path: string
  ) => Promise<Blob>;                                        //    Returns file data
  
  delete: (                                                  // 3. Delete method
    kitId: string, fileId: string, path: string              //    Same parameters
  ) => Promise<void>;                                        //    Returns nothing
  
  getUrl: (                                                  // 4. Get URL method
    kitId: string, fileId: string, path: string              //    Same parameters
  ) => string;                                               //    Returns URL string
}

// ------------------------------------------------------------
// IMPLEMENTATION 1: Memory provider (for testing)
// ------------------------------------------------------------
// Stores files in memory using a Map
// Fast, no persistence, perfect for unit tests

class MemoryFileProvider implements FileProvider {           // Says "I fulfill FileProvider"
  private files = new Map<string, Blob>();                   // In-memory storage
  
  async upload(kitId, fileId, path, blob) {                  // Fulfills upload contract
    const key = `${kitId}/${fileId}/${path}`;                // Build storage key
    this.files.set(key, blob);                               // Store in Map
    return URL.createObjectURL(blob);                        // Return object URL
  }
  
  async download(kitId, fileId, path) {                      // Fulfills download contract
    const key = `${kitId}/${fileId}/${path}`;                // Build storage key
    return this.files.get(key)!;                             // Return from Map
  }
  
  async delete(kitId, fileId, path) {                        // Fulfills delete contract
    this.files.delete(`${kitId}/${fileId}/${path}`);         // Remove from Map
  }
  
  getUrl(kitId, fileId, path) {                              // Fulfills getUrl contract
    return `memory://${kitId}/${fileId}/${path}`;            // Return fake URL
  }
}

// ------------------------------------------------------------
// IMPLEMENTATION 2: Local provider (browser storage)
// ------------------------------------------------------------
// Stores files in IndexedDB for offline persistence

class LocalFileProvider implements FileProvider {            // Also fulfills FileProvider
  async upload(kitId, fileId, path, blob) {                  // Same signature as interface
    const db = await openDB('semio-files');                  // Open IndexedDB
    await db.put('files', blob, `${kitId}/${fileId}/${path}`);// Store in database
    return `indexeddb://${kitId}/${fileId}/${path}`;         // Return IndexedDB URL
  }
  // ... other methods similar pattern
}

// ------------------------------------------------------------
// IMPLEMENTATION 3: Remote provider (cloud storage)
// ------------------------------------------------------------
// Stores files on a cloud server via HTTP

class RemoteFileProvider implements FileProvider {           // Also fulfills FileProvider
  async upload(kitId, fileId, path, blob) {                  // Same signature as interface
    const response = await fetch(                            // Make HTTP request
      `/api/kits/${kitId}/files/${fileId}`,                  // To server endpoint
      {
        method: 'PUT',                                       // PUT = upload
        body: blob                                           // File data in body
      }
    );
    const data = await response.json();                      // Parse response
    return data.url;                                         // Return server URL
  }
  // ... other methods similar pattern
}

// USAGE: Consumer doesn't know which implementation is used
function saveModel(provider: FileProvider, kitId: string, file: File) {
  return provider.upload(kitId, generateGuid(), file.name, file);
  // Works with Memory, Local, OR Remote - consumer doesn't care
}
```

**Store interface pattern**:

```typescript
// ============================================================
// EXAMPLE 2: ABSTRACT CLASS AS INTERFACE
// ============================================================
// Purpose: Show base class defining contract for all stores
// Relates to: Abstract methods MUST be implemented by subclasses
// KitStore, DesignAppStore, TypeAppStore all extend Store
// Each provides its own implementation of abstract methods
//
// js/semio/sketchpad/Sketchpad.tsx

// ------------------------------------------------------------
// ABSTRACT BASE CLASS: Defines what all stores must have
// ------------------------------------------------------------
// Abstract = "some methods have no implementation"
// Subclasses MUST provide those implementations

abstract class Store<TState> {                               // Generic over state type
                                                              // Store<Kit>, Store<DesignAppState>, etc.
  
  // ABSTRACT METHODS: Subclasses MUST implement these
  abstract snapshot(): TState;                               // Return current state
                                                              // No implementation here - subclass provides
  
  abstract hash(state: TState): string;                      // Generate hash for caching
                                                              // No implementation here - subclass provides
  
  abstract buildSnapshot(): TState;                          // Build state from Y.js
                                                              // No implementation here - subclass provides
  
  // CONCRETE PROPERTIES: All stores get these for free
  onChanged: Observable<void>;                               // Fires on shallow changes
                                                              // Base class provides implementation
  
  onChangedDeep: Observable<void>;                           // Fires on deep changes
                                                              // Base class provides implementation
  
  // CONCRETE METHODS: All stores get these behaviors
  protected notifyChanged() {                                // Notify listeners
    this.onChanged.emit();                                   // Emit shallow change event
  }
  
  protected notifyChangedDeep() {                            // Notify deep listeners
    this.onChangedDeep.emit();                               // Emit deep change event
  }
}

// ------------------------------------------------------------
// CONCRETE SUBCLASS 1: KitStore
// ------------------------------------------------------------
// Stores Kit data (types, designs, etc.)

class KitStore extends Store<Kit> {                          // Extends base, provides Kit state
  private yDoc: Y.Doc;                                       // Y.js document for this kit
  
  snapshot(): Kit {                                          // IMPLEMENTS abstract method
    return this.buildSnapshot();                             // Returns Kit object
  }
  
  hash(state: Kit): string {                                 // IMPLEMENTS abstract method
    return JSON.stringify(state);                            // Hash using JSON
  }
  
  buildSnapshot(): Kit {                                     // IMPLEMENTS abstract method
    return {                                                 // Build Kit from Y.js maps
      types: this.yTypes.toJSON(),                           // Convert Y.Array to array
      designs: this.yDesigns.toJSON(),                       // Convert Y.Array to array
      ...                                                    // Other kit properties
    };
  }
}

// ------------------------------------------------------------
// CONCRETE SUBCLASS 2: DesignAppStore
// ------------------------------------------------------------
// Stores Design app UI state (selection, hover, camera)

class DesignAppStore extends Store<DesignAppState> {         // Extends base, provides app state
  
  snapshot(): DesignAppState {                               // IMPLEMENTS abstract method
    return {                                                 // Returns UI state
      selection: this.ySelection.toJSON(),                   // Convert from Y.js
      hover: this.yHover.toJSON(),                           // Convert from Y.js
      camera: this.yCamera.toJSON(),                         // Convert from Y.js
      activeTool: this.yTool.get('activeTool')               // Get from Y.Map
    };
  }
  
  hash(state: DesignAppState): string { ... }                // IMPLEMENTS abstract method
  buildSnapshot(): DesignAppState { ... }                    // IMPLEMENTS abstract method
}

// WHY THIS PATTERN:
// ✓ All stores can be used interchangeably where Store<T> expected
// ✓ Each store provides its own data management
// ✓ Common behaviors (notify, subscribe) defined once
// ✓ TypeScript ensures all abstract methods are implemented
```

**Command interface**:

```typescript
// ============================================================
// EXAMPLE 3: FUNCTION INTERFACE FOR COMMAND PATTERN
// ============================================================
// Purpose: Show interface for functions (not just classes)
// Relates to: Commands have consistent signature for registry
// All commands follow same pattern: (context, ...args) => result
// Registry can store and execute any command uniformly
//
// js/semio/sketchpad/Sketchpad.tsx

// ------------------------------------------------------------
// FUNCTION INTERFACE: What all command handlers look like
// ------------------------------------------------------------
// This isn't a class interface - it's a function type
// Defines the "shape" all command functions must match

interface CommandHandler<TContext, TResult> {               // Generic over context and result
  (                                                          // It's a function type
    context: TContext,                                       // First param: execution context
    ...args: any[]                                           // Rest params: command arguments
  ): TResult | Promise<TResult>;                             // Returns result (sync or async)
}

// Example command that matches this interface:
const addPieceHandler: CommandHandler<DesignContext, PieceDiff> = 
  (context, typeGuid, position) => {                         // Matches interface signature
    const piece = createPiece(typeGuid, position);           // Create new piece
    return { added: [piece] };                               // Return diff
  };

// ------------------------------------------------------------
// REGISTRY: Store commands by name, execute uniformly
// ------------------------------------------------------------
// All commands stored in same Map regardless of specific types
// Enables dynamic command lookup and execution

const commandRegistry = new Map<                             // Map from name to handler
  string,                                                    // Key: command name string
  CommandHandler<any, any>                                   // Value: any command handler
>();

// Registration: Store command in registry
function registerCommand<TContext, TResult>(                 // Generic registration
  name: string,                                              // Command name
  handler: CommandHandler<TContext, TResult>                 // Handler function
): void {
  commandRegistry.set(name, handler);                        // Store in Map
}

// Usage: Register specific commands
registerCommand('semio.designApp.addPiece', addPieceHandler);   // Add piece
registerCommand('semio.designApp.deletePiece', deletePieceHandler); // Delete piece
registerCommand('semio.designApp.connect', connectHandler);     // Connect pieces

// ------------------------------------------------------------
// EXECUTION: Invoke any command by name
// ------------------------------------------------------------
// Uniform execution regardless of which command

function executeCommand(                                     // Execute by name
  name: string,                                              // Which command
  origin: string,                                            // Who's calling (for logging)
  ...args: any[]                                             // Command arguments
) {
  const handler = commandRegistry.get(name);                 // Look up handler
  if (!handler) {                                            // Command not found?
    throw new Error(`Unknown command: ${name}`);             // Error
  }
  
  const context = buildContext();                            // Build execution context
                                                              // (current state, stores, etc.)
  
  return handler(context, ...args);                          // Call handler with args
                                                              // All commands called same way
}

// USAGE: Execute commands uniformly
executeCommand('semio.designApp.addPiece', 'toolbar.addPiece', typeGuid, position);
executeCommand('semio.designApp.deletePiece', 'keyboard.delete', pieceGuids);
// Same pattern for all commands - interface enables uniformity
```

**TypeScript interface vs C# interface**:

```typescript
// ============================================================
// EXAMPLE 4: STRUCTURAL VS NOMINAL TYPING
// ============================================================
// Purpose: Show fundamental difference between TS and C# interfaces
// Relates to: TypeScript = "does it match the shape?"
//            C# = "did you explicitly say you implement it?"
// Same concept, different enforcement mechanisms
//
// Two different typing philosophies

// ------------------------------------------------------------
// TYPESCRIPT: Structural typing (shape matters, not name)
// ------------------------------------------------------------
// If an object HAS the right properties, it IS the type
// No explicit "implements" needed

interface Point {                                            // Define Point interface
  x: number;                                                 // Must have x: number
  y: number;                                                 // Must have y: number
  z: number;                                                 // Must have z: number
}

// This object automatically satisfies Point interface:
const p = {                                                  // Just create an object
  x: 0,                                                      // Has x: number ✓
  y: 0,                                                      // Has y: number ✓
  z: 0                                                       // Has z: number ✓
};                                                           // No "implements Point" needed!

// TypeScript sees: "Does p have x, y, z as numbers? Yes → it's a Point"
// This is called "duck typing" - if it walks like a duck...

function printPoint(point: Point) {                          // Takes Point parameter
  console.log(point.x, point.y, point.z);                    // Uses point properties
}

printPoint(p);                                               // Works! p matches Point shape
printPoint({ x: 1, y: 2, z: 3 });                            // Works! Inline object matches too

// ------------------------------------------------------------
// C#: Nominal typing (must explicitly implement)
// ------------------------------------------------------------
// MUST say "I implement this interface" - shape alone isn't enough

public interface IPoint {                                    // Define interface (I prefix by convention)
  double X { get; }                                          // Property getter
  double Y { get; }                                          // Property getter
  double Z { get; }                                          // Property getter
}

// This class explicitly says it implements IPoint:
public class Point3D : IPoint {                              // ": IPoint" = "I implement IPoint"
  public double X { get; set; }                              // Provides X property
  public double Y { get; set; }                              // Provides Y property
  public double Z { get; set; }                              // Provides Z property
}

// This would NOT work (even though shape matches):
// public class Vector3D {
//   public double X { get; set; }                           // Same shape as IPoint
//   public double Y { get; set; }
//   public double Z { get; set; }
// }
// void PrintPoint(IPoint p) { ... }
// PrintPoint(new Vector3D());                               // ERROR! Vector3D doesn't implement IPoint

// KEY DIFFERENCE:
// TypeScript: "Does it have x, y, z numbers?" → It's a Point
// C#: "Did you declare : IPoint?" → You must explicitly opt-in

// WHY IT MATTERS:
// TypeScript: More flexible, less ceremony, but less explicit
// C#: More explicit contracts, but requires more declarations
// Both work - just different philosophies
```

**Why interfaces matter for semio**

- Different file providers for different environments (browser, server, test)
- Stores share interface for consistent hook patterns
- Commands are interchangeable and discoverable

**What it enables**

- Swap implementations without changing consumers
- Test with mock implementations
- Plugin architecture for providers
- Type safety at boundaries

**What it limits**

- Interfaces add indirection (harder to navigate code)
- Wrong interface is hard to change
- Interface proliferation increases complexity
- Performance overhead in some languages
- Interfaces may not capture all requirements
- Design skill required to define good interfaces

---

#### 10.7 Inheritance: Reusing Structure

**Plain explanation**

Inheritance lets one class be based on another, getting all its properties and behaviors automatically. A "Dog" class can inherit from an "Animal" class, automatically gaining properties like weight and behaviors like eating, while adding dog-specific features like barking.

In semio, `KitDiffAppStore` inherits from `AppStore` which inherits from `Store`, each layer adding specialized functionality.

**Technical explanation**

**Store inheritance hierarchy in semio**:

```typescript
// ============================================================
// EXAMPLE 1: THREE-LEVEL INHERITANCE CHAIN
// ============================================================
// Purpose: Show how classes build on each other
// Relates to: Each level adds specialized functionality
// Store → AppStore → KitDiffAppStore → DesignAppStore
// Each inherits everything from parent, adds its own features
//
// js/semio/sketchpad/Sketchpad.tsx

// ------------------------------------------------------------
// LEVEL 1 (BASE): Store - any component with state
// ------------------------------------------------------------
// Most basic store - just holds state and notifies changes
// All other stores inherit from this

abstract class Store<TState> {                               // Generic over state type
                                                              // Abstract = can't create directly
  
  // ABSTRACT: Subclasses MUST implement these
  abstract snapshot(): TState;                               // Get current state
  abstract hash(state: TState): string;                      // Hash for caching
  abstract buildSnapshot(): TState;                          // Build state from Y.js
  
  // CONCRETE: All stores get these for free
  onChanged: Observable<void>;                               // Shallow change event
  onChangedDeep: Observable<void>;                           // Deep change event
  
  protected notifyChanged() {                                // Notify listeners
    this.onChanged.emit();                                   // Fire event
  }
}

// What Store provides:
// ✓ State management (snapshot)
// ✓ Cache invalidation (hash)
// ✓ Observable pattern (onChanged)
// ✗ No transactions, no undo/redo, no kit modification

// ------------------------------------------------------------
// LEVEL 2: AppStore - adds transactions and undo/redo
// ------------------------------------------------------------
// Extends Store, adds app-level features
// "extends Store" means it gets EVERYTHING from Store

abstract class AppStore<TState, TDiff, TEdit>                // More generics for diff/edit
  extends Store<TState>                                      // INHERITS from Store
{
  // NEW STATE: For transaction management
  protected undoStack: TEdit[] = [];                         // Past edits for undo
  protected redoStack: TEdit[] = [];                         // Undone edits for redo
  protected currentTransactionStack: TEdit[] = [];           // Current transaction
  
  // NEW METHODS: Transaction management
  startTransaction(): void {                                 // Begin a transaction
    this.isTransactionActive = true;                         // Track transaction state
    this.currentTransactionStack = [];                       // Start fresh
  }
  
  finalizeTransaction(): void {                              // Commit transaction
    const merged = this.mergeEdits(this.currentTransactionStack); // Combine edits
    this.undoStack.push(merged);                             // Add to undo stack
    this.redoStack = [];                                     // Clear redo
    this.isTransactionActive = false;                        // End transaction
  }
  
  abortTransaction(): void { ... }                           // Cancel transaction
  undo(): void { ... }                                       // Undo last edit
  redo(): void { ... }                                       // Redo undone edit
}

// What AppStore adds to Store:
// ✓ Everything from Store (inherited)
// ✓ Transaction management
// ✓ Undo/redo stacks
// ✗ No kit modification tracking

// ------------------------------------------------------------
// LEVEL 3: KitDiffAppStore - adds kit modification tracking
// ------------------------------------------------------------
// Extends AppStore, adds kit-specific features

abstract class KitDiffAppStore<TState, TDiff, TEdit>
  extends AppStore<TState, TDiff, TEdit>                     // INHERITS from AppStore
{
  // NEW ABSTRACT: Must provide kit access
  abstract kit(): KitStore;                                  // Get associated kit
  
  // OVERRIDE: Extend parent's applyEdit
  applyEdit(edit: KitDiffAppEdit): void {
    super.applyEdit(edit);                                   // Call parent's applyEdit
                                                              // (AppStore's edit handling)
    
    // ADDITIONAL: Apply kit changes too
    if (edit.kitDiff) {                                      // If edit has kit changes
      this.kit().change(edit.kitDiff);                       // Apply them to kit
    }
  }
}

// What KitDiffAppStore adds:
// ✓ Everything from AppStore (inherited)
// ✓ Everything from Store (inherited through AppStore)
// ✓ Kit modification tracking
// ✓ Kit diff application

// ------------------------------------------------------------
// LEVEL 4 (CONCRETE): DesignAppStore - specific implementation
// ------------------------------------------------------------
// Finally, a non-abstract class we can actually create

class DesignAppStore
  extends KitDiffAppStore<                                   // INHERITS from KitDiffAppStore
    DesignAppState,                                          // State type: design app state
    DesignAppDiff,                                           // Diff type: design app diffs
    DesignAppEdit                                            // Edit type: design app edits
  >
{
  private readonly designGuid: Guid;                         // Which design
  private readonly kitStore: KitStore;                       // Associated kit
  
  // IMPLEMENTS abstract method from KitDiffAppStore
  kit(): KitStore {
    return this.kitStore;                                    // Return the kit store
  }
  
  // IMPLEMENTS abstract method from Store
  snapshot(): DesignAppState {
    return this.buildSnapshot();                             // Build and return state
  }
  
  // Design-specific methods...
  selectPiece(guid: Guid) { ... }                            // Design-specific
  setCamera(camera: Camera) { ... }                          // Design-specific
}

// INHERITANCE CHAIN SUMMARY:
// DesignAppStore inherits:
// - From KitDiffAppStore: kit(), applyEdit with kit changes
// - From AppStore: undo/redo, transactions
// - From Store: snapshot, hash, observables
// Total: Gets features from 3 ancestor classes!
```

**Grasshopper component inheritance** (`Semio.Grasshopper.cs`):

```csharp
// ============================================================
// EXAMPLE 2: C# INHERITANCE WITH GENERICS
// ============================================================
// Purpose: Show inheritance in C# for Grasshopper components
// Relates to: Same pattern - base class with specialized children
// ModelComponent → IdComponent → TypeIdComponent
// Each level adds Grasshopper-specific functionality
//
// net/Semio.Grasshopper/Semio.Grasshopper.cs

// ------------------------------------------------------------
// LEVEL 1: Base class for all model components
// ------------------------------------------------------------
// Defines the template for all semio Grasshopper components
// Inherits from GH_Component (Grasshopper's base class)

public abstract class ModelComponent<TParam, TGoo, TModel>   // Three generic types
    : GH_Component                                           // Inherits from Grasshopper
    where TGoo : ModelGoo<TModel>                            // Constraint: TGoo must wrap TModel
{
    // ABSTRACT: Subclasses MUST implement these
    protected abstract void RegisterModelInputParams(        // Define input parameters
        GH_InputParamManager pManager                        // Grasshopper's input manager
    );
    
    protected abstract void RegisterModelOutputParams(       // Define output parameters
        GH_OutputParamManager pManager                       // Grasshopper's output manager
    );
    
    protected abstract void GetModelData(                    // Read input data into model
        IGH_DataAccess DA,                                   // Data access object
        TModel model                                         // Model to populate
    );
    
    protected abstract void SetModelData(                    // Write model data to outputs
        IGH_DataAccess DA,                                   // Data access object
        TModel model                                         // Model to output
    );
    
    // CONCRETE: Template method pattern
    protected override void SolveInstance(IGH_DataAccess DA) // Called by Grasshopper
    {
        var model = CreateModel();                           // Create model instance
        GetModelData(DA, model);                             // Read inputs (abstract)
        ProcessModel(model);                                 // Optional processing
        SetModelData(DA, model);                             // Write outputs (abstract)
    }
}

// What ModelComponent provides:
// ✓ Standard component structure
// ✓ Generic typing for any model type
// ✓ Template for input/output registration
// ✓ Template method for solving

// ------------------------------------------------------------
// LEVEL 2: Specialized for Id types (TypeId, DesignId, etc.)
// ------------------------------------------------------------
// Inherits from ModelComponent, adds Id-specific behavior

public abstract class IdComponent<TId>                       // Generic over Id type
    : ModelComponent<                                        // INHERITS from ModelComponent
        IdParam<TId>,                                        // Parameter type for Ids
        IdGoo<TId>,                                          // Goo wrapper for Ids
        TId                                                  // Model type is the Id itself
      >
    where TId : Id                                           // Constraint: TId must be an Id
{
    // Id-specific behavior shared by all Id components
    protected override void RegisterModelInputParams(        // IMPLEMENTS abstract method
        GH_InputParamManager pManager
    )
    {
        pManager.AddTextParameter(                           // All Ids have GUID input
            "Guid", "G", "Entity GUID",
            GH_ParamAccess.item
        );
    }
}

// What IdComponent adds:
// ✓ Everything from ModelComponent (inherited)
// ✓ Standard GUID input parameter
// ✓ Id-specific constraints

// ------------------------------------------------------------
// LEVEL 3: Concrete implementation for TypeId
// ------------------------------------------------------------
// Finally, a non-abstract class for Type IDs

public class TypeIdComponent                                 // Concrete class
    : IdComponent<TypeId>                                    // INHERITS from IdComponent<TypeId>
{
    public TypeIdComponent()                                 // Constructor
        : base("TypeId", "TId", "Creates a Type identifier") // Pass to base
    { }
    
    protected override void RegisterModelOutputParams(       // IMPLEMENTS from ModelComponent
        GH_OutputParamManager pManager
    )
    {
        pManager.AddParameter(                               // Add TypeId output
            new TypeIdParam(),                               // Custom parameter type
            "TypeId", "T", "The Type identifier",
            GH_ParamAccess.item
        );
    }
    
    // Other abstract methods implemented...
}

// INHERITANCE CHAIN:
// TypeIdComponent → IdComponent<TypeId> → ModelComponent<...> → GH_Component
// Each level adds specific functionality
```

**When semio uses inheritance**:

| Parent | Child | Why Inheritance |
|--------|-------|-----------------|
| `Store` | `AppStore` | Add transaction support |
| `AppStore` | `KitDiffAppStore` | Add kit modification |
| `KitDiffAppStore` | `DesignAppStore` | Specialize for design |
| `GH_Component` | `ModelComponent<>` | Grasshopper framework |
| `ModelComponent<>` | `TypeComponent` | Specialize for Type |

**Why semio uses inheritance sparingly**

- Clear "is-a" relationships: DesignAppStore IS-A AppStore
- Framework requirements (Grasshopper)
- Behavior reuse across stores

**What it enables**

- Shared transaction/undo logic across stores
- Grasshopper component consistency
- Polymorphism: treat all stores uniformly

**What it limits**

- Deep hierarchies are avoided (max 3 levels)
- Prefer composition for most cases
- Base class changes affect all children

---

#### 10.8 Composition: Building from Parts

**Plain explanation**

Composition means building complex objects by combining simpler objects, rather than inheriting from a parent. Instead of saying "a Car is-a Vehicle," you say "a Car has-a Engine, has-a Transmission, has-a set of Wheels."

semio is fundamentally about composition: a Design HAS Pieces, a Piece HAS a reference to a Type. The whole kit-of-parts concept is composition.

**Technical explanation**

**Composition in semio domain**:

```typescript
// ============================================================
// EXAMPLE 1: DOMAIN MODEL COMPOSITION
// ============================================================
// Purpose: Show "has-a" relationships in semio's domain
// Relates to: Composition = building complex from simple parts
// Kit HAS Types, Type HAS Connectors, Design HAS Pieces
// This is the foundation of "kit-of-parts" architecture
//
// js/semio/semio.ts

// ------------------------------------------------------------
// KIT: Composed of multiple collections
// ------------------------------------------------------------
// A Kit doesn't inherit from anything - it CONTAINS things
// This is pure composition

interface Kit {                                              // Kit definition
  // Composition: Kit HAS these collections
  types: Type[];                                             // Kit contains types
                                                              // (like a box of LEGO piece kinds)
  
  designs: Design[];                                         // Kit contains designs
                                                              // (like instruction booklets)
  
  qualities: Quality[];                                      // Kit contains quality definitions
                                                              // (like measurement standards)
  
  files: File[];                                             // Kit contains file references
                                                              // (like 3D model files)
  
  authors: Author[];                                         // Kit contains author info
                                                              // (like credits)
}

// ------------------------------------------------------------
// TYPE: Composed of connectors and models
// ------------------------------------------------------------
// Type doesn't inherit - it CONTAINS things

interface Type {                                             // Type definition
  connectors: Connector[];                                   // Type HAS connectors
                                                              // (like LEGO studs and tubes)
  
  models: Model[];                                           // Type HAS 3D models
                                                              // (like different visual representations)
  
  props: Prop[];                                             // Type HAS properties
                                                              // (like measurements)
}

// ------------------------------------------------------------
// DESIGN: Composed of pieces and connections
// ------------------------------------------------------------
// Design is a graph of pieces connected together

interface Design {                                           // Design definition
  pieces: Piece[];                                           // Design HAS pieces
                                                              // (instances of types)
  
  connections: Connection[];                                 // Design HAS connections
                                                              // (how pieces link together)
  
  layers: Layer[];                                           // Design HAS layers
                                                              // (organizational groups)
  
  groups: Group[];                                           // Design HAS groups
                                                              // (semantic clusters)
}

// ------------------------------------------------------------
// PIECE: Composed via reference
// ------------------------------------------------------------
// Piece doesn't CONTAIN a Type, it REFERENCES one
// This is composition via reference (saves memory)

interface Piece {                                            // Piece definition
  type: TypeId;                                              // Piece HAS-A reference to Type
                                                              // TypeId is just a GUID string
                                                              // The actual Type lives in kit.types
                                                              // This avoids duplicating Type data
  
  plane?: Plane;                                             // Piece HAS-A position (if fixed)
  center?: Point;                                            // Piece HAS-A center point
}

// COMPOSITION SUMMARY:
// Kit ──HAS→ Type[] ──HAS→ Connector[], Model[]
//      └─HAS→ Design[] ──HAS→ Piece[] ──REFS→ Type
//                       └─HAS→ Connection[]
//
// Everything is built from smaller parts
// This is the essence of "kit-of-parts" architecture
```

**UI composition** (React component composition):

```tsx
// ============================================================
// EXAMPLE 2: REACT COMPONENT COMPOSITION
// ============================================================
// Purpose: Show UI built from composed components
// Relates to: React is fundamentally compositional
// Sketchpad HAS Canvas, Navbar, Footer, Panels
// Each component can be developed and tested independently
//
// js/semio/sketchpad/Sketchpad.tsx

// ------------------------------------------------------------
// SKETCHPAD: Composed of multiple UI parts
// ------------------------------------------------------------
// Not inheritance - Sketchpad CONTAINS these components

function Sketchpad({                                         // Main app component
  remoteProviders                                            // Configuration prop
}: SketchpadProps) {
  return (
    <Providers>                                              {/* Wrapper for contexts */}
      
      {/* COMPOSITION: Sketchpad HAS a Canvas */}
      <Canvas>                                               {/* Main content area */}
        <WindowLayout />                                     {/* Contains windows */}
      </Canvas>
      
      {/* COMPOSITION: Sketchpad HAS a Navbar */}
      <Navbar items={navItems} />                            {/* Top navigation */}
                                                              {/* Navbar doesn't inherit Sketchpad */}
                                                              {/* It's a separate component */}
      
      {/* COMPOSITION: Sketchpad HAS a Footer */}
      <Footer items={footerItems} />                         {/* Bottom bar */}
      
      {/* COMPOSITION: Sketchpad HAS Panels */}
      <Panels>                                               {/* Side panels */}
        <WorkbenchPanel />                                   {/* Left panel */}
        <DetailsPanel />                                     {/* Right panel */}
        <SettingsPanel />                                    {/* Settings panel */}
      </Panels>
      
    </Providers>
  );
}

// Why composition instead of inheritance:
// ✓ Canvas can be tested independently
// ✓ Navbar can be reused in other apps
// ✓ Panels can be swapped or hidden
// ✓ Layout can be rearranged by changing composition

// ------------------------------------------------------------
// WINDOW: Composed of different view types
// ------------------------------------------------------------
// Window CONTAINS one of several possible views
// This is compositional - not "Window extends Scene3D"

function Window({ kind }: WindowProps) {                     // Window component
  return (
    <WindowFrame>                                            {/* Common frame UI */}
      
      {/* CONDITIONAL COMPOSITION: Show one view type */}
      {kind === 'scene' && <Scene3D />}                      {/* 3D view */}
      {kind === 'diagram' && <Diagram2D />}                  {/* 2D diagram */}
      {kind === 'table' && <Table />}                        {/* Table view */}
      
      {/* Each view is a separate component */}
      {/* Window doesn't inherit from any of them */}
      {/* It just CONTAINS the appropriate one */}
      
    </WindowFrame>
  );
}

// COMPOSITION HIERARCHY:
// Sketchpad
//   └── Canvas
//       └── WindowLayout
//           └── Window
//               └── Scene3D | Diagram2D | Table
//   └── Navbar
//   └── Footer
//   └── Panels
//       └── WorkbenchPanel, DetailsPanel, SettingsPanel
```

**Provider composition** (React Context):

```tsx
// ============================================================
// EXAMPLE 3: PROVIDER COMPOSITION (CONTEXT LAYERING)
// ============================================================
// Purpose: Show how contexts are composed as nested layers
// Relates to: Each provider adds one capability to the tree
// ThemeProvider + I18nProvider + XStateProvider = full context
// Child components can use ANY of these contexts
//
// js/semio/sketchpad/Sketchpad.tsx

// ------------------------------------------------------------
// COMPOSED PROVIDERS: Stack of contexts
// ------------------------------------------------------------
// Each provider adds one capability
// Nesting composes them together

function Providers({                                         // Provider wrapper component
  children                                                   // Components that need context
}: {
  children: ReactNode
}) {
  return (
    // LAYER 1: Theme context (light/dark mode)
    <ThemeProvider>                                          {/* Provides theme state */}
                                                              {/* Children can useTheme() */}
      
      {/* LAYER 2: Internationalization context */}
      <I18nProvider>                                         {/* Provides translations */}
                                                              {/* Children can useTranslation() */}
        
        {/* LAYER 3: XState actor context */}
        <XStateProvider actor={actor}>                       {/* Provides state machine */}
                                                              {/* Children can useActor() */}
          
          {/* LAYER 4: UI level context */}
          <LevelProvider level="base">                       {/* Provides UI level */}
                                                              {/* Children can useLevel() */}
            
            {/* LAYER 5: Transaction context */}
            <TransactionProvider>                            {/* Provides transactions */}
                                                              {/* Children can useTransaction() */}
              
              {children}                                     {/* Actual content */}
              
            </TransactionProvider>
          </LevelProvider>
        </XStateProvider>
      </I18nProvider>
    </ThemeProvider>
  );
}

// HOW COMPOSITION WORKS:
// Any component inside this tree can access:
// - Theme (from ThemeProvider)
// - Translations (from I18nProvider)
// - XState actor (from XStateProvider)
// - UI level (from LevelProvider)
// - Transactions (from TransactionProvider)
//
// Components don't need to know about all providers
// They just useContext() for what they need

// EXAMPLE CONSUMER:
function SomeButton() {
  const { theme } = useTheme();                              // From ThemeProvider
  const { t } = useTranslation();                            // From I18nProvider
  const actor = useSketchpadActor();                         // From XStateProvider
  const transaction = useTransaction();                      // From TransactionProvider
  
  // Has access to all composed contexts!
  return (
    <button onClick={() => actor.send({ type: 'CLICK' })}>
      {t('button.label')}
    </button>
  );
}

// COMPOSITION BENEFIT:
// ✓ Each provider has single responsibility
// ✓ Providers can be reused in other apps
// ✓ Easy to add/remove capabilities
// ✓ Clear dependencies (what's in the tree)
```

**Composition vs Inheritance in semio**:

| Relationship | Approach | Example |
|--------------|----------|---------|
| Kit contains Types | Composition | `kit.types: Type[]` |
| DesignAppStore needs transactions | Inheritance | `extends AppStore` |
| Sketchpad shows panels | Composition | `<Panels><WorkbenchPanel /></Panels>` |
| TypeComponent is GH_Component | Inheritance | `extends ModelComponent<>` |

**Why semio prefers composition**

- Domain model is naturally compositional (kit-of-PARTS)
- React components are designed for composition
- Flexible arrangements of pieces/connections
- Types can be reused across designs

**What it enables**

- Reuse types across multiple designs
- Swap models within types
- Configure kits from smaller parts
- React component flexibility

**What it limits**

- References (GUIDs) add indirection
- Validation needed (does referenced type exist?)
- Serialization more complex (nested vs flat)

---

#### 10.9 Design Patterns: Proven Solutions

**Plain explanation**

Design patterns are named solutions to common problems. Instead of inventing a new approach every time you need to create objects flexibly, you use the "Factory" pattern. Instead of inventing how to notify many objects of a change, you use the "Observer" pattern.

semio uses many patterns: Observer for Y.js changes, Factory for store creation, Command for undo/redo.

**Technical explanation**

**Patterns used in semio**:

**Observer pattern** (Y.js subscriptions):

```typescript
// ============================================================
// EXAMPLE 1: OBSERVER PATTERN - NOTIFY ON CHANGE
// ============================================================
// Purpose: Show how observers get notified when data changes
// Relates to: Multiple components watch one data source
// When Kit changes, Navbar, Canvas, Diagram all update
// Each observer subscribes and gets called on change
//
// js/semio/sketchpad/Sketchpad.tsx

// ------------------------------------------------------------
// THE SUBJECT: Store that observers watch
// ------------------------------------------------------------
// Store manages a list of observers (callbacks)
// When data changes, it notifies all of them

class Store<TState> {                                        // Generic store base
  private observers: Set<() => void> = new Set();           // Set of callback functions
                                                              // Using Set prevents duplicates
  
  subscribe(callback: () => void): () => void {              // Add an observer
    this.observers.add(callback);                            // Store the callback
    
    return () => this.observers.delete(callback);            // Return unsubscribe function
                                                              // Caller keeps this to clean up
  }
  
  protected notify(): void {                                 // Notify all observers
    for (const observer of this.observers) {                 // Loop through each
      observer();                                            // Call the callback
                                                              // Observer does whatever it wants
    }
  }
  
  // When data changes internally:
  protected onChange(newState: TState) {
    this.cachedState = newState;                             // Update cached state
    this.notify();                                           // Tell all observers
  }
}

// ------------------------------------------------------------
// THE OBSERVER: React hook subscribing to changes
// ------------------------------------------------------------
// React components use this to watch store changes
// Component re-renders when store notifies

function useSync<T>(                                         // Generic hook
  store: Store<T>,                                           // Which store to watch
  selector: (s: T) => T = identitySelector                   // Optional: pick specific data
): T {
  return useSyncExternalStore(                               // React's built-in hook
    (onStoreChange) => store.subscribe(onStoreChange),       // Subscribe to store
                                                              // React passes onStoreChange callback
                                                              // We subscribe it to our store
    
    () => selector(store.snapshot())                         // Get current value
                                                              // Called on subscribe and on change
  );
}

// USAGE: Component observes kit changes
function TypesList() {
  const types = useSync(kitStore, kit => kit.types);         // Watch types array
  // When kitStore.types changes, this component re-renders
  return <ul>{types.map(t => <li>{t.name}</li>)}</ul>;
}
```

**Command pattern** (undo/redo):

```typescript
// ============================================================
// EXAMPLE 2: COMMAND PATTERN - ENCAPSULATED OPERATIONS
// ============================================================
// Purpose: Show how commands enable undo/redo
// Relates to: Each action is an object with do AND undo
// Adding a piece stores how to add AND how to remove it
// This enables reversing any operation
//
// js/semio/sketchpad/Sketchpad.tsx

// ------------------------------------------------------------
// COMMAND STRUCTURE: Do and Undo paired together
// ------------------------------------------------------------
// Every edit stores both directions
// "Forward" is do, "backward" is undo

interface AppEdit<TSelectionDiff> {                          // Edit object structure
  do: AppStep<TSelectionDiff>;                               // Forward operation
                                                              // What to apply to move forward
  
  undo: AppStep<TSelectionDiff>;                             // Backward operation
                                                              // What to apply to move backward
}

// Example: Adding a piece
// do: { pieces: { added: [newPiece] } }                     // Add the piece
// undo: { pieces: { removed: [newPiece.guid] } }            // Remove the piece
// Now we can go forward (add) or backward (remove)!

// ------------------------------------------------------------
// COMMAND REGISTRATION: Named commands in registry
// ------------------------------------------------------------
// Commands are registered by name so they can be called dynamically

registerCommand(                                             // Register a command
  "semio.designApp.addPiece",                                // Command name
  (ctx, args) => {                                           // Handler function
    
    const piece = createPiece(args);                         // Create the new piece
    
    // Return the diff (what changed)
    return {
      kitDiff: {                                             // Changes to kit data
        designs: {                                           // Changes to designs
          updated: [{                                        // Which design updated
            diff: {                                          // What changed in design
              pieces: {                                      // Changes to pieces
                added: [piece]                               // Piece was added
              }
            }
          }]
        }
      }
    };
    // The inverse (undo) diff is calculated automatically
    // from this forward diff
  }
);

// ------------------------------------------------------------
// COMMAND EXECUTION: Invoke by name
// ------------------------------------------------------------
// Any code can execute any command by name
// Enables dynamic command invocation (toolbars, hotkeys, etc.)

executeCommand(                                              // Execute a command
  "semio.designApp.addPiece",                                // Which command
  origin,                                                    // Who called (for logging)
  {                                                          // Arguments
    typeGuid,                                                // What type of piece
    center                                                   // Where to place it
  }
);

// UNDO/REDO USAGE:
// User adds piece → command creates edit with do/undo
// User hits Ctrl+Z → apply undo step (remove piece)
// User hits Ctrl+Shift+Z → apply do step (add piece back)
```

**Factory pattern** (store creation):

```typescript
// ============================================================
// EXAMPLE 3: FACTORY PATTERN - DEFERRED CREATION
// ============================================================
// Purpose: Show how factories delay object creation
// Relates to: Create objects only when needed, with parameters
// Don't create DesignAppStore until user navigates to design
// Factory knows HOW to create, caller decides WHEN
//
// js/semio/sketchpad/shared.ts

// ------------------------------------------------------------
// FACTORY REGISTRATION: Store the "recipe"
// ------------------------------------------------------------
// We don't create stores immediately
// We register a FUNCTION that creates them

let designAppStoreFactory: ((kit: KitStore, designGuid: Guid) => DesignAppStore) | null = null;

function registerDesignAppStoreFactory(                      // Register the factory
  factory: (kit: KitStore, designGuid: Guid) => DesignAppStore // Factory is a function
): void {
  designAppStoreFactory = factory;                           // Store the factory
                                                              // Not calling it yet!
}

// Factory function is typically defined in Design.tsx:
registerDesignAppStoreFactory((kit: KitStore, designGuid: Guid) => {
  return new DesignAppStore(kit, designGuid);                // Creates the store
});

// ------------------------------------------------------------
// FACTORY USAGE: Create when needed
// ------------------------------------------------------------
// When user navigates to /kit/{kitId}/design/{designId}
// THEN we create the store using the factory

function getDesignAppStoreFactory(): typeof designAppStoreFactory {
  return designAppStoreFactory;                              // Return the factory function
}

// Usage in routing/navigation:
function navigateToDesign(kitStore: KitStore, designGuid: Guid) {
  const factory = getDesignAppStoreFactory();                // Get the factory
  
  if (!factory) {                                            // Factory not registered?
    throw new Error("DesignAppStore factory not registered"); // Error
  }
  
  const store = factory(kitStore, designGuid);               // NOW create the store
                                                              // Factory called with parameters
                                                              // Returns new DesignAppStore
  
  return store;
}

// WHY FACTORY PATTERN:
// ✓ Don't create stores for designs user hasn't visited
// ✓ Can inject different factories for testing
// ✓ Avoids circular dependencies (Design.tsx registers factory)
// ✓ Parameters provided at creation time, not registration time
```

**Strategy pattern** (file providers):

```typescript
// ============================================================
// EXAMPLE 4: STRATEGY PATTERN - SWAPPABLE ALGORITHMS
// ============================================================
// Purpose: Show how different implementations are swapped
// Relates to: Same interface, different behavior based on context
// Browser uses RemoteFileProvider, Electron uses LocalFileProvider
// Code that uses files doesn't know which strategy is active
//
// js/semio/sketchpad/Sketchpad.tsx

// ------------------------------------------------------------
// THE INTERFACE: What all strategies must provide
// ------------------------------------------------------------
// All file providers implement this interface
// Consumer code works with interface, not specific class

interface FileProvider {                                     // Strategy interface
  upload: (                                                  // Upload capability
    kitId: string, fileId: string, path: string, blob: Blob
  ) => Promise<string>;
  
  download: (                                                // Download capability
    kitId: string, fileId: string, path: string
  ) => Promise<Blob>;
  
  delete: (                                                  // Delete capability
    kitId: string, fileId: string, path: string
  ) => Promise<void>;
  
  getUrl: (                                                  // Get URL capability
    kitId: string, fileId: string, path: string
  ) => string;
}

// STRATEGY IMPLEMENTATIONS:
// class MemoryFileProvider implements FileProvider { ... }  // In-memory (testing)
// class LocalFileProvider implements FileProvider { ... }   // IndexedDB (offline)
// class RemoteFileProvider implements FileProvider { ... }  // HTTP (production)

// ------------------------------------------------------------
// STRATEGY SELECTION: Choose at runtime
// ------------------------------------------------------------
// Based on environment, select the appropriate strategy
// The rest of the code doesn't know which one is used

function createFileProvider(): FileProvider {               // Factory for strategy selection
  
  if (isElectron) {                                         // Running in Electron desktop app?
    return new LocalFileProvider();                         // Use local file system
                                                             // Files stored on disk
  }
  
  if (isTesting) {                                          // Running in test environment?
    return new MemoryFileProvider();                        // Use in-memory storage
                                                             // Fast, no side effects
  }
  
  return new RemoteFileProvider();                          // Default: use cloud storage
                                                             // Files on server
}

// Inject strategy based on environment
const provider = createFileProvider();                       // Get appropriate strategy

// ------------------------------------------------------------
// STRATEGY USAGE: Consumer doesn't know which strategy
// ------------------------------------------------------------
// This code works with ANY file provider
// Doesn't know if files go to memory, disk, or cloud

async function uploadModelFile(file: File) {
  const url = await provider.upload(                         // Use the strategy
    currentKitId,
    generateGuid(),
    file.name,
    file
  );
  // Doesn't know if:
  // - File went to memory Map
  // - File went to IndexedDB
  // - File went to cloud server
  // Strategy handles the details!
  
  return url;
}
```

**Registry pattern** (plugin architecture):

```typescript
// ============================================================
// EXAMPLE 5: REGISTRY PATTERN - DYNAMIC REGISTRATION
// ============================================================
// Purpose: Show how plugins register themselves dynamically
// Relates to: Open for extension, closed for modification
// New apps register without modifying Sketchpad.tsx
// New event handlers register without modifying the machine
//
// js/semio/sketchpad/shared.ts

// ------------------------------------------------------------
// APP PLUGIN REGISTRY: Apps register themselves
// ------------------------------------------------------------
// Map stores registered plugins by ID
// Plugins register when their module loads

const appPlugins = new Map<string, AppPlugin>();             // Registry storage
                                                              // Key: plugin ID (e.g., "design")
                                                              // Value: plugin object

function registerAppPlugin(plugin: AppPlugin): void {        // Registration function
  appPlugins.set(plugin.id, plugin);                         // Add to registry
  console.log(`Registered app plugin: ${plugin.id}`);        // Log for debugging
}

// USAGE: Each app registers itself on module load
// In Design.tsx:
registerAppPlugin({                                          // Design app registration
  id: "design",                                              // Plugin ID
  namespace: "DESIGN",                                       // Event prefix
  machine: { ... },                                          // XState contributions
  createDefaultState: () => ({ selection: { pieces: [] } })  // State factory
});

// In Type.tsx:
registerAppPlugin({                                          // Type app registration
  id: "type",
  namespace: "TYPE",
  machine: { ... },
  createDefaultState: () => ({ selection: { connectors: [] } })
});

// Sketchpad.tsx NEVER MODIFIED to add new apps!
// Just add a new XxxApp.tsx file that registers itself

// ------------------------------------------------------------
// EVENT HANDLER REGISTRY: Handlers register themselves
// ------------------------------------------------------------
// Similar pattern for event handlers

const eventHandlers = new Map<string, EventHandler>();       // Registry storage
                                                              // Key: event type (e.g., "DESIGN.SELECT_PIECE")
                                                              // Value: handler function

function registerEventHandler(                               // Registration function
  eventType: string,                                         // Which event
  handler: EventHandler                                      // How to handle it
): void {
  eventHandlers.set(eventType, handler);                     // Add to registry
}

// USAGE: Components register handlers
registerEventHandler("DESIGN.SELECT_PIECE", {                // Register handler
  action: (context, event) => ({                             // Handler logic
    designApp: {
      ...context.designApp,
      selection: { pieces: [event.pieceGuid] }
    }
  })
});

// GETTING FROM REGISTRY:
function getAppPlugins(): Map<string, AppPlugin> {           // Get all plugins
  return appPlugins;                                         // Return the registry
}

function getEventHandler(eventType: string): EventHandler {  // Get specific handler
  return eventHandlers.get(eventType);                       // Look up in registry
}

// WHY REGISTRY PATTERN:
// ✓ Decouples registration from usage
// ✓ Enables plugin architecture (open/closed)
// ✓ Self-registering modules
// ✓ Dynamic discovery of capabilities
```

**Why patterns in semio**

- Observer: React/Y.js integration is observer-based
- Command: Undo/redo requires encapsulated actions
- Factory: Deferred store creation for route-based apps
- Strategy: Environment-specific implementations

**What it enables**

- Proven solutions to recurring problems
- Shared vocabulary ("use the Observer pattern")
- Flexible, extensible architecture
- Testability through dependency injection

**What it limits**

- Pattern overhead for simple cases
- Learning curve for newcomers
- Over-engineering risk

---

#### 10.10 SOLID Principles: Guidelines for Good Design

**Plain explanation**

SOLID is an acronym for five principles that guide good object-oriented design. They're not rigid rules but guidelines that, when followed, tend to produce code that's easier to understand, modify, and test.

semio follows SOLID: single-responsibility modules, open/closed plugin architecture, interface segregation.

**Technical explanation**

**S – Single Responsibility Principle in semio**:

```typescript
// ============================================================
// EXAMPLE 1: SINGLE RESPONSIBILITY - ONE JOB PER MODULE
// ============================================================
// Purpose: Show how each module has exactly one reason to change
// Relates to: If something changes, only ONE module needs to update
// Domain logic changes → only semio.ts changes
// UI routing changes → only Sketchpad.tsx changes
//
// semio monorepo organization

// ------------------------------------------------------------
// EACH MODULE HAS ONE JOB:
// ------------------------------------------------------------

// semio.ts: Domain logic ONLY
// - Kit, Type, Design, Piece interfaces
// - Diff operations (getKitDiff, applyKitDiff)
// - Validation (validateKit, areConnectorsCompatible)
// - Geometry calculations (computePiecePlane)
// WHY: If domain rules change, only this file changes
// NO: React components, database code, network code

// Sketchpad.tsx: App orchestration ONLY
// - XState machine setup
// - Provider composition
// - Routing between apps
// - Store initialization
// WHY: If app structure changes, only this file changes
// NO: Domain logic, specific app UI, validation rules

// Design.tsx: Design editing ONLY
// - Design app state (selection, hover, camera)
// - Design app commands (addPiece, deletePiece)
// - Design app components (Canvas, Diagram, Toolbar)
// WHY: If design editing changes, only this file changes
// NO: Type editing, kit management, home screen

// KitStore: Kit persistence ONLY
// - Y.js document management
// - Snapshot building
// - Change observation
// WHY: If storage mechanism changes, only this file changes
// NO: UI code, validation, domain logic

// ------------------------------------------------------------
// ANTI-PATTERN: What we avoid
// ------------------------------------------------------------

// ❌ BAD: Design.tsx handling database writes
// Design.tsx should use KitStore, not write to Y.js directly

// ❌ BAD: semio.ts containing React components
// Domain logic should be pure - no UI framework code

// ❌ BAD: KitStore validating business rules
// Storage layer should just store - domain validates

// BENEFIT: When validation rules change:
// ✓ Only semio.ts changes
// ✓ Sketchpad.tsx unchanged
// ✓ Design.tsx unchanged
// ✓ KitStore unchanged
```

**O – Open/Closed Principle in semio**:

```typescript
// ============================================================
// EXAMPLE 2: OPEN/CLOSED - EXTEND WITHOUT MODIFYING
// ============================================================
// Purpose: Show how to add features without changing existing code
// Relates to: Code is "closed" for modification, "open" for extension
// Add new app → create file, register plugin, done!
// Don't need to edit Sketchpad.tsx at all
//
// js/semio/sketchpad/shared.ts + app files

// ------------------------------------------------------------
// ADDING A NEW APP: Extension only
// ------------------------------------------------------------
// To add a new app called "MyApp":

// STEP 1: Create MyApp.tsx
// This is the ONLY file you create or modify

// STEP 2: Register the plugin (in MyApp.tsx)
registerAppPlugin({                                          // Register new plugin
  id: "myapp",                                               // Unique ID
  namespace: "MYAPP",                                        // Event prefix
  machine: {                                                 // XState contributions
    actions: { ... },                                        // Custom actions
    guards: { ... },                                         // Custom guards
    eventHandlers: { ... }                                   // Custom event handlers
  },
  createDefaultState: () => ({                               // Initial state factory
    selection: [],                                           // App-specific state
    panelVisibility: { details: true }
  })
});

// STEP 3: Register event handlers (in MyApp.tsx)
registerEventHandler("MYAPP.DO_THING", {                     // Handle custom events
  action: (context, event) => ({
    myApp: { ...context.myApp, ...event.changes }
  })
});

// THAT'S IT! Sketchpad.tsx is NEVER MODIFIED

// ------------------------------------------------------------
// WHAT'S "CLOSED": Sketchpad.tsx core code
// ------------------------------------------------------------
// This code doesn't change when apps are added:

// In Sketchpad.tsx:
const plugins = getAppPlugins();                             // Get all registered plugins
                                                              // Dynamically discovers new apps
                                                              // No hardcoded app list

for (const [id, plugin] of plugins) {                        // Process each plugin
  machine.registerActions(plugin.machine.actions);           // Add its actions
  machine.registerGuards(plugin.machine.guards);             // Add its guards
}
// New apps automatically included without code change

// ------------------------------------------------------------
// WHAT'S "OPEN": Plugin registration
// ------------------------------------------------------------
// Any new code can register plugins:
// - Create new file
// - Call registerAppPlugin()
// - Done!

// BENEFIT:
// ✓ Sketchpad.tsx stable (doesn't change)
// ✓ New features via new files
// ✓ Parallel development (different teams, different apps)
// ✓ No merge conflicts in Sketchpad.tsx
```

**L – Liskov Substitution in semio**:

```typescript
// ============================================================
// EXAMPLE 3: LISKOV SUBSTITUTION - SUBTYPES INTERCHANGEABLE
// ============================================================
// Purpose: Show how subclasses can replace parent classes
// Relates to: If A extends B, A can be used anywhere B is expected
// MemoryFileProvider can replace FileProvider anywhere
// Code works correctly regardless of which subtype is used
//
// js/semio/sketchpad/Sketchpad.tsx

// ------------------------------------------------------------
// THE FUNCTION: Expects abstract FileProvider
// ------------------------------------------------------------
// This function accepts ANY FileProvider
// Doesn't matter which concrete class is passed

function uploadModel(                                        // Upload function
  file: File,                                                // File to upload
  provider: FileProvider                                     // ANY FileProvider works
): Promise<string> {
  return provider.upload(                                    // Call upload method
    kitId,                                                   // Kit identifier
    fileId,                                                  // File identifier
    file.name,                                               // File name
    file                                                     // File blob
  );
}

// ------------------------------------------------------------
// SUBSTITUTION: Any subtype works correctly
// ------------------------------------------------------------
// All three calls work correctly
// The function doesn't know which implementation is used
// That's Liskov Substitution!

uploadModel(file, new MemoryFileProvider());                 // ✓ Works: uses memory
                                                              // Returns memory:// URL
                                                              // File stored in Map

uploadModel(file, new LocalFileProvider());                  // ✓ Works: uses IndexedDB
                                                              // Returns indexeddb:// URL
                                                              // File stored in browser

uploadModel(file, new RemoteFileProvider());                 // ✓ Works: uses HTTP
                                                              // Returns https:// URL
                                                              // File stored on server

// KEY INSIGHT:
// The function has NO special cases for different providers
// No "if (provider instanceof MemoryFileProvider)" checks
// All providers behave correctly for the same interface

// VIOLATION EXAMPLE (what to avoid):
// If LocalFileProvider.upload() threw an error for files > 10MB
// but MemoryFileProvider.upload() accepted any size,
// that would VIOLATE Liskov Substitution
// Callers couldn't safely substitute one for the other
```

**I – Interface Segregation in semio**:

```typescript
// ============================================================
// EXAMPLE 4: INTERFACE SEGREGATION - SMALL FOCUSED INTERFACES
// ============================================================
// Purpose: Show preference for many small interfaces over one big one
// Relates to: Clients shouldn't depend on methods they don't use
// FileProvider has 4 methods - just what file operations need
// Not a giant IKitService with 50 unrelated methods
//
// js/semio/sketchpad/Sketchpad.tsx

// ------------------------------------------------------------
// GOOD: Small, focused interface
// ------------------------------------------------------------
// FileProvider has ONLY file-related methods
// Each method is about file operations

interface FileProvider {                                     // Focused interface
  upload: (                                                  // Upload files
    kitId: string, fileId: string, path: string, blob: Blob
  ) => Promise<string>;
  
  download: (                                                // Download files
    kitId: string, fileId: string, path: string
  ) => Promise<Blob>;
  
  delete: (                                                  // Delete files
    kitId: string, fileId: string, path: string
  ) => Promise<void>;
  
  getUrl: (                                                  // Get file URLs
    kitId: string, fileId: string, path: string
  ) => string;
}

// A component that only needs downloads:
function ModelViewer({ provider }: { provider: FileProvider }) {
  // Only uses download - but must accept whole interface
  // That's OK because interface is small
  const model = await provider.download(kitId, fileId, path);
}

// ------------------------------------------------------------
// BAD: Giant interface with everything (what we avoid)
// ------------------------------------------------------------
// ❌ Don't do this:

interface IKitService {                                      // Too many responsibilities!
  // File operations
  uploadFile: (...) => Promise<string>;
  downloadFile: (...) => Promise<Blob>;
  deleteFile: (...) => Promise<void>;
  
  // Kit CRUD
  createKit: (...) => Promise<Kit>;
  loadKit: (...) => Promise<Kit>;
  saveKit: (...) => Promise<void>;
  
  // Validation
  validateKit: (...) => ValidationResult;
  validateType: (...) => ValidationResult;
  
  // Collaboration
  connectToRoom: (...) => Promise<void>;
  sendUpdate: (...) => void;
  
  // ... 40 more methods
}

// PROBLEMS WITH GIANT INTERFACE:
// ✗ ModelViewer depends on 50 methods to use 1
// ✗ Hard to mock in tests (must implement everything)
// ✗ Changes to collaboration break file components
// ✗ Single Responsibility violated

// ------------------------------------------------------------
// GOOD: Multiple small interfaces
// ------------------------------------------------------------
// Each interface has one purpose

interface FileProvider { ... }                               // File operations only
interface KitRepository { ... }                              // Kit CRUD only
interface Validator { ... }                                  // Validation only
interface CollaborationProvider { ... }                      // Real-time sync only

// Components depend ONLY on what they need
function ModelViewer({ files }: { files: FileProvider }) { ... }
function KitList({ repo }: { repo: KitRepository }) { ... }
```

**D – Dependency Inversion in semio**:

```typescript
// ============================================================
// EXAMPLE 5: DEPENDENCY INVERSION - DEPEND ON ABSTRACTIONS
// ============================================================
// Purpose: High-level modules shouldn't depend on low-level details
// Relates to: Depend on abstractions (interfaces), not implementations
// Sketchpad depends on RemoteProviders interface, not WebSocket/HTTP
//
// js/semio/sketchpad/Sketchpad.tsx

// ------------------------------------------------------------
// THE ABSTRACTION (what high-level code depends on)
// ------------------------------------------------------------

interface RemoteProviders {                                  // Abstract interface
  yProvider?: (guid: Guid) => YProvider;                     // Sync provider (abstract)
  fileProvider?: FileProvider;                               // File provider (abstract)
}

// Sketchpad is HIGH-LEVEL policy
// RemoteProviders is ABSTRACTION
// WebSocket, HTTP, IndexedDB are LOW-LEVEL details

// ------------------------------------------------------------
// HIGH-LEVEL MODULE (depends on abstraction)
// ------------------------------------------------------------

interface SketchpadProps {                                   // Component props
  remoteProviders?: RemoteProviders;                         // Depends on ABSTRACTION
  // NOT: websocketUrl: string                               // ❌ Not on concrete details
  // NOT: httpClient: AxiosInstance                          // ❌ Not on implementation
}

function Sketchpad({                                         // High-level component
  remoteProviders                                            // Receives abstraction
}: SketchpadProps) {
  
  // Uses abstract interface - doesn't know implementation
  const fileProvider = remoteProviders?.fileProvider;        // Uses abstraction
  
  if (fileProvider) {                                        // If provider exists
    const url = fileProvider.getUrl(kitId, fileId, path);    // Uses abstract method
    // Doesn't know if it's HTTP, WebSocket, or local storage
  }
}

// ------------------------------------------------------------
// LOW-LEVEL IMPLEMENTATIONS (implement the abstraction)
// ------------------------------------------------------------

// PRODUCTION: Real HTTP client
const productionProviders: RemoteProviders = {
  yProvider: (guid) => new WebSocketYProvider(guid),         // WebSocket sync
  fileProvider: new HTTPFileProvider(API_URL),               // HTTP file access
};

// DEVELOPMENT: Mock providers
const devProviders: RemoteProviders = {
  yProvider: (guid) => new LocalYProvider(guid),             // Local Y.js
  fileProvider: new MemoryFileProvider(),                    // In-memory files
};

// TESTING: Test doubles
const testProviders: RemoteProviders = {
  yProvider: (guid) => new MockYProvider(),                  // Mock sync
  fileProvider: new MockFileProvider(),                      // Mock files
};

// ------------------------------------------------------------
// DEPENDENCY INJECTION (wire it together at startup)
// ------------------------------------------------------------

// The App INJECTS the appropriate implementation
function App() {
  const providers = IS_PRODUCTION                            // Choose at runtime
    ? productionProviders                                    // Real providers
    : devProviders;                                          // Dev providers
  
  return (
    <Sketchpad remoteProviders={providers} />                // Inject dependency
  );
}

// ------------------------------------------------------------
// WHY DEPENDENCY INVERSION?
// ------------------------------------------------------------
// BEFORE (tight coupling):
//   Sketchpad → WebSocket, HTTP, IndexedDB (breaks when these change)
//
// AFTER (dependency inversion):
//   Sketchpad → RemoteProviders ← WebSocket, HTTP, IndexedDB
//                  ↑
//              Abstraction layer
//
// BENEFITS:
// ✓ Sketchpad doesn't change when storage technology changes
// ✓ Can swap WebSocket for long polling without touching UI
// ✓ Can test UI without real network connections
// ✓ High-level policy protected from low-level volatility
```

**Why SOLID matters for semio**

- Single Responsibility: Each file has one job, easy to find code
- Open/Closed: Plugin architecture for apps and events
- Interface Segregation: Small interfaces enable mocking
- Dependency Inversion: Different environments (browser, Electron, test)

**What it enables**

- Extensible architecture
- Testable components
- Maintainable codebase

**What it limits**

- More files and indirection
- Design effort upfront
- Learning curve for patterns

---

#### 10.11 Scalability: Growing Without Breaking

**Plain explanation**

Scalability is the ability to handle growth. A system that works for 100 users but crashes with 10,000 users doesn't scale. A system that handles 10,000 users but costs $1 million for 100,000 users scales poorly.

For semio, scalability means: kits with thousands of pieces, collaborative editing with many users, large model files.

**Technical explanation**

**Scaling dimensions in semio**:

| Dimension | Challenge | Solution |
|-----------|-----------|----------|
| Kit size (pieces) | 10,000+ pieces in design | Virtualization, pagination |
| File size (models) | Large .glb files | Streaming, compression |
| Concurrent users | Real-time collaboration | Y.js CRDT, WebSocket |
| Build time | Large monorepo | Nx caching, incremental builds |

**UI virtualization** (large piece lists):

```tsx
// ============================================================
// EXAMPLE 1: UI VIRTUALIZATION - RENDER ONLY WHAT'S VISIBLE
// ============================================================
// Purpose: Handle 10,000+ pieces without lag
// Relates to: Don't render what you can't see
// Only ~20 rows are in viewport at once, so only render those
//
// Conceptual example based on @tanstack/react-virtual

import { useVirtualizer } from '@tanstack/react-virtual';    // Virtualization library

function PieceList({ pieces }: { pieces: Piece[] }) {        // Component with huge list
  
  // ------------------------------------------------------------
  // CONFIGURE THE VIRTUALIZER
  // ------------------------------------------------------------
  
  const virtualizer = useVirtualizer({                       // Create virtualizer
    count: pieces.length,                                    // Total items (10,000+)
    getScrollElement: () => containerRef.current,            // Scroll container
    estimateSize: () => 40,                                  // Row height in pixels
    // Only ~10-20 items rendered at once
  });
  
  // ------------------------------------------------------------
  // RENDER ONLY VISIBLE ITEMS
  // ------------------------------------------------------------
  
  return (
    <div 
      ref={containerRef}                                     // Reference for scroll detection
      style={{ overflow: 'auto', height: 400 }}              // Scrollable container
    >
      <div style={{ height: virtualizer.getTotalSize() }}>   // Full height for scrollbar
        {virtualizer.getVirtualItems().map(virtualRow => (   // Only visible items!
          <PieceRow                                          // Single visible row
            key={virtualRow.key}                             // React key
            piece={pieces[virtualRow.index]}                 // Get piece at index
            style={{                                         // Position absolutely
              position: 'absolute',
              top: virtualRow.start,                         // Calculated position
              height: virtualRow.size,
            }}
          />
        ))}
      </div>
    </div>
  );
}

// ------------------------------------------------------------
// WHY VIRTUALIZATION?
// ------------------------------------------------------------
// WITHOUT: Render 10,000 <PieceRow> → browser freezes
// WITH:    Render ~20 <PieceRow>    → smooth 60fps scrolling
//
// The virtualizer:
// - Tracks scroll position
// - Calculates which items are visible
// - Only creates DOM elements for visible items
// - Recycles elements as you scroll
```

**DerivedStore caching** (expensive computations):

```typescript
// ============================================================
// EXAMPLE 2: DERIVEDSTORE CACHING - COMPUTE ONCE, USE MANY
// ============================================================
// Purpose: Avoid recomputing expensive values on every render
// Relates to: Memoization and caching for performance
// Computing piece metadata for 10,000 pieces is expensive
// Only recompute when pieces actually change
//
// js/semio/sketchpad/Sketchpad.tsx

// ------------------------------------------------------------
// THE PROBLEM: Expensive computation
// ------------------------------------------------------------
// computePiecesMetadata calculates:
// - Hierarchy depth for each piece
// - Parent/child relationships
// - Flattened planes and positions
// For 10,000 pieces, this takes ~200ms

// WITHOUT caching:
function BadComponent() {
  const metadata = computePiecesMetadata(design);            // Runs EVERY render
  // 60 fps × 200ms = impossible, app freezes
}

// ------------------------------------------------------------
// THE SOLUTION: DerivedStore
// ------------------------------------------------------------

// Create a cached computed value
const piecesMetadataNode = derivedStore.getOrCreate(         // Get from cache or create
  "piecesMetadata",                                          // Cache key (unique string)
  [
    {                                                        // Dependencies:
      store: designStore,                                    // Watch this store
      path: [yPathMapKey("pieces")]                          // Watch this path
    }
  ],
  () => computePiecesMetadata(designStore.snapshot())        // Compute function
);

// ------------------------------------------------------------
// USING THE CACHED VALUE
// ------------------------------------------------------------

function GoodComponent() {
  const metadata = piecesMetadataNode.snapshot();            // Returns cached value
  // If pieces haven't changed: instant (cached)
  // If pieces changed: recomputes once, then caches
}

// ------------------------------------------------------------
// HOW IT WORKS
// ------------------------------------------------------------
// 1. First call: runs computePiecesMetadata, stores result
// 2. Subsequent calls: returns stored result (fast!)
// 3. When pieces change: Y.js notifies, cache is invalidated
// 4. Next call: recomputes and caches new result

// VISUALIZATION:
// Render 1: pieces unchanged → cache hit  → 0.01ms
// Render 2: pieces unchanged → cache hit  → 0.01ms
// Render 3: piece added      → recompute  → 200ms
// Render 4: pieces unchanged → cache hit  → 0.01ms
// Render 5: pieces unchanged → cache hit  → 0.01ms
```

**Nx build caching** (monorepo scaling):

```bash
# ============================================================
# EXAMPLE 3: NX BUILD CACHING - DON'T REBUILD WHAT DIDN'T CHANGE
# ============================================================
# Purpose: Make large monorepo builds fast
# Relates to: Caching at the build system level
# semio has ~15 packages - rebuilding all takes 10+ minutes
# Nx caches outputs and replays them when inputs match
#
# Run from: d:\semio

# ------------------------------------------------------------
# FIRST BUILD: Computes everything from scratch
# ------------------------------------------------------------

$ npx nx build @semio/js
> @semio/js:build [2m 30s]                                   # Full build: slow

# What happened:
# 1. Nx computed input hash (all source files + dependencies)
# 2. Ran the build command
# 3. Stored output files in .nx/cache/ with input hash as key

# ------------------------------------------------------------
# SECOND BUILD: Cache hit (nothing changed)
# ------------------------------------------------------------

$ npx nx build @semio/js
> @semio/js:build [retrieved from cache, 0.1s]               # Cache hit: fast!

# What happened:
# 1. Nx computed input hash → matches previous build
# 2. Skipped build entirely
# 3. Copied cached outputs to dist/ folder
# 4. 2 minutes 30 seconds saved!

# ------------------------------------------------------------
# AFFECTED BUILDS: Only rebuild what changed
# ------------------------------------------------------------

$ npx nx affected -t build
> Only rebuilding packages affected by changes

# If you changed js/semio/semio.ts:
# - @semio/js      → rebuild (source changed)
# - @semio/vscode  → rebuild (depends on @semio/js)
# - @semio/docs    → skip (not affected)
# - @semio/desktop → rebuild (depends on @semio/js)

# BEFORE NX:    15 packages × 2 min = 30 min CI
# WITH NX:      3 affected  × 2 min = 6 min CI
```

**Y.js document scaling** (large collaborative documents):

```typescript
// ============================================================
// EXAMPLE 4: Y.JS DOCUMENT SCALING - EFFICIENT COLLABORATION
// ============================================================
// Purpose: Handle large shared documents with many editors
// Relates to: CRDT (Conflict-free Replicated Data Type) efficiency
// Y.js is optimized for documents that grow over time
//
// js/semio/sketchpad/Sketchpad.tsx

// ------------------------------------------------------------
// HOW Y.JS HANDLES LARGE DOCUMENTS
// ------------------------------------------------------------

// 1. EFFICIENT ENCODING
// Y.js uses compact binary encoding for operations
// 1 million operations ≈ 10 MB (very efficient)

// 2. DELTA UPDATES
// Only changes are sent over the network
const design = yDoc.getMap('design');                        // Get shared map
design.set('pieces', [...pieces, newPiece]);                 // Add piece

// Network sends:
// ✗ NOT the entire 10,000 piece array
// ✓ Just the delta: { insert: newPiece, position: 10000 }

// 3. GARBAGE COLLECTION
// Old operations are merged when all clients have them
yDoc.gc = true;                                              // Enable GC
// 100,000 historical operations → ~1,000 tombstones

// ------------------------------------------------------------
// FOR VERY LARGE KITS: Subdocuments
// ------------------------------------------------------------

// Problem: One kit with 50 designs, each with 10,000 pieces
// Loading entire Y.Doc would be too slow

// Solution: Split into subdocuments
const yDoc = new Y.Doc();                                    // Main document
const subDocs = yDoc.getMap('subDocs');                      // Subdoc container

// Each design gets its own Y.Doc
function loadDesign(designGuid: Guid): Y.Doc {               // Lazy load function
  const ySubDoc = new Y.Doc({ guid: designGuid });           // Create subdoc
  subDocs.set(designGuid, ySubDoc);                          // Register it
  return ySubDoc;                                            // Return for use
}

// Benefits:
// - Only load designs being edited
// - Smaller sync payloads
// - Faster initial load
// - Independent GC per design
```

**Why scalability matters for semio**

- Architects work with large buildings (1000s of pieces)
- Teams collaborate in real-time
- Kit files contain 3D models
- Monorepo has many packages

**What it enables**

- Handle real-world project sizes
- Smooth collaboration at scale
- Fast development cycles despite large codebase

**What it limits**

- Complexity of optimization
- Memory management concerns
- Testing at scale is difficult

---

#### 10.12 Reliability: Always Working

**Plain explanation**

Reliability means the system works when you need it. A reliable system doesn't crash, doesn't lose data, and continues functioning even when things go wrong.

For semio, reliability means: designs don't lose pieces, collaborative edits don't conflict, offline editing works.

**Technical explanation**

**Reliability in semio**:

| Failure | Protection | Implementation |
|---------|------------|----------------|
| Browser crash | Local persistence | IndexedDB auto-save |
| Network disconnect | Offline-first | Y.js local state + sync |
| Concurrent edits | Conflict-free | CRDT merging |
| Invalid data | Validation | Constraints + fixes |
| Build failure | Caching | Nx cache recovery |

**Offline-first architecture**:

```typescript
// ============================================================
// EXAMPLE 1: OFFLINE-FIRST - WORK WITHOUT NETWORK
// ============================================================
// Purpose: Users can edit designs even when disconnected
// Relates to: Local-first data ownership
// Y.js stores everything locally, syncs when network returns
//
// js/semio/sketchpad/Sketchpad.tsx

// ------------------------------------------------------------
// SETUP: Y.js with IndexedDB persistence
// ------------------------------------------------------------

const yDoc = new Y.Doc();                                    // Create Y.js document

const persistence = new IndexeddbPersistence(                // Connect to browser storage
  kitGuid,                                                   // Storage key (kit identifier)
  yDoc                                                       // Document to persist
);

// Data is now saved to IndexedDB automatically
// Even if browser crashes, data survives

// ------------------------------------------------------------
// EDITING: Works completely offline
// ------------------------------------------------------------

yDoc.transact(() => {                                        // Start transaction
  yPieces.push([newPiece]);                                  // Add piece locally
});                                                          // Auto-saved to IndexedDB

// No network needed! User can:
// - Add pieces ✓
// - Delete connections ✓
// - Modify attributes ✓
// - Undo/redo ✓

// ------------------------------------------------------------
// SYNCING: Automatic when network returns
// ------------------------------------------------------------

const provider = new WebsocketProvider(                      // Connect to server
  serverUrl,                                                 // WebSocket server URL
  kitGuid,                                                   // Room identifier
  yDoc                                                       // Local document
);

provider.on('sync', () => {                                  // Sync complete event
  console.log('Synced with server');                         // All local changes sent
});

// TIMELINE:
// 1. User edits offline for 2 hours
// 2. 100 operations queued locally in Y.js
// 3. User reconnects to WiFi
// 4. Y.js automatically syncs all 100 operations
// 5. Other users see the changes
```

**CRDT conflict resolution**:

```typescript
// ============================================================
// EXAMPLE 2: CRDT CONFLICT RESOLUTION - NO MERGE CONFLICTS
// ============================================================
// Purpose: Multiple users can edit simultaneously without conflicts
// Relates to: Conflict-free Replicated Data Types (CRDT)
// Unlike Git (which has merge conflicts), Y.js always merges cleanly
//
// Conceptual explanation of Y.js CRDT behavior

// ------------------------------------------------------------
// SCENARIO 1: Both users add items (no conflict)
// ------------------------------------------------------------

// User A (offline):
yPieces.push([{ guid: "piece-1", name: "Wall" }]);           // Adds Piece 1

// User B (offline):  
yPieces.push([{ guid: "piece-2", name: "Door" }]);           // Adds Piece 2

// When both sync → RESULT: Both pieces exist!
// pieces = [Piece 1, Piece 2]
// No conflict! CRDTs are designed for concurrent additions

// ------------------------------------------------------------
// SCENARIO 2: Both users modify same field (last-writer-wins)
// ------------------------------------------------------------

// User A at t=100:
piece.name = "Wall A";                                       // Sets name to "Wall A"

// User B at t=200:
piece.name = "Wall B";                                       // Sets name to "Wall B"

// When both sync → RESULT: "Wall B" (later timestamp wins)
// This is deterministic - both clients compute same result

// ------------------------------------------------------------
// SCENARIO 3: Text editing (character-level merge)
// ------------------------------------------------------------

// Original: "Hello World"
// User A types at position 5:  "Hello, World"  (added comma)
// User B types at position 11: "Hello World!"  (added exclamation)

// When both sync → RESULT: "Hello, World!"
// Y.js merges at character level, preserving both edits

// ------------------------------------------------------------
// WHY THIS MATTERS FOR SEMIO
// ------------------------------------------------------------
// Two architects editing same design:
// - One adds pieces to the left side
// - Other adds connections on the right side
// - No merge conflicts, ever
// - No "please resolve conflicts" dialogs
// - Just seamless collaboration
```

**Graceful degradation**:

```typescript
// ============================================================
// EXAMPLE 3: GRACEFUL DEGRADATION - FAIL SAFELY
// ============================================================
// Purpose: When something fails, provide a fallback instead of crashing
// Relates to: Defensive programming and error handling
// If remote upload fails, use local storage instead
//
// js/semio/sketchpad/Sketchpad.tsx

// ------------------------------------------------------------
// FALLBACK CHAIN: Remote → Local → Error message
// ------------------------------------------------------------

async function uploadModel(blob: Blob): Promise<string> {    // Upload function
  
  try {                                                      // Try primary option
    return await remoteProvider.upload(                      // Remote upload
      kitId, fileId, path, blob
    );
  } catch (error) {                                          // Remote failed
    
    console.warn('Remote upload failed, using local storage');
    
    try {                                                    // Try fallback
      return await localProvider.upload(                     // Local storage
        kitId, fileId, path, blob
      );
    } catch (localError) {                                   // Local also failed
      
      console.error('Both uploads failed');
      throw new Error('Could not save model');               // Only then show error
    }
  }
}

// USER EXPERIENCE:
// Best case:   Remote works         → file synced to cloud
// Fallback:    Remote fails         → file saved locally
// Worst case:  Both fail            → clear error message

// ------------------------------------------------------------
// UI FALLBACK: 3D fails → show 2D
// ------------------------------------------------------------

function ModelView({ model }: { model: Model }) {            // Model viewer
  return (
    <ErrorBoundary                                           // React error boundary
      fallback={<Diagram2D model={model} />}                 // 2D fallback
    >
      <Scene3D model={model} />                              // Try 3D first
    </ErrorBoundary>
  );
}

// SCENARIOS:
// WebGL available:     3D view with rotation and zoom
// WebGL unavailable:   2D orthographic diagram
// Both fail:           Static placeholder image

// ------------------------------------------------------------
// WHY GRACEFUL DEGRADATION?
// ------------------------------------------------------------
// Users have varying:
// - Network quality (fiber vs. mobile)
// - Browser capabilities (Chrome vs. old Safari)
// - Hardware (gaming PC vs. old laptop)
//
// Better to show something than crash with an error
```

**Data validation on load**:

```typescript
// ============================================================
// EXAMPLE 4: DATA VALIDATION - DETECT AND FIX CORRUPTION
// ============================================================
// Purpose: Ensure loaded data is valid and fix problems automatically
// Relates to: Defensive programming and data integrity
// Kits might be corrupted from bugs, crashes, or manual editing
//
// js/semio/semio.ts - validateKit function

// ------------------------------------------------------------
// LOAD AND VALIDATE
// ------------------------------------------------------------

const kit = loadKitFromStorage(kitGuid);                     // Load from storage

const validation = validateKit(kit);                         // Run validation

// Validation checks:
// - All GUIDs are unique
// - All references resolve (piece.type points to real type)
// - Required fields exist (kit.name is present)
// - Constraints satisfied (piece.scale > 0)

// ------------------------------------------------------------
// HANDLE VALIDATION RESULTS
// ------------------------------------------------------------

if (hasSemioErrors(validation)) {                            // Has errors?
  console.error(                                             // Log for debugging
    'Kit has validation errors:', 
    validation.problems
  );
  
  // AUTO-FIX: Apply available fixes
  for (const problem of validation.problems) {               // Each problem
    if (problem.fixes.length > 0) {                          // Has a fix?
      kit = applyKitDiff(kit, problem.fixes[0].diff);        // Apply first fix
    }
  }
  
  // Save the fixed kit
  saveKitToStorage(kitGuid, kit);                            // Persist fix
}

// ------------------------------------------------------------
// EXAMPLE PROBLEM AND FIX
// ------------------------------------------------------------
// 
// Problem: {
//   constraintId: "guid-unique",
//   message: "Duplicate GUID 'abc-123' found",
//   location: { entityKind: "Piece", entityGuid: "abc-123" },
//   fixes: [{
//     title: "Regenerate GUID",
//     diff: { designs: { updated: [{ guid: "design-1", pieces: { 
//       updated: [{ guid: "abc-123-NEW", ... }] 
//     }}]}}
//   }]
// }
//
// Fix automatically regenerates a new GUID and updates references
```

**Why reliability matters for semio**

- Architects invest hours in designs—can't lose work
- Collaboration requires consistent state
- Real-world projects depend on semio

**What it enables**

- User trust
- Offline workflows
- Concurrent collaboration

**What it limits**

- CRDT overhead
- Validation performance
- Complex sync logic

---

#### 10.13 Performance: Speed and Efficiency

**Plain explanation**

Performance is how fast and efficiently a system operates. A fast website loads in milliseconds. A slow one takes seconds and users leave. Performance is often invisible when good and painfully obvious when bad.

For semio, performance means: fast piece selection, smooth 3D rendering, instant undo/redo.

**Technical explanation**

**Performance optimizations in semio**:

| Operation | Optimization | Result |
|-----------|--------------|--------|
| Re-renders | Memoization, selectors | Only changed components update |
| 3D rendering | Instancing, LOD | Thousands of pieces at 60fps |
| Kit loading | Lazy loading | Fast initial load |
| Snapshot creation | Hash caching | Avoid rebuild if unchanged |
| Build | Nx caching | 0.1s cached vs 2m full |

**Memoization** (prevent unnecessary recalculation):

```typescript
// ============================================================
// EXAMPLE 1: MEMOIZATION - CACHE EXPENSIVE COMPUTATIONS
// ============================================================
// Purpose: Avoid recalculating the same thing on every render
// Relates to: Caching results based on inputs
// React re-renders often - memoization prevents wasted work
//
// js/semio/sketchpad/Design.tsx

// ------------------------------------------------------------
// useMemo: Cache computed values
// ------------------------------------------------------------

const piecesMetadata = useMemo(                              // Cache this value
  () => {
    return computePiecesMetadata(pieces, connections);       // Expensive calculation
  },                                                         // Only run when:
  [pieces, connections]                                      // These dependencies change
);

// WITHOUT useMemo:
// Render 1: compute (200ms)
// Render 2: compute again (200ms) - same inputs!
// Render 3: compute again (200ms) - wasted work!

// WITH useMemo:
// Render 1: compute (200ms) - cache result
// Render 2: return cached (0.01ms) - same inputs
// Render 3: return cached (0.01ms) - saved 400ms!

// ------------------------------------------------------------
// useCallback: Cache function references
// ------------------------------------------------------------

const handleSelect = useCallback(                            // Cache this function
  (pieceGuid: Guid) => {                                     // Selection handler
    actor.send({ type: 'DESIGN.SELECT_PIECE', pieceGuid }); // Send event
  },
  [actor]                                                    // Only recreate if actor changes
);

// WHY THIS MATTERS:
// Child components compare function references
// New function reference = child re-renders
// Same function reference = child skips render

// USAGE:
<PieceRow onSelect={handleSelect} />                         // Stable reference
// Child uses React.memo and skips re-render if props unchanged
```

**Selector optimization** (granular subscriptions):

```typescript
// ============================================================
// EXAMPLE 2: SELECTOR OPTIMIZATION - SUBSCRIBE TO WHAT YOU NEED
// ============================================================
// Purpose: Components only re-render when their specific data changes
// Relates to: Granular reactivity and minimal updates
// If 100 things can change, only listen to the 2 you care about
//
// js/semio/sketchpad/Sketchpad.tsx

// ------------------------------------------------------------
// BAD: Subscribe to entire state
// ------------------------------------------------------------

const state = useSelector(actor, (s) => s);                  // ❌ Returns whole state

// PROBLEMS:
// - ANY state change triggers re-render
// - User hovers a piece → this component re-renders
// - User changes theme → this component re-renders
// - User types in search → this component re-renders
// Result: Constant unnecessary re-renders

// ------------------------------------------------------------
// GOOD: Subscribe only to what's needed
// ------------------------------------------------------------

// Only re-render when selection changes
const selection = useSelector(                               // ✓ Granular subscription
  actor,                                                     // State machine actor
  (s) => s.context.designApp?.selection                      // Extract just selection
);

// Only re-render when theme changes
const theme = useSelector(                                   // ✓ Separate subscription
  actor,                                                     // Same actor
  (s) => s.context.theme                                     // Extract just theme
);

// ------------------------------------------------------------
// HOW IT WORKS
// ------------------------------------------------------------
// XState compares selector return values
// If selection unchanged but hover changed:
//   - selection selector returns same value → no re-render
//   - hover selector returns new value → that component re-renders
//
// RESULT: Each component only re-renders when ITS data changes
```

**Snapshot hash caching**:

```typescript
// ============================================================
// EXAMPLE 3: SNAPSHOT HASH CACHING - DON'T REBUILD UNCHANGED DATA
// ============================================================
// Purpose: Avoid expensive snapshot rebuilds when data hasn't changed
// Relates to: Hash-based change detection
// Building a snapshot walks the entire Y.js document - expensive!
// Cache the result and only rebuild when hash changes
//
// js/semio/sketchpad/Sketchpad.tsx - Store base class

class Store<TState> {                                        // Generic store
  private cachedSnapshot: TState | null = null;              // Cached result
  private cachedHash: string | null = null;                  // Hash of cached state
  
  // ------------------------------------------------------------
  // GET SNAPSHOT: Return cached or rebuild
  // ------------------------------------------------------------
  
  snapshot(): TState {                                       // Get current state
    const currentHash = this.hash(this.buildSnapshot());     // Compute current hash
    
    if (currentHash === this.cachedHash) {                   // Hash matches cached?
      return this.cachedSnapshot!;                           // Return cached (fast!)
    }
    
    // Hash changed - need to rebuild
    this.cachedSnapshot = this.buildSnapshot();              // Rebuild snapshot
    this.cachedHash = currentHash;                           // Cache new hash
    return this.cachedSnapshot;                              // Return new snapshot
  }
  
  // ------------------------------------------------------------
  // ABSTRACT METHODS (implemented by subclasses)
  // ------------------------------------------------------------
  
  protected abstract hash(state: TState): string;            // Compute hash
  protected abstract buildSnapshot(): TState;                // Build from Y.js
}

// ------------------------------------------------------------
// HOW IT WORKS
// ------------------------------------------------------------
// 1. First call: buildSnapshot() runs, hash computed, cached
// 2. Second call: hash matches → return cached (0.01ms vs 50ms)
// 3. Data changes: Y.js notifies → hash changes → rebuild
// 4. Third call: new hash matches new cache → fast again

// WHY HASH INSTEAD OF DEEP EQUALITY?
// - Deep equality on 10,000 pieces = slow
// - Hash computation is O(n) but fast
// - Hash comparison is O(1) - just string compare
```

**3D rendering optimization**:

```typescript
// ============================================================
// EXAMPLE 4: 3D RENDERING OPTIMIZATION - THOUSANDS OF PIECES AT 60FPS
// ============================================================
// Purpose: Render large designs smoothly using GPU optimization
// Relates to: GPU instancing and Level of Detail (LOD)
// Without optimization: 1000 pieces = 1000 draw calls = 15 fps
// With optimization: 1000 pieces = 1 draw call = 60 fps
//
// js/semio/sketchpad/elements.tsx - Scene3D component

// ------------------------------------------------------------
// INSTANCED MESH: One draw call for many identical meshes
// ------------------------------------------------------------

// Create ONE mesh that renders many instances
const instancedMesh = new THREE.InstancedMesh(               // Instanced mesh
  geometry,                                                  // Shared geometry (Wall shape)
  material,                                                  // Shared material
  pieceCount                                                 // How many instances (1000+)
);

// Set transform for each instance
pieces.forEach((piece, index) => {                           // For each piece
  const matrix = planeToMatrix4(piece.plane);                // Convert semio Plane to Matrix4
  instancedMesh.setMatrixAt(index, matrix);                  // Set instance transform
});

// GPU renders all instances in ONE draw call
// CPU sends: 1 geometry + 1000 transforms
// GPU draws: 1000 pieces in parallel

// ------------------------------------------------------------
// LEVEL OF DETAIL (LOD): Simpler geometry for distant objects
// ------------------------------------------------------------

const lod = new THREE.LOD();                                 // LOD container

lod.addLevel(highDetailMesh, 0);                             // Full detail when close
lod.addLevel(mediumDetailMesh, 50);                          // Simpler at 50 units away
lod.addLevel(lowDetailMesh, 100);                            // Simplest at 100+ units

// When camera is:
// - 0-50 units:   Use highDetailMesh   (10,000 triangles)
// - 50-100 units: Use mediumDetailMesh (1,000 triangles)
// - 100+ units:   Use lowDetailMesh    (100 triangles)

// ------------------------------------------------------------
// COMBINED EFFECT
// ------------------------------------------------------------
// Without optimization:
//   1000 pieces × 10,000 triangles × per-piece draw = 10fps
//
// With instancing + LOD:
//   1 draw call × adaptive triangles = 60fps
//
// User zooms in: more detail, fewer pieces visible
// User zooms out: less detail, more pieces visible
// Always smooth!
```

**Why performance matters for semio**

- Interactive 3D requires 60fps
- Large designs have thousands of pieces
- Real-time collaboration adds overhead

**What it enables**

- Smooth user experience
- Handle large projects
- Responsive editing

**What it limits**

- Optimization complexity
- Memory vs speed tradeoffs
- Debugging optimized code

---

#### 10.14 Testing: Verifying Correctness

**Plain explanation**

Testing is running your code with known inputs and checking that outputs are correct. It's like proofreading before publishing—catching mistakes before they reach users. Automated tests can be run on every code change, providing continuous verification.

semio uses unit tests (vitest), E2E tests (Playwright), and domain tests (pure function validation).

**Technical explanation**

**Testing pyramid in semio**:

```
┌───────────────────────────────────────────┐
│            E2E Tests (Playwright)          │  Slowest, most realistic
│         Full user workflows in browser     │
├───────────────────────────────────────────┤
│        Integration Tests                   │  Medium speed
│     Component rendering, API calls         │
├───────────────────────────────────────────┤
│          Unit Tests (Vitest)               │  Fastest, most numerous
│     Pure functions, domain logic           │
└───────────────────────────────────────────┘
```

**Unit tests** (`js/semio/semio.test.ts`):

```typescript
// ============================================================
// EXAMPLE 1: UNIT TESTS - TESTING PURE FUNCTIONS IN ISOLATION
// ============================================================
// Purpose: Verify domain logic works correctly with known inputs
// Relates to: Fast feedback loop, test-driven development
// Unit tests run in milliseconds - can run on every save
//
// js/semio/semio.test.ts

import { describe, it, expect } from 'vitest';               // Test framework
import { applyKitDiff, getKitDiff, inverseKitDiff } from './semio';

describe('Kit Diff Operations', () => {                      // Test group
  
  // ------------------------------------------------------------
  // TEST 1: Adding a type via diff
  // ------------------------------------------------------------
  
  it('applies diff to add a type', () => {                   // Single test case
    // ARRANGE: Set up initial state
    const kit = { types: [], designs: [] };                  // Empty kit
    const diff = {                                           // Diff that adds a type
      types: { 
        added: [{ guid: 'type-1', name: 'Wall' }]            // New type to add
      } 
    };
    
    // ACT: Perform the operation
    const result = applyKitDiff(kit, diff);                  // Apply the diff
    
    // ASSERT: Verify the result
    expect(result.types).toHaveLength(1);                    // Should have 1 type
    expect(result.types[0].name).toBe('Wall');               // Should be named 'Wall'
  });
  
  // ------------------------------------------------------------
  // TEST 2: Inverse diff reverses operations (critical for undo)
  // ------------------------------------------------------------
  
  it('inverse diff reverses the operation', () => {          // Test undo capability
    // ARRANGE: Before and after states
    const before = { types: [] };                            // Started empty
    const after = { types: [{ guid: 'type-1', name: 'Wall' }] };
    
    // ACT: Get diff and its inverse
    const diff = getKitDiff(before, after);                  // Calculate diff
    const inverse = inverseKitDiff(before, diff);            // Calculate inverse
    
    const restored = applyKitDiff(after, inverse);           // Apply inverse
    
    // ASSERT: Should be back to before state
    expect(restored.types).toHaveLength(0);                  // Empty again!
  });
});

// ------------------------------------------------------------
// WHY UNIT TESTS ARE VALUABLE
// ------------------------------------------------------------
// - Run in 50ms (instant feedback)
// - No browser needed (can run in CI)
// - Test edge cases exhaustively
// - Document expected behavior
// - Catch regressions immediately
```

**E2E tests** (`js/semio/playwright/`):

```typescript
// ============================================================
// EXAMPLE 2: E2E TESTS - TESTING REAL USER WORKFLOWS IN BROWSER
// ============================================================
// Purpose: Verify complete user journeys work end-to-end
// Relates to: Integration testing, user acceptance testing
// E2E tests use a real browser - slower but catches more bugs
//
// js/semio/playwright/kit/design/seed.spec.ts

import { test, expect } from '@playwright/test';             // Playwright framework

test.describe('design', () => {                              // Test group: design app
  
  // ------------------------------------------------------------
  // SEED TEST: Create the minimum state for other tests
  // ------------------------------------------------------------
  
  test('seed', async ({ page }) => {                         // Single E2E test
    // STEP 1: Navigate to app
    await page.goto('http://localhost:5173');                // Open Sketchpad
    
    // STEP 2: Create temporary kit (using element ID)
    await page.locator(                                      // Find by stable ID
      '[id="semio.sketchpad.app.home.createTemporary"]'      // Home app button
    ).click();                                               // Click it
    
    // STEP 3: Create design  
    await page.locator(                                      // Find by stable ID
      '[id="semio.sketchpad.app.kit.kitApp.createDesign"]'   // Kit app button
    ).click();                                               // Click it
    
    // STEP 4: Verify result
    await expect(                                            // Assertion
      page.getByText('New Design')                           // Find text
    ).toBeVisible();                                         // Should be visible
  });
});

// ------------------------------------------------------------
// E2E TEST CONVENTIONS IN SEMIO
// ------------------------------------------------------------
// 1. ID LOCATORS ONLY: '[id="semio.sketchpad.*"]'
//    ✓ Stable across UI changes
//    ✗ Never use text selectors or CSS classes
//
// 2. HIERARCHICAL SEEDING: sketchpad → kit → design
//    Each test builds on previous seeds
//
// 3. NO DIRECT BROWSER API: Only interact via UI
//    ✓ page.locator().click()
//    ✗ window.document.getElementById()
```

**Validation tests** (cross-platform consistency):

```typescript
// ============================================================
// EXAMPLE 3: CROSS-PLATFORM VALIDATION TESTS
// ============================================================
// Purpose: Ensure TypeScript, Python, and C# produce identical output
// Relates to: Multi-language consistency, contract testing
// semio has implementations in 3 languages - they must agree!
//
// js/semio/semio.test.ts

import { validateKit, serializeValidationResult } from './semio';
import invalidKit from '../assets/semio/kit_invalid.json';   // Test fixture
import expectedOutput from '../assets/semio/validation.json'; // Golden file

test('validation output matches expected', () => {           // Consistency test
  // ARRANGE: Load known-invalid kit
  // kit_invalid.json has all validation constraint violations
  
  // ACT: Run TypeScript validation
  const result = validateKit(invalidKit);                    // Validate with TS
  const serialized = serializeValidationResult(result);      // Convert to JSON
  
  // ASSERT: Must match golden file exactly
  expect(serialized).toEqual(expectedOutput);                // Compare to expected
});

// ------------------------------------------------------------
// WHY THIS MATTERS
// ------------------------------------------------------------
// The same kit can be:
// - Loaded in Sketchpad (TypeScript)
// - Processed in engine (Python)
// - Used in Grasshopper (C#)
//
// All three MUST report the same validation errors
// Otherwise: TypeScript says "valid", C# says "error" → chaos

// ------------------------------------------------------------
// GOLDEN FILE TESTING
// ------------------------------------------------------------
// 1. assets/semio/kit_invalid.json - Input (invalid kit)
// 2. assets/semio/validation.json  - Expected output
// 3. Tests in TS, PY, C# compare their output to validation.json
//
// If any language produces different output → test fails
// This catches cross-platform bugs before they ship
```

**Nx test orchestration**:

```bash
# ============================================================
# EXAMPLE 4: NX TEST ORCHESTRATION - SMART TEST RUNNING
# ============================================================
# Purpose: Run tests efficiently across the monorepo
# Relates to: Build system integration, affected tests
# Nx knows which packages depend on what - runs minimal tests
#
# Run from: d:\semio

# ------------------------------------------------------------
# OPTION 1: Run all tests (CI full build)
# ------------------------------------------------------------

npm run test
# Runs:
# - @semio/js tests (vitest)
# - @semio/engine tests (pytest)
# - @semio/vscode tests (VS Code test runner)
# - ... all packages with test command

# ------------------------------------------------------------
# OPTION 2: Run tests for specific package
# ------------------------------------------------------------

npx nx test @semio/js
# Only runs tests in js/semio/
# Useful when working on one package

# ------------------------------------------------------------
# OPTION 3: Run ONLY affected tests (fastest)
# ------------------------------------------------------------

npx nx affected -t test
# Nx analyzes git changes:
# - What files changed?
# - Which packages depend on those files?
# - Only test affected packages!
#
# EXAMPLE:
# Changed: js/semio/semio.ts
# Affected: @semio/js, @semio/vscode (depends on js)
# NOT affected: @semio/engine (Python, independent)
#
# BEFORE: Run 15 test suites (5 minutes)
# AFTER:  Run 2 test suites (30 seconds)
```

**Test file conventions**:

| Type | Location | Framework |
|------|----------|-----------|
| Unit | `*.test.ts` next to source | Vitest |
| E2E | `playwright/*.spec.ts` | Playwright |
| C# | `Semio.Tests/` | xUnit |
| Python | `test_*.py` | pytest |

**Why testing matters for semio**

- Domain logic must be correct (diff operations are complex)
- Cross-platform consistency (TypeScript = Python = C#)
- Regressions are caught before merge
- Refactoring is safe with test coverage

**What it enables**

- Confidence in changes
- Fast feedback on errors
- Documentation via tests
- Safe refactoring

**What it limits**

- Test maintenance overhead
- Slow E2E tests
- Mocking complexity

---

#### 10.15 Monitoring: Seeing What's Happening

**Plain explanation**

Monitoring is watching your system while it runs—tracking its health, performance, and behavior. It's like the dashboard of a car: speedometer, fuel gauge, temperature warning lights.

semio uses console logs during development, VS Code extension diagnostics, and Nx build reports.

**Technical explanation**

**Monitoring in semio development**:

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    semio MONITORING STACK                                │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  Development:                                                           │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  Console Logs: [DEBUG] [SLUG] diagnostic messages               │    │
│  │  VS Code Diagnostics: validation errors in Problems panel       │    │
│  │  React DevTools: component state inspection                     │    │
│  │  XState Inspector: state machine visualization                  │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
│  CI/CD:                                                                 │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  Nx Reports: reports/eslint.json, reports/typescript.json       │    │
│  │  Test Reports: vitest output, Playwright traces                 │    │
│  │  Build Output: Nx build logs with timing                        │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

**Debug logging convention**:

```typescript
// ============================================================
// EXAMPLE 1: DEBUG LOGGING - TEMPORARY DIAGNOSTIC MESSAGES
// ============================================================
// Purpose: Add temporary logs to understand what's happening
// Relates to: Debugging, diagnostics, troubleshooting
// The [DEBUG] prefix makes logs easy to find and remove later
//
// Any file in js/semio/sketchpad/

// ------------------------------------------------------------
// CONVENTION: [DEBUG] [SLUG] message
// ------------------------------------------------------------

console.log(                                                 // Temporary log
  '[DEBUG] [PIECE-DRAG] Mounting Dropzone:',                 // [DEBUG] + [CONTEXT]
  { pieceGuid }                                              // Relevant data
);

console.log(                                                 // Another log
  '[DEBUG] [YJSSYNC] Kit synced:',                           // Different context
  { typeCount: types.length }                                // Metrics
);

// WHY THIS CONVENTION:
// 1. [DEBUG] prefix → easy to search and remove
// 2. [SLUG] context → identify which investigation
// 3. Object logging → expandable in DevTools

// CLEANUP: Before committing
// Search: console.log('[DEBUG]
// Delete all matches

// ------------------------------------------------------------
// PERFORMANCE LOGGING: Detect overfetching
// ------------------------------------------------------------

enablePerformanceLogging(true);                              // Enable perf monitoring

// Console will show:
// [PERF] Rapid re-render detected in <DesignCanvas>
//   Rendered 15 times in 1000ms
//   Possible overfetching or missing memoization

// This helps catch:
// - Components re-rendering too often
// - Expensive computations running repeatedly
// - Missing useMemo/useCallback optimizations
```

**VS Code extension diagnostics**:

```typescript
// ============================================================
// EXAMPLE 2: VS CODE DIAGNOSTICS - PROBLEMS PANEL INTEGRATION
// ============================================================
// Purpose: Show validation errors directly in the editor
// Relates to: IDE integration, real-time feedback
// Developers see errors as they type, not just at build time
//
// js/vscode/extension.ts

function updateDiagnostics(document: TextDocument): void {   // Update on file change
  
  // ------------------------------------------------------------
  // STEP 1: Analyze the file for violations
  // ------------------------------------------------------------
  
  const violations = analyzeFile(document.uri.fsPath);       // Run analysis
  
  // violations might include:
  // - Missing SPDX license header
  // - Inline comments detected
  // - Empty region blocks
  // - Missing translations
  
  // ------------------------------------------------------------
  // STEP 2: Convert violations to VS Code diagnostics
  // ------------------------------------------------------------
  
  const diagnostics = violations.map(v => ({                 // Map each violation
    range: new Range(                                        // Where in the file
      v.line, 0,                                             // Start: line N, column 0
      v.line, 999                                            // End: same line, full width
    ),
    message: v.message,                                      // Error message
    severity: DiagnosticSeverity.Error,                      // Error level
    source: 'semio-repo'                                     // Source identifier
  }));
  
  // ------------------------------------------------------------
  // STEP 3: Show in VS Code Problems panel
  // ------------------------------------------------------------
  
  diagnosticCollection.set(document.uri, diagnostics);       // Update Problems panel
}

// RESULT IN VS CODE:
// Problems panel shows:
// ┌──────────────────────────────────────────────────────────┐
// │ PROBLEMS (3)                                             │
// │ ├── Design.tsx                                           │
// │ │   └── Line 42: Inline comment detected                 │
// │ │   └── Line 100: Missing region end name                │
// │ └── Kit.tsx                                              │
// │     └── Line 15: Missing SPDX license header             │
// └──────────────────────────────────────────────────────────┘
```

**Report generation** (CI hooks):

```bash
# ============================================================
# EXAMPLE 3: REPORT GENERATION - CI/CD MONITORING
# ============================================================
# Purpose: Generate JSON reports for automated analysis
# Relates to: Continuous Integration, automated quality gates
# Each hook generates a report that CI can check
#
# Run from: d:\semio

# ------------------------------------------------------------
# CODE QUALITY REPORT
# ------------------------------------------------------------

npx tsx hooks/code.ts                                        # Run code analysis

# Output file: reports/code.json
{
  "violations": [
    {
      "file": "js/semio/sketchpad/Design.tsx",               # Which file
      "line": 42,                                            # Which line
      "rule": "code:comment:inline",                         # Which rule
      "message": "Inline comment detected"                   # Human message
    }
  ]
}

# ------------------------------------------------------------
# TYPESCRIPT ERRORS REPORT
# ------------------------------------------------------------

npx tsx hooks/typescript.ts                                  # Run TypeScript check

# Output file: reports/typescript.json
# Contains all TypeScript compilation errors

# ------------------------------------------------------------
# ESLINT LINTING REPORT
# ------------------------------------------------------------

npx tsx hooks/eslint.ts                                      # Run ESLint

# Output file: reports/eslint.json
# Contains all linting violations

# ------------------------------------------------------------
# CI WORKFLOW
# ------------------------------------------------------------
# 1. Pre-commit hook runs all hooks
# 2. If any report has violations → commit blocked
# 3. Developer fixes issues
# 4. Try commit again
# 5. Clean reports → commit succeeds
```

**Performance monitoring hooks**:

```typescript
// ============================================================
// EXAMPLE 4: PERFORMANCE MONITORING - TRACK RENDER COUNTS
// ============================================================
// Purpose: Detect when components render too often
// Relates to: Performance debugging, optimization
// If a component renders 100 times per second, something is wrong
//
// Any React component during debugging

// ------------------------------------------------------------
// TRACK RENDER COUNT
// ------------------------------------------------------------

function DesignCanvas() {                                    // Component to monitor
  const renderCountRef = useRef(0);                          // Persistent counter
  renderCountRef.current++;                                  // Increment on each render
  
  console.log(                                               // Log render count
    `[DEBUG] [PERF] DesignCanvas render #${renderCountRef.current}`
  );
  
  // HEALTHY: render #1, #2, #3 (stable)
  // UNHEALTHY: #1, #2, #3... #50... #100 in 1 second (overfetching!)
  
  // ... component code
}

// ------------------------------------------------------------
// TRACK EFFECT RUNS
// ------------------------------------------------------------

useEffect(() => {                                            // Track this effect
  console.log(                                               // Log when it runs
    '[DEBUG] [EFFECT] Design pieces changed:', 
    pieces.length
  );
}, [pieces]);                                                // Dependency array

// HEALTHY: Runs when pieces actually change
// UNHEALTHY: Runs every render (unstable dependency)

// ------------------------------------------------------------
// DIAGNOSIS EXAMPLE
// ------------------------------------------------------------
// Symptom: DesignCanvas renders 60x per second
// Investigation: pieces dependency is unstable
//   const pieces = kit.types[0].connectors  // New array each time!
// Fix: const pieces = useMemo(() => ..., [stable deps])
```

**Why monitoring matters for semio**

- Debug complex state machine transitions
- Track performance regressions
- Identify overfetching in React hooks
- CI catches policy violations

**What it enables**

- Fast debugging with searchable logs
- Automated policy enforcement
- Performance regression detection

**What it limits**

- Console noise during development
- Report parsing overhead
- Log cleanup required before commit

---

## Part 5: Understanding Complex Systems

### Chapter 11: How Large Products Work

#### 11.1 The Anatomy of a Large System

**Plain explanation**

Large systems are like cities. A small town might have one road, one store, one school. A city has highways, districts, public transit, utilities, emergency services—interconnected systems that each have specialized purposes but work together.

semio is a large system: 5+ languages, 10+ packages, multiple deployment targets, real-time collaboration, 3D rendering.

**Technical explanation**

**semio system anatomy**:

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    semio LARGE SYSTEM ARCHITECTURE                       │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  USER-FACING:                                                           │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  Sketchpad (React + Three.js)                                   │    │
│  │  Desktop App (Electron)                                         │    │
│  │  Grasshopper Plugin (C#)                                        │    │
│  │  VS Code Extension (TypeScript)                                 │    │
│  │  Documentation (Astro + MDX)                                    │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                           │                                              │
│  DOMAIN LOGIC:                                                          │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  @semio/js (TypeScript) - shared domain, UI components          │    │
│  │  py/engine (Python) - computation, schema generation            │    │
│  │  net/Semio (C#) - .NET domain for Rhino/Grasshopper            │    │
│  │  go/repo (Go) - CLI and MCP tools                               │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                           │                                              │
│  DATA:                                                                  │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  SQLite (kit.db) - static kit storage                           │    │
│  │  IndexedDB (Y.js) - browser persistence                         │    │
│  │  File System (.zip, .glb) - model assets                        │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                           │                                              │
│  INFRASTRUCTURE:                                                        │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  Nx (build orchestration)                                       │    │
│  │  GitHub Actions (CI/CD)                                         │    │
│  │  npm/PyPI/NuGet/Yak (package distribution)                      │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

**Package count in semio**:

| Language | Packages | Purpose |
|----------|----------|---------|
| TypeScript | @semio/js, @semio/vscode, @semio/docs, @semio/desktop, @semio/play | Web, editor, docs |
| Python | @semio/engine | Computation, schemas |
| C# | Semio, Semio.Grasshopper | Rhino integration |
| Go | go/repo, go/mcp | CLI, MCP server |

**Why semio is a large system**

- Multiple user interfaces (Sketchpad, Grasshopper, VS Code)
- Real-time collaboration (Y.js CRDT)
- 3D rendering (Three.js)
- Cross-platform (browser, desktop, Rhino)
- Schema consistency across languages

**What it enables**

- Right tool for each job
- Independent deployment
- Specialized teams

**What it limits**

- Coordination complexity
- Cross-language debugging
- Schema synchronization

---

#### 11.2 Multi-Language Systems

**Plain explanation**

Most large products use multiple programming languages—like a restaurant using different tools for different jobs: knives for prep, stoves for cooking, refrigerators for storage.

semio uses TypeScript (web), Python (computation), C# (Rhino), and Go (CLI)—each chosen for its ecosystem.

**Technical explanation**

**semio multi-language architecture**:

| Language | Package | Why This Language |
|----------|---------|-------------------|
| TypeScript | @semio/js | React ecosystem, type safety, browser |
| Python | py/engine | NumPy/SciPy computation, schema generation |
| C# | net/Semio | Rhino/Grasshopper SDK requirement |
| Go | go/repo | Fast CLI, cross-compilation, single binary |
| SQL | sql/sqlite | Database queries |

**How semio languages communicate**:

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    semio CROSS-LANGUAGE COMMUNICATION                    │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  TypeScript ──JSON Kit──> Python                                        │
│      │                       │                                           │
│      │  GraphQL              │  JSON                                    │
│      ▼                       ▼                                           │
│  TypeScript <──JSON Kit── Python                                        │
│                                                                          │
│  TypeScript ──JSON Kit──> C# (Grasshopper import/export)                │
│                                                                          │
│  TypeScript ──CLI JSON──> Go (via stdio)                                │
│      │                       │                                           │
│      │  MCP Protocol         │  JSON                                    │
│      ▼                       ▼                                           │
│  VS Code <──Tool Results── Go MCP Server                                │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

**Shared schemas** (JSON as universal format):

```typescript
// ============================================================
// EXAMPLE 1: SHARED SCHEMAS - SAME DATA STRUCTURE ACROSS LANGUAGES
// ============================================================
// Purpose: All languages work with identical data structures
// Relates to: Cross-language interoperability, schema-first design
// When TypeScript creates a Kit, Python and C# can read it perfectly
//
// Same structure in 3 languages:

// ------------------------------------------------------------
// TYPESCRIPT: js/semio/semio.ts
// ------------------------------------------------------------

export interface Kit {                                       // Kit interface
  guid: Guid;                                                // Unique identifier
  name: string;                                              // Human readable name
  types: Type[];                                             // Building block types
  designs: Design[];                                         // Assembled designs
}

// ------------------------------------------------------------
// PYTHON: py/engine/engine.py
// ------------------------------------------------------------

@dataclass                                                   # Python dataclass
class Kit:
    guid: str                                                # Same field: guid
    name: str                                                # Same field: name
    types: list[Type]                                        # Same field: types
    designs: list[Design]                                    # Same field: designs

// ------------------------------------------------------------
// C#: net/Semio/Semio.cs
// ------------------------------------------------------------

public class Kit {                                           // C# class
    public Guid Guid { get; set; }                           // Same field: Guid
    public string Name { get; set; }                         // Same field: Name
    public List<Type> Types { get; set; }                    // Same field: Types
    public List<Design> Designs { get; set; }                // Same field: Designs
}

// ------------------------------------------------------------
// ALL PRODUCE IDENTICAL JSON:
// ------------------------------------------------------------
{
  "guid": "abc-123",                                         // Same in all languages
  "name": "Metabolism",                                      // Same in all languages
  "types": [...],                                            // Same structure
  "designs": [...]                                           // Same structure
}

// TypeScript → JSON → Python → JSON → C# → JSON → TypeScript
// Perfect round-trip compatibility!
```

**Schema generation** (single source of truth):

```bash
# ============================================================
# EXAMPLE 2: SCHEMA GENERATION - ONE SOURCE, MANY OUTPUTS
# ============================================================
# Purpose: Generate all schemas from TypeScript definitions
# Relates to: Don't Repeat Yourself (DRY), code generation
# Instead of maintaining 4 schema files, maintain 1 and generate rest
#
# Run from: d:\semio

# ------------------------------------------------------------
# GENERATE ALL SCHEMAS FROM TYPESCRIPT
# ------------------------------------------------------------

npx tsx py/engine/generate-schemas.ts                        # Run generator

# This script reads js/semio/semio.ts and produces:

# 1. JSON SCHEMA: jsonschema/kit.json
# - Used by: VS Code extension for validation
# - Used by: API documentation
# - Used by: Form generation

# 2. GRAPHQL: graphql/semio/schema.graphql
# - Used by: Python engine GraphQL API
# - Used by: Frontend queries

# 3. SQLITE: sql/sqlite/schema.sql
# - Used by: Static kit storage (.zip/kit.db)
# - Used by: Python database operations

# ------------------------------------------------------------
# WHY GENERATE SCHEMAS?
# ------------------------------------------------------------
# WITHOUT generation (manual maintenance):
#   - Change Type in semio.ts
#   - Forget to update kit.json → validation broken
#   - Forget to update schema.sql → database migration fails
#   - 4 files can get out of sync
#
# WITH generation (single source):
#   - Change Type in semio.ts
#   - Run generator
#   - All schemas updated automatically
#   - Impossible to have inconsistency
```

**Why multi-language for semio**

- Browser requires JavaScript/TypeScript
- Rhino requires C#
- Computation benefits from Python/NumPy
- CLI benefits from Go's single binary

**What it enables**

- Optimal language per domain
- Leverage ecosystem libraries
- Independent team expertise

**What it limits**

- Schema must be kept in sync
- Cross-language debugging
- More build tooling

---

#### 11.3 The Frontend-Backend Split

**Plain explanation**

Every application you use has two parts: what you see and interact with (the frontend), and what happens behind the scenes (the backend). It's like a restaurant: the dining room is the frontend where customers interact, and the kitchen is the backend where food is prepared.

semio's Sketchpad is frontend (React), Engine is backend (Python).

**Technical explanation**

**semio frontend-backend split**:

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    semio FRONTEND-BACKEND SPLIT                          │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  FRONTEND (User's Device):                                              │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  Sketchpad (React + Three.js)                                   │    │
│  │  ├── UI rendering (React components)                            │    │
│  │  ├── State management (XState, Y.js)                            │    │
│  │  ├── 3D visualization (Three.js)                                │    │
│  │  ├── Offline persistence (IndexedDB)                            │    │
│  │  └── API client (fetch, WebSocket)                              │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                           │                                              │
│                           │ HTTP/WebSocket                              │
│                           ▼                                              │
│  BACKEND (Server):                                                      │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  Engine (Python FastAPI)                                        │    │
│  │  ├── GraphQL API                                                 │    │
│  │  ├── Kit validation                                              │    │
│  │  ├── Piece placement computation                                 │    │
│  │  ├── Y.js WebSocket sync                                         │    │
│  │  └── SQLite kit storage                                          │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

**Frontend code** (Sketchpad UI component):

```tsx
// ============================================================
// EXAMPLE 1: FRONTEND CODE - USER INTERFACE IN REACT
// ============================================================
// Purpose: Render 3D view and handle user interactions
// Relates to: Frontend runs in user's browser
// Manages state, handles clicks, displays graphics
//
// js/semio/sketchpad/Design.tsx

export function DesignAppCanvas() {                          // Design canvas component
  
  // ------------------------------------------------------------
  // STATE MANAGEMENT (client-side)
  // ------------------------------------------------------------
  
  const [pieces, setPieces] = useSyncField(                  // Sync with Y.js store
    designStore,                                             // Source store
    "pieces"                                                 // Field to sync
  );
  
  const [camera, setCamera] = useDesignAppCamera();          // 3D camera position
  const [selection, setSelection] = useDesignAppSelection(); // Selected pieces
  
  // ------------------------------------------------------------
  // 3D RENDERING (WebGL via Three.js)
  // ------------------------------------------------------------
  
  return (
    <Canvas camera={camera}>                                 {/* Three.js canvas */}
      <Suspense fallback={<Loading3D />}>                    {/* Loading state */}
        {pieces.map(piece => (                               /* Render each piece */
          <PieceGeometry                                     /* 3D geometry component */
            key={piece.guid}                                 /* React key */
            piece={piece}                                    /* Piece data */
            selected={selection.pieces.has(piece.guid)}      /* Is selected? */
            onSelect={() => setSelection({                   /* Click handler */
              pieces: new Set([piece.guid])                  /* Set as selected */
            })}
          />
        ))}
      </Suspense>
    </Canvas>
  );
}

// FRONTEND RESPONSIBILITIES:
// ✓ Render UI components
// ✓ Handle user interactions
// ✓ Manage local state
// ✓ Animate 3D graphics
// ✗ NOT: database access, heavy computation, secrets
```

**Backend code** (Engine API):

```python
# ============================================================
# EXAMPLE 2: BACKEND CODE - SERVER-SIDE LOGIC IN PYTHON
# ============================================================
# Purpose: Handle data storage, computation, and validation
# Relates to: Backend runs on trusted server
# Has database access, performs heavy computation
#
# py/engine/engine.py

# ------------------------------------------------------------
# GRAPHQL API ENDPOINT
# ------------------------------------------------------------

@app.post("/graphql")                                        # POST /graphql route
async def graphql(request: GraphQLRequest) -> GraphQLResponse:
    """Backend: Handle GraphQL queries"""
    
    result = await schema.execute(                           # Execute GraphQL query
        request.query,                                       # Query string
        variable_values=request.variables,                   # Query variables
        context_value={"db": database}                       # Database connection
    )
    
    return GraphQLResponse(                                  # Return response
        data=result.data,                                    # Query results
        errors=result.errors                                 # Any errors
    )

# ------------------------------------------------------------
# REST API ENDPOINT
# ------------------------------------------------------------

@app.get("/kit/{kit_id}")                                    # GET /kit/:id route
async def get_kit(kit_id: str) -> Kit:
    """Backend: Retrieve kit from SQLite"""
    
    async with aiosqlite.connect(                            # Connect to database
        f".semio/kit.db"                                     # SQLite file
    ) as db:
        cursor = await db.execute(                           # Execute SQL query
            "SELECT * FROM kits WHERE guid = ?",             # Query template
            [kit_id]                                         # Parameter (safe)
        )
        row = await cursor.fetchone()                        # Get result row
        return Kit.from_row(row)                             # Convert to Kit object

# BACKEND RESPONSIBILITIES:
# ✓ Database access (SQLite)
# ✓ Heavy computation (piece placement)
# ✓ Data validation
# ✓ Authentication/authorization
# ✗ NOT: rendering UI, handling mouse events
```

**Why semio uses frontend-backend split**

- Sketchpad runs in browser (untrusted)
- Engine runs on server (trusted, compute-heavy)
- Different technologies optimal for each
- Offline-first with sync when online

**What it enables**

- Rich 3D visualization in browser
- Heavy computation on server
- Offline editing capability
- Multiple frontends (web, desktop)

**What it limits**

- Sync complexity
- Offline/online state management
- Data consistency challenges

---

#### 11.4 Databases in Large Systems

**Plain explanation**

At small scale, one database handles everything. At large scale, you need many specialized storage systems.

semio uses SQLite for kit storage, IndexedDB for browser persistence, and file system for 3D assets.

**Technical explanation**

**semio database architecture**:

| Database | Location | Purpose |
|----------|----------|---------|
| SQLite | `.semio/kit.db` | Static kit storage, types, designs |
| IndexedDB | Browser | Y.js document persistence |
| File System | `.semio/files/` | 3D models (.glb), images |
| Memory | Runtime | Y.js live document state |

**SQLite schema** (sql/sqlite/schema.sql):

```sql
-- ============================================================
-- EXAMPLE 1: SQLITE SCHEMA - RELATIONAL DATABASE FOR KITS
-- ============================================================
-- Purpose: Store kit data in a structured, queryable format
-- Relates to: Relational databases, SQL, foreign keys
-- SQLite is embedded (no server) and perfect for local storage
--
-- sql/sqlite/schema.sql

-- ------------------------------------------------------------
-- KITS TABLE: Top-level kit container
-- ------------------------------------------------------------

CREATE TABLE kits (
    guid TEXT PRIMARY KEY,                                   -- Unique identifier
    name TEXT NOT NULL,                                      -- Kit name (required)
    description TEXT,                                        -- Optional description
    version TEXT                                             -- Semantic version
);

-- ------------------------------------------------------------
-- TYPES TABLE: Building block definitions
-- ------------------------------------------------------------

CREATE TABLE types (
    guid TEXT PRIMARY KEY,                                   -- Type unique ID
    kit_guid TEXT REFERENCES kits(guid),                     -- Parent kit (foreign key)
    name TEXT NOT NULL,                                      -- Type name
    parent_guid TEXT REFERENCES types(guid)                  -- Parent type (hierarchy)
);

-- ------------------------------------------------------------
-- DESIGNS TABLE: Assembled configurations
-- ------------------------------------------------------------

CREATE TABLE designs (
    guid TEXT PRIMARY KEY,                                   -- Design unique ID
    kit_guid TEXT REFERENCES kits(guid),                     -- Parent kit
    name TEXT NOT NULL,                                      -- Design name
    parent_guid TEXT REFERENCES designs(guid)                -- Parent design (hierarchy)
);

-- ------------------------------------------------------------
-- PIECES TABLE: Instances in a design
-- ------------------------------------------------------------

CREATE TABLE pieces (
    guid TEXT PRIMARY KEY,                                   -- Piece unique ID
    design_guid TEXT REFERENCES designs(guid),               -- Parent design
    type_guid TEXT REFERENCES types(guid),                   -- Type this piece uses
    name TEXT,                                               -- Optional name
    plane_origin_x REAL,                                     -- Position X
    plane_origin_y REAL,                                     -- Position Y
    plane_origin_z REAL                                      -- Position Z
);

-- ------------------------------------------------------------
-- CONNECTIONS TABLE: Links between pieces
-- ------------------------------------------------------------

CREATE TABLE connections (
    guid TEXT PRIMARY KEY,                                   -- Connection unique ID
    design_guid TEXT REFERENCES designs(guid),               -- Parent design
    connected_piece_guid TEXT REFERENCES pieces(guid),       -- First piece
    connecting_piece_guid TEXT REFERENCES pieces(guid),      -- Second piece
    gap REAL,                                                -- Gap distance
    shift REAL,                                              -- Lateral shift
    rotation REAL                                            -- Rotation angle
);

-- BENEFITS OF RELATIONAL MODEL:
-- ✓ Foreign keys ensure data integrity
-- ✓ JOINs enable complex queries
-- ✓ Indexes make lookups fast
-- ✓ ACID transactions prevent corruption
```

**IndexedDB for Y.js persistence**:

```typescript
// ============================================================
// EXAMPLE 2: INDEXEDDB - BROWSER-NATIVE PERSISTENCE
// ============================================================
// Purpose: Save Y.js documents to browser storage
// Relates to: Offline-first design, client-side storage
// IndexedDB survives browser restarts, enabling offline work
//
// js/semio/sketchpad/Sketchpad.tsx

import { IndexeddbPersistence } from 'y-indexeddb';          // Y.js IndexedDB adapter

function createKitStore(kitGuid: Guid): KitStore {           // Create a new kit store
  const yDoc = new Y.Doc();                                  // Create Y.js document
  
  // ------------------------------------------------------------
  // PERSIST TO INDEXEDDB
  // ------------------------------------------------------------
  
  const persistence = new IndexeddbPersistence(              // Create persistence layer
    `semio-kit-${kitGuid}`,                                  // Storage key (unique per kit)
    yDoc                                                     // Document to persist
  );
  
  // Every change to yDoc is automatically saved to IndexedDB
  // On page reload, yDoc is automatically restored from IndexedDB
  
  // ------------------------------------------------------------
  // Y.JS DOCUMENT STRUCTURE
  // ------------------------------------------------------------
  
  const yTypes = yDoc.getArray<Y.Map<any>>('types');         // Array of type maps
  const yDesigns = yDoc.getArray<Y.Map<any>>('designs');     // Array of design maps
  
  return new KitStore(yDoc, yTypes, yDesigns);               // Return store wrapper
}

// TIMELINE:
// 1. User opens Sketchpad → yDoc created
// 2. IndexeddbPersistence checks if data exists in IndexedDB
// 3. If exists: loads previous state (offline edits restored!)
// 4. User makes edits → yDoc updates → IndexedDB saves automatically
// 5. User closes browser → data persists
// 6. User reopens → step 2 restores state
```

**File storage for assets**:

```typescript
// ============================================================
// EXAMPLE 3: FILE STORAGE - BINARY ASSETS ON DISK
// ============================================================
// Purpose: Store 3D models, images, and other binary files
// Relates to: File system storage, blob handling
// Large binary files don't fit well in databases
//
// js/semio/sketchpad/Sketchpad.tsx

class FileProvider {                                         // File operations abstraction
  
  // ------------------------------------------------------------
  // UPLOAD: Save a blob to the file system
  // ------------------------------------------------------------
  
  async upload(                                              // Upload method
    kitId: string,                                           // Kit identifier
    fileId: string,                                          // File identifier
    path: string,                                            // Relative path
    blob: Blob                                               // Binary data
  ): Promise<string> {
    
    const filePath = `.semio/files/${kitId}/${fileId}/${path}`;  // Construct path
    
    await fs.writeFile(                                      // Write to disk
      filePath,
      Buffer.from(await blob.arrayBuffer())                  // Convert Blob to Buffer
    );
    
    return filePath;                                         // Return saved path
  }
  
  // ------------------------------------------------------------
  // DOWNLOAD: Read a blob from the file system
  // ------------------------------------------------------------
  
  async download(                                            // Download method
    kitId: string,
    fileId: string,
    path: string
  ): Promise<Blob> {
    
    const filePath = `.semio/files/${kitId}/${fileId}/${path}`;
    
    const buffer = await fs.readFile(filePath);              // Read from disk
    
    return new Blob([buffer]);                               // Convert Buffer to Blob
  }
  
  // ------------------------------------------------------------
  // GET URL: Get path for direct access
  // ------------------------------------------------------------
  
  getUrl(kitId: string, fileId: string, path: string): string {
    return `.semio/files/${kitId}/${fileId}/${path}`;        // Return file path
  }
}

// FILE STORAGE HIERARCHY:
// .semio/
// ├── files/
// │   ├── kit-abc/                                          # Per-kit folder
// │   │   ├── file-123/                                     # Per-file folder
// │   │   │   └── model.glb                                 # Actual 3D model
// │   │   └── file-456/
// │   │       └── texture.png                               # Texture image
```

**Why semio uses multiple storage types**

- SQLite: Optimal for relational kit data
- IndexedDB: Browser-native persistence
- File system: Binary 3D assets don't fit in databases

**What it enables**

- Offline editing with sync
- Fast local queries
- Efficient 3D asset loading

**What it limits**

- Schema migration complexity
- Cross-storage transactions impossible

---

#### 11.5 API Design and Versioning

**Plain explanation**

APIs are the contracts between systems. Like a menu at a restaurant, they define what you can order and how to ask for it. As systems evolve, APIs must change, but old customers still expect their old orders to work.

semio uses JSON for kit exchange and GraphQL for complex queries.

**Technical explanation**

**semio API design**:

| Protocol | Where | Purpose |
|----------|-------|---------|
| JSON | kit.json | Kit import/export |
| GraphQL | Engine | Complex queries |
| MCP | go/mcp | Tool protocol for AI |
| CLI JSON | go/repo | Command output |

**Kit JSON API** (jsonschema/kit.json):

```json
// ============================================================
// EXAMPLE 1: JSON SCHEMA - DEFINING THE KIT FORMAT
// ============================================================
// Purpose: Define the structure of kit.json files
// Relates to: API contracts, data validation
// Any tool that reads/writes kits must follow this schema
//
// jsonschema/kit.json

{
  "$schema": "https://json-schema.org/draft/2020-12/schema", // JSON Schema version
  "$id": "kit.json",                                          // Schema identifier
  "type": "object",                                           // Kit is an object
  
  // ------------------------------------------------------------
  // PROPERTIES: What fields a kit can have
  // ------------------------------------------------------------
  
  "properties": {
    "guid": {                                                 // Unique identifier
      "type": "string",                                       // Must be string
      "format": "uuid"                                        // Must be UUID format
    },
    "name": {                                                 // Kit name
      "type": "string"                                        // Must be string
    },
    "types": {                                                // Building blocks
      "type": "array",                                        // Array of types
      "items": { "$ref": "#/$defs/Type" }                     // Each is a Type
    },
    "designs": {                                              // Assembled designs
      "type": "array",                                        // Array of designs
      "items": { "$ref": "#/$defs/Design" }                   // Each is a Design
    }
  },
  
  // ------------------------------------------------------------
  // REQUIRED: Mandatory fields
  // ------------------------------------------------------------
  
  "required": ["guid", "name"]                                // These must exist
}

// VALIDATION EXAMPLE:
// ✓ { "guid": "abc-123", "name": "Test" }           // Valid (has required)
// ✗ { "name": "Test" }                              // Invalid (missing guid)
// ✗ { "guid": 123, "name": "Test" }                 // Invalid (guid must be string)
```

**GraphQL API** (graphql/semio/schema.graphql):

```graphql
# ============================================================
# EXAMPLE 2: GRAPHQL API - FLEXIBLE QUERIES AND MUTATIONS
# ============================================================
# Purpose: Allow clients to request exactly the data they need
# Relates to: API design, query efficiency
# Unlike REST (fixed endpoints), GraphQL lets clients specify fields
#
# graphql/semio/schema.graphql

# ------------------------------------------------------------
# QUERIES: Read data
# ------------------------------------------------------------

type Query {
  kit(guid: ID!): Kit                                        # Get kit by ID
  type(guid: ID!): Type                                      # Get type by ID
  design(guid: ID!): Design                                  # Get design by ID
  piece(guid: ID!): Piece                                    # Get piece by ID
}

# ------------------------------------------------------------
# MUTATIONS: Write data
# ------------------------------------------------------------

type Mutation {
  # Type operations
  createType(input: CreateTypeInput!): Type!                 # Create new type
  updateType(guid: ID!, input: UpdateTypeInput!): Type!      # Update existing
  deleteType(guid: ID!): Boolean!                            # Delete type
  
  # Design operations
  createDesign(input: CreateDesignInput!): Design!           # Create design
  placePiece(designGuid: ID!, input: PlacePieceInput!): Piece!  # Add piece
}

# ------------------------------------------------------------
# TYPES: Data structures
# ------------------------------------------------------------

type Kit {
  guid: ID!                                                  # Required unique ID
  name: String!                                              # Required name
  types: [Type!]!                                            # All types in kit
  designs: [Design!]!                                        # All designs in kit
}

type Type {
  guid: ID!                                                  # Required unique ID
  name: String!                                              # Required name
  connectors: [Connector!]!                                  # Type's connectors
}

# GRAPHQL QUERY EXAMPLE:
# Client asks for exactly what it needs:
# query {
#   kit(guid: "abc") {
#     name                                                   # Just the name
#     types { name connectors { id } }                       # Types with connector IDs only
#   }
# }
# Server returns exactly that - no over-fetching!
```

**MCP tool protocol** (go/mcp/main.go):

```json
// ============================================================
// EXAMPLE 3: MCP (MODEL CONTEXT PROTOCOL) - AI TOOL INTEGRATION
// ============================================================
// Purpose: Allow AI assistants to call our tools
// Relates to: API design, machine-readable specifications
// MCP defines how AI (like Claude, Copilot) can use our tools
//
// go/mcp/main.go defines tools like:

{
  "name": "analyze",                                         // Tool name
  "description": "Analyze codebase for policy violations",   // What it does (AI reads this)
  "inputSchema": {                                           // What parameters it accepts
    "type": "object",                                        // Input is an object
    "properties": {                                          // Object has properties
      "scope": {                                             // One property: "scope"
        "type": "string",                                    // Scope is a string
        "description": "Scope to analyze (file, folder, or bundle)"  // What scope means
      }
    }
  }
}

// HOW IT WORKS:
// 1. AI assistant reads this schema
// 2. User asks "check my code for problems"
// 3. AI calls: analyze({ scope: "js/semio/" })
// 4. MCP server runs the tool
// 5. Results returned to AI
// 6. AI explains results to user
```

**Why careful API design for semio**

- JSON must be compatible across languages
- GraphQL enables flexible queries
- MCP enables AI tool integration

**What it enables**

- Cross-platform kit exchange
- Flexible data fetching
- AI assistant integration

**What it limits**

- Schema versioning complexity
- Breaking changes need migration

---

#### 11.6 DevOps and Infrastructure

**Plain explanation**

DevOps is everything needed to keep software running in the real world. It's like the difference between building a car and running a taxi service—you need mechanics, gas stations, dispatch systems, and safety inspections.

semio uses GitHub Actions for CI/CD and Nx for build orchestration.

**Technical explanation**

**semio CI/CD pipeline**:

```yaml
# ============================================================
# EXAMPLE 1: GITHUB ACTIONS - AUTOMATED DEPLOYMENT
# ============================================================
# Purpose: Automatically build and deploy when code changes
# Relates to: DevOps, continuous integration/deployment
# This runs in the cloud every time we push to main branch
#
# .github/workflows/gh-pages.yml

name: Deploy Documentation                                   # Workflow name

on:                                                          # WHEN to run
  push:                                                      # When code is pushed
    branches: [main]                                         # Only on main branch

jobs:                                                        # WHAT to do
  deploy:                                                    # Job name
    runs-on: ubuntu-latest                                   # Run on Linux server
    steps:                                                   # Steps in order
    
      # STEP 1: Get the code
      - uses: actions/checkout@v4                            # Download our repo
      
      # STEP 2: Set up Node.js
      - uses: actions/setup-node@v4                          # Install Node.js
        with:
          node-version: '22'                                 # Version 22
      
      # STEP 3: Install dependencies
      - name: Install dependencies                           # Step name
        run: npm ci                                          # Run npm clean install
      
      # STEP 4: Run checks
      - name: Run preflight checks                           # Run all checks
        run: npm run preflight                               # Formatting, linting, types
      
      # STEP 5: Build docs
      - name: Build documentation                            # Build the docs site
        run: npm run build -- --projects=@semio/docs         # Just docs package
      
      # STEP 6: Deploy to web
      - name: Deploy to GitHub Pages                         # Upload to hosting
        uses: peaceiris/actions-gh-pages@v3                  # Deploy action
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}          # Auth token
          publish_dir: ./js/docs/dist                        # Folder to deploy

# FLOW:
# Developer pushes code → GitHub runs this → Website updates automatically
# No manual deployment needed!
```

**Build orchestration** (nx.json):

```json
// ============================================================
// EXAMPLE 2: NX BUILD ORCHESTRATION - SMART BUILDS
// ============================================================
// Purpose: Build packages in correct order, cache results
// Relates to: Build systems, dependency management
// Nx understands which packages depend on which
//
// nx.json

{
  "targetDefaults": {                                        // Default settings for targets
    "build": {                                               // For "build" commands
      "dependsOn": ["^build"],                               // First build dependencies (^)
      "cache": true                                          // Cache results
    },
    "test": {                                                // For "test" commands
      "dependsOn": ["build"],                                // First run build
      "cache": true                                          // Cache test results
    }
  },
  "plugins": [                                               // Nx plugins
    "@nx/js"                                                 // JavaScript plugin
  ]
}

// HOW IT WORKS:
// 1. Run: npm run build
// 2. Nx finds all packages
// 3. Builds dependencies first (@semio/js before @semio/sketchpad)
// 4. Caches everything
// 5. Next build: skip unchanged packages
//
// EXAMPLE BUILD ORDER:
// @semio/assets (no deps) → @semio/js (depends on assets) →
// @semio/sketchpad (depends on js) → @semio/docs (depends on all)
```

**Package distribution**:

| Platform | Package | Registry |
|----------|---------|----------|
| JavaScript | @semio/js | npm |
| Python | semio-engine | PyPI |
| .NET | Semio | NuGet |
| Grasshopper | Semio | Yak (McNeel) |
| VS Code | semio | VS Code Marketplace |

**Why DevOps for semio**

- Multi-language requires coordinated builds
- Multiple registries need automated publishing
- Cross-platform testing ensures consistency

**What it enables**

- Automated publishing to all registries
- Consistent builds across platforms
- Fast feedback on changes

**What it limits**

- Complex build configuration
- Registry-specific quirks

---

#### 11.7 Security Considerations

**Plain explanation**

Security in software means protecting against bad actors who try to steal data, break systems, or misuse resources. It's like a building's security—locks, cameras, access cards, and guards working together.

semio uses input validation, authentication, and HTTPS.

**Technical explanation**

**semio security layers**:

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    semio SECURITY LAYERS                                 │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  1. INPUT VALIDATION (Zod schemas):                                     │
│     kitSchema.parse(userInput)  // Validate before processing           │
│                                                                          │
│  2. AUTHENTICATION:                                                     │
│     JWT tokens for API access                                           │
│     GitHub OAuth for user identity                                      │
│                                                                          │
│  3. AUTHORIZATION:                                                      │
│     Kit ownership checks                                                │
│     Read/write permissions                                              │
│                                                                          │
│  4. TRANSPORT:                                                          │
│     HTTPS for all network traffic                                       │
│     WSS for WebSocket connections                                       │
│                                                                          │
│  5. STORAGE:                                                            │
│     SQLite file permissions                                             │
│     IndexedDB browser isolation                                         │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

**Input validation** (Zod):

```typescript
// ============================================================
// EXAMPLE 1: INPUT VALIDATION - PROTECTING AGAINST BAD DATA
// ============================================================
// Purpose: Ensure user input matches expected format
// Relates to: Security, type safety, error prevention
// Zod validates at runtime (TypeScript only checks at compile time)
//
// js/semio/semio.ts

import { z } from 'zod';                                     // Import Zod validation library

// DEFINE EXPECTED SHAPE:
// This "schema" describes what a valid Piece looks like
export const pieceSchema = z.object({                        // Piece is an object
  guid: z.string().uuid(),                                   // guid: must be valid UUID string
  name: z.string().optional(),                               // name: optional string
  typeGuid: z.string().uuid(),                               // typeGuid: must be valid UUID
  plane: planeSchema.optional(),                             // plane: optional, uses another schema
  center: pointSchema.optional()                             // center: optional point
});

// VALIDATE INPUT:
// Takes unknown data, returns valid Piece or throws error
function validatePiece(input: unknown): Piece {              // unknown = could be anything
  return pieceSchema.parse(input);                           // Parse validates and transforms
  // If input is { guid: "not-a-uuid" }:
  //   Throws: "Invalid uuid"
  // If input is { guid: "abc-123-...", typeGuid: "xyz-456-..." }:
  //   Returns typed Piece object
}

// WHY THIS MATTERS:
// Without validation, malicious input could:
// - Inject SQL: { name: "'; DROP TABLE pieces; --" }
// - Cause crashes: { guid: null }
// - Corrupt data: { typeGuid: "invalid" }
```

**Kit validation**:

```typescript
// ============================================================
// EXAMPLE 2: KIT VALIDATION - BUSINESS RULE ENFORCEMENT
// ============================================================
// Purpose: Ensure kit data follows business rules (not just format)
// Relates to: Data integrity, domain validation
// This catches semantic errors like duplicate GUIDs
//
// js/semio/semio.ts

export function validateKit(kit: Kit): ValidationResult {   // Takes kit, returns problems
  const problems: Problem[] = [];                           // Collect all problems
  
  // GUID UNIQUENESS CHECK:
  // Every entity in semio must have a unique GUID
  const guids = new Set<string>();                          // Track seen GUIDs
  
  for (const type of kit.types) {                           // Check each type
    if (guids.has(type.guid)) {                             // Already seen this GUID?
      problems.push({                                        // Report problem
        constraintId: 'guid-unique',                         // Problem type
        severity: 'error',                                   // How serious
        message: `Duplicate GUID: ${type.guid}`,             // Human message
        location: { entityKind: 'Type', entityGuid: type.guid }  // Where
      });
    }
    guids.add(type.guid);                                    // Remember this GUID
  }
  
  // Similar checks for designs, pieces, connections...
  
  return { problems };                                       // Return all found problems
}

// USAGE:
// const result = validateKit(importedKit);
// if (result.problems.length > 0) {
//   console.error("Kit has problems:", result.problems);
// }
//
// DIFFERENCE FROM ZOD:
// - Zod: Is this valid JSON with correct types?
// - validateKit: Does this make sense as a kit?
```

**Why security matters for semio**

- Collaborative editing exposes data to network
- User-uploaded 3D models could be malicious
- Kit sharing requires access control

**What it enables**

- Safe collaborative editing
- Trusted kit exchange
- Protected user data

**What it limits**

- Development overhead
- Complexity in offline scenarios

---

#### 11.8 Documentation as Code

**Plain explanation**

In large systems, documentation becomes code itself—automatically generated, version-controlled, and tested. Like how blueprints and building codes must stay synchronized with actual construction.

semio generates documentation from code and validates it in CI.

**Technical explanation**

**semio documentation layers**:

| Type | Source | Output |
|------|--------|--------|
| Dev docs | AGENTS.md, README.md | Developer onboarding |
| User docs | js/docs (MDX) | semio.dev website |
| API docs | GraphQL introspection | GraphQL Playground |
| Schema docs | jsonschema/*.json | JSON Schema reference |
| Inline | TypeScript types | Type tooltips in IDE |

**MDX documentation** (js/docs):

```mdx
---
title: Kit of Parts Architecture
description: Understanding semio's approach to modular design
---

import { Aside, Steps } from '@/components/docs';

# Kit of Parts Architecture

semio is built on the concept of **kit-of-parts architecture**, 
where designs are composed from reusable types.

<Aside type="tip">
A kit is like a box of LEGO pieces—each type can be combined
with others through their connectors.
</Aside>

<Steps>
1. Define types with connectors
2. Create designs that place types
3. Connect pieces via their connectors
</Steps>
```

**Generated schemas**:

```bash
# ============================================================
# EXAMPLE 1: SCHEMA GENERATION - SINGLE SOURCE OF TRUTH
# ============================================================
# Purpose: Generate all schemas from one source
# Relates to: Documentation as code, consistency
# One TypeScript definition generates everything else
#
# py/engine/generate-schemas.ts

npx tsx py/engine/generate-schemas.ts

# INPUT:
# - js/semio/semio.ts (TypeScript types)
#
# OUTPUT:
# - jsonschema/kit.json     (for input validation)
# - graphql/semio/schema.graphql (for API)
# - sql/sqlite/schema.sql   (for database)
#
# WHY:
# - Change TypeScript once
# - All other schemas update automatically
# - No manual synchronization needed
# - Documentation always matches code
```

**Documentation validation** (hooks/code.ts):

```typescript
// ============================================================
// EXAMPLE 2: DOCUMENTATION VALIDATION - KEEP DOCS ACCURATE
// ============================================================
// Purpose: Automatically check that docs match reality
// Relates to: Documentation as code, continuous integration
// If docs mention a file that doesn't exist, that's an error
//
// hooks/code.ts

function validateDocumentation(): ValidationReport {        // Returns list of problems
  const agents = parseMarkdown('AGENTS.md');                // Parse AGENTS.md content
  const readme = parseMarkdown('README.md');                // Parse README.md content
  
  const problems: Problem[] = [];                           // Collect all issues
  
  // CHECK THAT DOCUMENTED FILES EXIST:
  // If AGENTS.md says "see js/semio/foo.ts" but foo.ts doesn't exist...
  for (const ref of agents.fileReferences) {                // Each file reference
    if (!fileExists(ref.path)) {                            // Does file actually exist?
      problems.push({                                        // No - report problem
        id: 'doc:missing-file',                              // Problem type
        message: `Documented file does not exist: ${ref.path}`  // What's wrong
      });
    }
  }
  
  // RUNS IN CI:
  // npm run preflight → hooks/code.ts → finds outdated docs
  // Blocks merge if docs are wrong
  
  return { problems };                                       // Return all found problems
}

// EXAMPLE OUTPUT:
// [
//   { id: 'doc:missing-file', message: 'Documented file does not exist: js/old/file.ts' },
//   { id: 'doc:wrong-function', message: 'Function createKit no longer exists in semio.ts' }
// ]
```

**Why docs-as-code for semio**

- Schema changes must update docs
- Multi-language requires synchronized docs
- Onboarding developers requires current docs

**What it enables**

- Always-current documentation
- Version-controlled changes
- Automated validation

**What it limits**

- More tooling to maintain
- Documentation in code

---

#### 11.9 Legacy Systems and Technical Debt

**Plain explanation**

Every codebase accumulates "technical debt"—shortcuts, outdated patterns, and workarounds that made sense at the time but complicate future work. Like a house where each renovation added quirks that the next owner must work around.

semio manages debt through explicit refactoring plans and gradual migration.

**Technical explanation**

**semio technical debt tracking**:

| Type | Location | Status |
|------|----------|--------|
| Refactor plans | plans/*.md | Active tracking |
| Legacy patterns | Commented code | Keep for reference |
| Migration status | AGENTS.md | Current state |
| TODO tickets | tickets/ | Specific tasks |

**Refactoring documentation** (plans/REFACTOR-PLAN-SKETCHPAD.md):

```markdown
# ============================================================
# EXAMPLE 1: REFACTOR PLAN - DOCUMENTING TECHNICAL DEBT
# ============================================================
# Purpose: Track what needs to change and how
# Relates to: Technical debt, gradual improvement
# Writing it down ensures everyone knows the plan
#
# plans/REFACTOR-PLAN-SKETCHPAD.md

# Sketchpad Refactor Plan

## Current State                        <!-- What we have now -->
- Mixed XState and Zustand state management  <!-- Problem: two systems -->
- Inconsistent hook patterns                 <!-- Problem: confusing API -->
- Legacy uiMachine still referenced          <!-- Problem: dead code -->

## Target State                         <!-- What we want -->
- Unified XState for all UI state            <!-- Goal: one system -->
- Triadic hook pattern [value, setter, canSet]  <!-- Goal: consistent API -->
- Single sketchpadMachine                    <!-- Goal: clean code -->

## Migration Steps                      <!-- How to get there -->
1. [ ] Consolidate to single machine         <!-- Step 1: merge machines -->
2. [ ] Remove uiMachine references           <!-- Step 2: delete old code -->
3. [ ] Update all hooks to triadic pattern   <!-- Step 3: fix hooks -->
4. [ ] Remove Zustand dependencies           <!-- Step 4: cleanup -->

<!-- WHY DOCUMENT THIS:
     - New devs understand the plan
     - Work can be divided across team
     - Progress is trackable
     - Prevents conflicting changes -->
```

**Gradual migration pattern**:

```typescript
// ============================================================
// EXAMPLE 2: GRADUAL MIGRATION - CHANGING SAFELY
// ============================================================
// Purpose: Replace old patterns without breaking everything
// Relates to: Technical debt, backwards compatibility
// Keep both old and new working while you migrate
//
// js/semio/sketchpad/Design.tsx

// STEP 1: CREATE NEW PATTERN ALONGSIDE OLD
export function useDesignAppSelection(): HookResult<DesignSelection> {
  // NEW: XState-based (the target pattern)
  const actor = useSketchpadActor();                         // Get XState actor
  const selection = useSelector(actor, selectDesignSelection);  // Read from XState
  
  // OLD: Legacy zustand (kept for reference during migration)
  // const selection = useDesignStore(s => s.selection);     // COMMENTED OUT
  // ^ This is the old way - keeping it here shows what we're replacing
  
  const setSelection = useCallback((sel: DesignSelection) => {
    actor.send({ type: 'DESIGN.SET_SELECTION', selection: sel });  // XState way
  }, [actor]);
  
  const canSet = true;                                       // Can always set selection
  
  return [selection, setSelection, canSet];                  // Return triadic tuple
}

// STEP 2: UPDATE CONSUMERS ONE BY ONE
// - Find all places using the old pattern
// - Update each to use the new hook
// - Test after each change

// STEP 3: REMOVE OLD IMPLEMENTATION
// - Once all consumers updated, delete old code
// - Remove old dependencies (zustand)
// - Delete commented legacy code
```

**Ticket system for debt**:

```yaml
# ============================================================
# EXAMPLE 3: TICKET TRACKING - MANAGING WORK
# ============================================================
# Purpose: Break refactoring into trackable tasks
# Relates to: Project management, technical debt
# Each ticket is a small, completable unit of work
#
# tickets/2025/06/15/HOOK-MIGRATION/ticket.md

---
slug: HOOK-MIGRATION                                         # Ticket identifier
status: open                                                 # Still in progress
summary: Migrate legacy hooks to triadic pattern             # One-line description
---

## Tasks                                                     # Checklist

- [ ] Update useDesignAppSelection                           # Design app hook
      # Change from: zustand store access
      # Change to:   XState selector
      
- [ ] Update useTypeAppSelection                             # Type app hook
      # Same pattern as design
      
- [ ] Update useKitAppSelection                              # Kit app hook
      # Same pattern as design
      
- [ ] Remove deprecated hook exports                         # Cleanup
      # Delete old exports from index.ts
      # Update any remaining consumers

# WORKFLOW:
# 1. Developer claims ticket
# 2. Does one task at a time
# 3. Checks off completed tasks
# 4. Once all done: closes ticket
#
# WHY TICKETS:
# - Large refactors become manageable
# - Progress is visible
# - Multiple people can work together
# - Nothing gets forgotten
```

**Why managing debt matters for semio**

- Multi-language increases surface area
- Fast iteration creates shortcuts
- Team changes leave undocumented patterns

**What it enables**

- Sustainable long-term development
- Clear migration paths
- Team continuity

**What it limits**

- Slows new feature development
- Requires ongoing maintenance

---

## Part 6: Semio as a Case Study

### Chapter 12: Understanding Semio

#### 12.1 What Semio Is

**Plain explanation**

Semio is software for designing buildings and structures using "kits of parts"—like grown-up LEGO. Instead of designing every wall, beam, and window from scratch, architects define reusable components (types) and then assemble them into designs. The types know how to connect to each other.

Imagine designing a house by saying "I want this room module here, this bathroom module there, connected by this hallway module" rather than drawing every line. Semio manages the components and their relationships.

**Technical explanation**

Semio is a **design information modeling** system with:

**Core domain model**:

```
Kit (container for everything)
├── Types (reusable components)
│   ├── Connectors (how types attach)
│   ├── Models (3D representations)
│   └── Attributes (metadata)
├── Designs (assemblies of types)
│   ├── Pieces (instances of types)
│   ├── Connections (how pieces attach)
│   └── Layers (organization)
├── Qualities (measurements/standards)
└── Files (associated documents)
```

**System architecture**:

```
┌─────────────────────────────────────────────────────────────┐
│                      SEMIO ECOSYSTEM                         │
│                                                              │
│  ┌─────────────────────────────────────────────────────┐    │
│  │                 SKETCHPAD (Frontend)                │    │
│  │  React + TypeScript + XState + Y.js                 │    │
│  │  Web / Desktop (Electron)                           │    │
│  └─────────────────────────────────────────────────────┘    │
│                            │                                 │
│                       HTTP/gRPC                              │
│                            │                                 │
│  ┌─────────────────────────────────────────────────────┐    │
│  │                  ENGINE (Backend)                   │    │
│  │  Python + FastAPI + SQLite                          │    │
│  │  Validation, computation, storage                   │    │
│  └─────────────────────────────────────────────────────┘    │
│                            │                                 │
│  ┌─────────────────────────────────────────────────────┐    │
│  │            INTEGRATIONS (Plugins)                   │    │
│  │  Grasshopper (C#), Rhino, Revit                     │    │
│  └─────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
```

**Repository structure** (multi-language monorepo):

```
semio/
├── js/          # TypeScript/JavaScript
│   ├── semio/   # Core domain models (@semio/js)
│   ├── sketchpad/
│   ├── vscode/  # VS Code extension
│   └── docs/    # Documentation
├── py/          # Python
│   └── engine/  # Backend engine (@semio/engine)
├── net/         # C#/.NET
│   ├── Semio/   # Core library
│   └── Semio.Grasshopper/  # Grasshopper plugin
├── go/          # Go
│   ├── repo/    # Repository CLI
│   └── mcp/     # Model Context Protocol server
└── assets/      # Shared assets
```

**Why Semio was built**

Architecture firms waste time:

- Redrawing similar components repeatedly
- Manually tracking how parts connect
- Losing design knowledge when projects end
- Converting between software formats

Semio captures design knowledge in reusable kits, making past work useful for future projects.

**What it enables**

- Rapid design iteration with proven components
- Consistent design language across projects
- Automatic validation of connections
- Multi-platform collaboration (web, desktop, CAD)
- Version control for design components
- AI assistance for design exploration

**What it limits**

- Learning curve for kit-of-parts thinking
- Initial investment to create type libraries
- Less flexibility than freeform design
- Complex setup for enterprise deployment
- Multi-language codebase requires diverse skills

---

#### 12.2 The Semio Architecture

**Plain explanation**

Semio is built like a well-organized company. Different departments handle different jobs: the frontend team handles the visual interface (Sketchpad), the backend team handles data processing (Engine), and the integration team connects to external tools (Grasshopper, Rhino). They all share a common language (the domain model) so they understand each other.

The architecture separates concerns so each part can evolve independently and teams with different expertise can work in parallel.

**Technical explanation**

**Multi-language monorepo architecture**:

```
┌─────────────────────────────────────────────────────────────┐
│                    DOMAIN LAYER                              │
│     (Same concepts in each language)                        │
│                                                              │
│   js/semio/semio.ts   py/engine/    net/Semio/Semio.cs     │
│   ────────────────────────────────────────────────────────  │
│   Kit, Type, Design, Piece, Connection, Connector           │
│   (Synchronized via JSON schemas + code generation)          │
└─────────────────────────────────────────────────────────────┘
                           │
           ┌───────────────┼───────────────┐
           ▼               ▼               ▼
┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐
│   FRONTEND      │ │    BACKEND      │ │  INTEGRATIONS   │
│                 │ │                 │ │                 │
│ js/sketchpad/   │ │ py/engine/      │ │ net/Semio.      │
│                 │ │                 │ │ Grasshopper/    │
│ React           │ │ FastAPI         │ │                 │
│ TypeScript      │ │ Python          │ │ C#              │
│ XState          │ │ SQLite          │ │ Rhino/GH APIs   │
│ Y.js            │ │                 │ │                 │
│                 │ │ Validation      │ │ CAD Geometry    │
│ UI Rendering    │ │ Computation     │ │ Native Plugins  │
│ State Mgmt      │ │ Persistence     │ │                 │
│ Collaboration   │ │                 │ │                 │
└─────────────────┘ └─────────────────┘ └─────────────────┘
           │               │               │
           └───────────────┼───────────────┘
                           ▼
┌─────────────────────────────────────────────────────────────┐
│                  SHARED INFRASTRUCTURE                       │
│                                                              │
│   JSON Schemas      SQL Schemas       GraphQL Schema        │
│   (jsonschema/)     (sql/)            (graphql/)            │
│                                                              │
│   Code generation: TypeScript → JSON Schema → Python/C#     │
└─────────────────────────────────────────────────────────────┘
```

**Key architectural decisions**:

| Decision | Rationale |
|----------|-----------|
| **Monorepo** | Single source of truth, atomic changes across languages |
| **Multi-language** | Best tool for each job (TS for UI, Python for ML, C# for CAD) |
| **Schema-first** | TypeScript defines truth, generates other languages |
| **Event-driven UI** | XState provides predictable, debuggable state management |
| **CRDT collaboration** | Y.js enables real-time multi-user editing |
| **Plugin architecture** | Apps register via plugins, no core modifications |

**Build and development**:

```bash
# Nx manages the monorepo
npm run dev          # Start all development servers
npm run test         # Run all tests
npm run build        # Build all packages

# Language-specific
npm run dev:js       # JavaScript packages
npm run dev:engine   # Python engine
npm run build:net    # .NET packages
```

**Why these design choices**

Different parts of Semio have different requirements:

- **UI** needs fast iteration → TypeScript/React
- **Computation** needs libraries → Python
- **CAD plugins** need platform access → C#
- **CLI tools** need fast startup → Go

A monorepo keeps them synchronized despite different languages.

**What it enables**

- Parallel development by specialized teams
- Independent deployment of components
- Shared domain model prevents drift
- Comprehensive testing across boundaries
- Gradual migration and experimentation
- Clear separation of concerns

**What it limits**

- Complex build system
- Requires expertise in multiple languages
- Schema synchronization overhead
- Larger repository size
- CI/CD complexity
- Onboarding takes longer

---

#### 12.3 The Frontend: Sketchpad

**Plain explanation**

Sketchpad is the visual application where users actually design. It's like a sophisticated drawing program specifically built for kit-of-parts architecture. Users can browse component libraries, drag types onto a canvas, connect them, and see their design in 2D diagrams and 3D views simultaneously.

Sketchpad runs in web browsers and as a desktop application, syncing changes in real-time when multiple people collaborate.

**Technical explanation**

**Sketchpad architecture**:

```
┌─────────────────────────────────────────────────────────────┐
│                    SKETCHPAD SHELL                           │
│                                                              │
│  ┌─────────────────────────────────────────────────────┐    │
│  │                     NAVBAR                          │    │
│  │  Navigation, panel toggles, global actions          │    │
│  └─────────────────────────────────────────────────────┘    │
│                                                              │
│  ┌─────────────────────────────────────────────────────┐    │
│  │                     CANVAS                          │    │
│  │                                                     │    │
│  │   ┌─────────┐  ┌─────────┐  ┌─────────────────┐     │    │
│  │   │ Panel   │  │ Window  │  │ Window          │     │    │
│  │   │ (Left)  │  │ (Scene) │  │ (Diagram/Table) │     │    │
│  │   │         │  │         │  │                 │     │    │
│  │   │ Tools   │  │  3D     │  │  2D Graph       │     │    │
│  │   │ Details │  │  View   │  │  or Data Table  │     │    │
│  │   └─────────┘  └─────────┘  └─────────────────┘     │    │
│  │                                                     │    │
│  └─────────────────────────────────────────────────────┘    │
│                                                              │
│  ┌─────────────────────────────────────────────────────┐    │
│  │                     FOOTER                          │    │
│  │  Status, model tags, context actions                │    │
│  └─────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
```

**State management with XState**:

```typescript
// ============================================================
// EXAMPLE 1: XSTATE STATE MACHINE - PREDICTABLE UI STATE
// ============================================================
// Purpose: Manage all UI state in one predictable system
// Relates to: State management, finite state machines
// XState ensures only valid state transitions happen
//
// js/semio/sketchpad/Sketchpad.tsx

// SINGLE SOURCE OF TRUTH for all UI state
const sketchpadMachine = createMachine({
  id: 'sketchpad',                                           // Machine name
  type: 'parallel',                                          // Multiple things happen at once
  context: {                                                 // All state data lives here
    theme: 'system',                                         // Light/dark/system
    language: 'en',                                          // Current language
    expertise: 'normal',                                     // User skill level
    // App-specific state slices
    homeApp: { ... },                                        // Home screen state
    kitApp: { ... },                                         // Kit browser state
    designApp: { ... },                                      // Design editor state
    typeApp: { ... },                                        // Type editor state
  },
  states: {                                                  // Possible states
    navigation: {                                            // Navigation region
      states: {
        home: { ... },                                       // At home screen
        kit: { ... },                                        // Browsing kit
        design: { ... },                                     // Editing design
        type: { ... },                                       // Editing type
      }
    }
  }
});

// READING STATE:
// Components read state via selectors (like database queries)
const selection = useSelector(actor, (s) => s.context.designApp.selection);
// ^ Only re-renders when selection actually changes

// MODIFYING STATE:
// Components send events (like commands)
actor.send({ type: 'DESIGN.SELECT_PIECE', pieceGuid: '...' });
// ^ Machine decides if this is valid, then updates state
```

**Real-time collaboration with Y.js**:

```typescript
// ============================================================
// EXAMPLE 2: Y.JS CRDT - CONFLICT-FREE COLLABORATION
// ============================================================
// Purpose: Allow multiple users to edit simultaneously
// Relates to: Real-time systems, distributed computing
// Y.js is a CRDT (Conflict-free Replicated Data Type)
//
// js/semio/sketchpad/Sketchpad.tsx

// KIT DATA stored in Y.js documents (not in XState)
const yDoc = new Y.Doc();                                    // Create Y.js document
const yTypes = yDoc.getArray('types');                       // Get/create types array

// CHANGES SYNC AUTOMATICALLY to all connected users
yTypes.push([newType]);                                      // Add a type
// ^ Other users see this immediately (within milliseconds)
// No server needed for this - direct peer-to-peer sync

// CONFLICT RESOLUTION IS AUTOMATIC
// Scenario:
// - User A adds type at index 0  (timestamp: 10:00:00.001)
// - User B adds type at index 0  (timestamp: 10:00:00.002)
// - These happen "simultaneously" on different computers
//
// Without CRDT: One change would overwrite the other
// With Y.js: BOTH types added, order is deterministic
//
// Result: [UserA's type, UserB's type] - same on both computers

// WHY THIS MATTERS FOR ARCHITECTURE:
// - Two architects can work on same design
// - No "file is locked" messages
// - No merge conflicts
// - Changes visible instantly
```

**Why Sketchpad is built this way**

Design applications need:

- **Immediate feedback**: XState ensures predictable state updates
- **Multiple views**: Same data shown as 3D scene, 2D diagram, and table
- **Collaboration**: Y.js provides conflict-free real-time sync
- **Cross-platform**: React works in browser and Electron
- **Extensibility**: Plugin architecture for adding new apps

**What it enables**

- Rich, responsive design experience
- Real-time multi-user editing
- Cross-platform (web + desktop)
- Predictable, debuggable state management
- Multiple synchronized views of same data
- Undo/redo across all operations

**What it limits**

- Complex state machine can be hard to debug
- Y.js learning curve
- Bundle size for web version
- Electron adds desktop overhead
- Real-time sync requires connection
- Performance with very large designs

---

#### 12.4 The Backend: The Engine

**Plain explanation**

The Engine is Semio's brain—it handles all the complex calculations, stores data permanently, validates designs, and coordinates with external tools. While the Sketchpad shows you pretty pictures and lets you drag things around, the Engine makes sure everything is correct and saves your work.

It's like the kitchen in a restaurant: you don't see it from the dining room, but it's where the real work happens.

**Technical explanation**

**Engine responsibilities**:

```
┌─────────────────────────────────────────────────────────────┐
│                        ENGINE                                │
│                                                              │
│  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────┐  │
│  │   API Layer     │  │  Business Logic │  │  Storage    │  │
│  │                 │  │                 │  │             │  │
│  │ FastAPI routes  │  │ Validation      │  │ SQLite DB   │  │
│  │ GraphQL schema  │  │ Computation     │  │ File system │  │
│  │ Auth/authz      │  │ Transformations │  │ .semio zips │  │
│  └─────────────────┘  └─────────────────┘  └─────────────┘  │
│                                                              │
│  ┌─────────────────────────────────────────────────────────┐│
│  │                Schema Generation                        ││
│  │                                                         ││
│  │  TypeScript → JSON Schema → Python types → SQL schema   ││
│  │  (source)     (exchange)    (runtime)     (storage)     ││
│  └─────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────┘
```

**Python implementation**:

```python
# ============================================================
# EXAMPLE 1: FASTAPI - PYTHON WEB FRAMEWORK
# ============================================================
# Purpose: Provide HTTP API for frontend and integrations
# Relates to: Backend architecture, web services
# FastAPI is modern, fast, and auto-generates documentation
#
# py/engine/engine.py

from fastapi import FastAPI                                  # Web framework
from pydantic import BaseModel                               # Data validation

app = FastAPI()                                              # Create the application

# MODELS generated from JSON schemas
# These Python classes match the TypeScript types exactly
class Kit(BaseModel):                                        # Kit data structure
    name: str                                                # Required name
    version: str                                             # Required version
    types: List[Type]                                        # List of types
    designs: List[Design]                                    # List of designs
    # ... Pydantic validates automatically when data arrives

# API ENDPOINT: Validate a kit
@app.post("/kits/validate")                                  # POST to /kits/validate
async def validate_kit(kit: Kit) -> ValidationResult:        # Takes Kit, returns result
    """Validate a kit and return problems with fixes."""     # Doc shown in API docs
    return validate_semio_kit(kit)                           # Run validation
    # If someone sends invalid JSON:
    #   Pydantic rejects it automatically with helpful error

# API ENDPOINT: Get a kit by ID
@app.get("/kits/{kit_id}")                                   # GET /kits/some-kit-id
async def get_kit(kit_id: str) -> Kit:                       # Takes ID, returns Kit
    """Retrieve a kit from storage."""
    return storage.load_kit(kit_id)                          # Load from database
```

**Storage format** (SQLite in .semio ZIP files):

```sql
-- ============================================================
-- EXAMPLE 2: SQLITE SCHEMA - PORTABLE DATABASE
-- ============================================================
-- Purpose: Store kit data in a portable, reliable format
-- Relates to: Data persistence, relational databases
-- Each kit is a ZIP file containing an SQLite database
--
-- sql/sqlite/schema.sql

-- TYPES TABLE: Stores type definitions
CREATE TABLE types (
    guid TEXT PRIMARY KEY,                                   -- Unique identifier
    name TEXT NOT NULL,                                      -- Type name (required)
    variant TEXT,                                            -- Optional variant
    is_virtual INTEGER DEFAULT 0,                            -- 1 = abstract type
    -- ... other fields
);

-- CONNECTORS TABLE: How types attach to each other
CREATE TABLE connectors (
    guid TEXT PRIMARY KEY,                                   -- Unique identifier
    type_guid TEXT REFERENCES types(guid),                   -- Which type owns this
    name TEXT,                                               -- Connector name
    point_x REAL,                                            -- X position
    point_y REAL,                                            -- Y position
    point_z REAL,                                            -- Z position
    -- ... other fields
);

-- WHY SQLITE IN A ZIP:
-- - SQLite = portable, no server needed
-- - ZIP = single file to share
-- - Can include 3D models alongside database
-- - Works offline
-- - Easy to backup and version control
```

**Why this separation matters**

Frontend and backend have different concerns:

| Frontend (Sketchpad) | Backend (Engine) |
|---------------------|------------------|
| Fast rendering | Data integrity |
| User interaction | Validation rules |
| Runs on user device | Runs on server |
| JavaScript/TypeScript | Python |
| Optimized for UX | Optimized for correctness |

Separating them allows independent development, testing, and deployment.

**What it enables**

- Heavy computation off-loaded from browser
- Consistent validation everywhere
- Single source of truth for business rules
- ML/AI integration (Python ecosystem)
- Headless operation for automation
- Reliable data persistence

**What it limits**

- Network latency for operations
- Requires server infrastructure
- Two codebases to maintain
- Data synchronization challenges
- Offline functionality harder
- Deployment complexity

---

#### 12.5 Kits: The Core Data Model

**Plain explanation**

A Kit is a box of LEGO—it contains all the pieces (types) you can use, all the things you've built with them (designs), quality standards, and documentation. Kits are portable: you can share a kit with someone, and they get everything needed to use and modify those designs.

Every Semio project is organized around kits. Small projects might have one kit; large ones might have many kits that reference each other.

**Technical explanation**

**Kit structure**:

```typescript
// ============================================================
// EXAMPLE 1: KIT INTERFACE - THE CONTAINER FOR EVERYTHING
// ============================================================
// Purpose: Define what a kit contains
// Relates to: Domain modeling, data structures
// This is the top-level container in semio
//
// js/semio/semio.ts

interface Kit {
  // IDENTITY - How we find and identify kits
  guid: Guid;              // Unique identifier (never changes)
  name: string;            // Human-readable name
  version: string;         // Semantic version (1.0.0, 1.1.0, etc.)

  // COMPONENTS - The building blocks
  types: Type[];           // Reusable component definitions
  designs: Design[];       // Assemblies using types
  
  // STANDARDS - Rules and measurements
  qualities: Quality[];    // Measurement definitions
  interfaces: Interface[]; // Connector compatibility rules
  
  // ORGANIZATION - Structure and metadata
  files: File[];           // Associated documents (PDFs, images)
  folders: Folder[];       // Organizational structure
  tags: Tag[];             // Categorization labels
  concepts: Concept[];     // Semantic grouping
  authors: Author[];       // Who created/contributed
  
  // METADATA - Extra information
  description?: string;    // Optional description
  icon?: string;           // Optional icon
  image?: string;          // Optional preview image
  attributes: Attribute[]; // Custom key-value pairs
}

// ANALOGY:
// Kit is like a LEGO box:
// - types = the brick designs in the instruction manual
// - designs = the models you build
// - qualities = the standards for brick sizes
// - files = the instruction booklet
// - authors = who designed the set
```

**Kit persistence** (as `.semio` ZIP file):

```
my-project.semio (ZIP archive)
├── .semio/
│   └── kit.db           # SQLite database with all data
├── models/
│   ├── wall-type-a.glb  # 3D model files
│   └── window-type-b.glb
├── docs/
│   └── spec.pdf         # Documentation
└── thumbnails/
    └── preview.png
```

**Kit lifecycle**:

```
┌─────────────────────────────────────────────────────────────┐
│                     KIT LIFECYCLE                            │
│                                                              │
│  CREATE          DEVELOP           PUBLISH         USE      │
│  ───────         ───────           ───────         ───      │
│  New empty       Add types         Version         Import   │
│  kit             Add designs       Export          Reference│
│                  Validate          Share           Extend   │
│                  Test                                        │
│                                                              │
│  ┌─────────┐    ┌─────────┐       ┌─────────┐   ┌─────────┐ │
│  │ Local   │───►│ Local   │──────►│ Remote  │◄──│ Other   │ │
│  │ Empty   │    │ Working │       │ Shared  │   │ Users   │ │
│  └─────────┘    └─────────┘       └─────────┘   └─────────┘ │
└─────────────────────────────────────────────────────────────┘
```

**Why this structure exists**

Kits solve real problems in architectural practice:

- **Reusability**: Types defined once, used in many designs
- **Portability**: Everything in one file, easy to share
- **Versioning**: Track changes over time
- **Composition**: Kits can reference other kits
- **Validation**: Quality definitions enforce standards

**What it enables**

- Self-contained design packages
- Easy sharing between teams
- Library of reusable components
- Version control for design work
- Offline work with local kits
- Consistent standards across projects

**What it limits**

- Kit management overhead
- Large kits can be slow to load
- Merge conflicts when collaborating
- Cross-kit references add complexity
- File size grows with embedded assets
- Requires understanding kit concepts

---

#### 12.6 Types: Reusable Components

**Plain explanation**

A Type is a template for a building component—like a LEGO brick design. It defines what the component looks like (models), how it can connect to other components (connectors), and its properties (attributes). Once you define a "Standard Window" type, you can place 50 of them in a design without redefining each one.

Types are the heart of kit-of-parts design: invest time in defining types well, and design becomes fast assembly.

**Technical explanation**

**Type structure**:

```typescript
// ============================================================
// EXAMPLE 1: TYPE INTERFACE - REUSABLE COMPONENT TEMPLATE
// ============================================================
// Purpose: Define a reusable building component
// Relates to: Domain modeling, component-based design
// Types are templates - pieces are instances of types
//
// js/semio/semio.ts

interface Type {
  guid: Guid;                 // Unique identifier
  name: string;               // e.g., "Standard Wall"
  variant?: string;           // Variations: "Wall" → "Wall-Corner"
  
  // 3D REPRESENTATIONS - How it looks
  models: Model[];            // GLB, OBJ, etc. (multiple for LOD)
  
  // CONNECTION POINTS - How it attaches to others
  connectors: Connector[];    // Where this type can connect
  
  // PROPERTIES - Measurable characteristics
  props: Prop[];              // Width, height, weight, etc.
  
  // FLAGS - Behavioral settings
  isVirtual?: boolean;        // true = abstract, can't use directly
  canScale?: boolean;         // true = can be resized
  canMirror?: boolean;        // true = can be mirrored
  
  // METADATA
  unit?: string;              // e.g., "mm" or "inch"
  availableCount?: number;    // Stock quantity (for inventory)
  location?: Location;        // Where it's physically stored
  authors: Author[];          // Who designed this type
  attributes: Attribute[];    // Custom key-value pairs
}

// ANALOGY:
// Type is like a LEGO brick design:
// - models = the actual plastic shape
// - connectors = the studs and holes
// - props = dimensions (2x4, 2x2)
// - isVirtual = like a category, not a real brick
```

**Connectors define attachment points**:

```typescript
// ============================================================
// EXAMPLE 2: CONNECTOR INTERFACE - HOW TYPES ATTACH
// ============================================================
// Purpose: Define where and how components connect
// Relates to: Graph theory, topology
// Connectors are like USB ports - they define compatibility
//
// js/semio/semio.ts

interface Connector {
  id: string;                 // Unique within type (e.g., "left")
  name?: string;              // Human-readable (e.g., "Left Side")
  
  // POSITION AND ORIENTATION - Where is the connection point?
  point: Point;               // XYZ location on the type
  direction: Vector;          // Which way it faces (outward)
  
  // COMPATIBILITY - What can connect here?
  interface?: Interface;      // Reference to compatibility rules
  mandatory?: boolean;        // true = must be connected
  
  // DIAGRAM POSITIONING - For 2D visualization
  t?: number;                 // Position on ring (0 to 1)
}

// EXAMPLE:
// A room module might have connectors:
// - id: "north"  → point: (0, 5, 0)  → direction: (0, 1, 0)
// - id: "south"  → point: (0, -5, 0) → direction: (0, -1, 0)
// - id: "door"   → point: (2, 0, 0)  → direction: (1, 0, 0)
//
// INTERFACE controls compatibility:
// - "door-frame" interface only connects to "door-frame"
// - "window-opening" only connects to "window"
```

**Visual representation**:

```
TYPE: "Room Module"
                    ┌─────────────────────────────────────┐
                    │                                     │
                    │            3D MODEL                 │
    Connector ────► ○                                     ○ ◄──── Connector
    "West"          │           (GLB file)                │       "East"
    Interface:      │                                     │       Interface:
    "door-frame"    │                                     │       "window-frame"
                    │                                     │
                    └─────────────────────────────────────┘
                                     ○
                                     │
                                Connector
                                "South"
                                Interface:
                                "corridor"
```

**Type hierarchy** (inheritance):

```
Base Type: "Wall" (virtual, defines connectors)
├── Subtype: "Wall-Standard" (common wall)
├── Subtype: "Wall-Corner" (90° corner)
├── Subtype: "Wall-Glass" (glazed wall)
└── Subtype: "Wall-Door" (includes door opening)
```

**Why types are central**

Types encode design knowledge:

- Geometry (what it looks like)
- Topology (how it connects)
- Semantics (what it means)
- Constraints (rules it follows)

This knowledge persists across projects and can be shared.

**What it enables**

- Rapid assembly from proven components
- Consistent design language
- Automatic connection validation
- Design rules encoded once
- Knowledge transfer between projects
- Parametric variations (scale, mirror)

**What it limits**

- Upfront investment to create types
- Less flexibility than freeform design
- Type updates can break designs
- Complex types are hard to create
- Balance between too few/too many types
- Versioning and evolution challenges

---

#### 12.7 Designs: Using Types

**Plain explanation**

A Design is an assembly—a building made from types. If types are LEGO bricks, a design is the spaceship you built from them. The design specifies which types are used, where each piece is placed, and how pieces connect to each other.

You can have many designs using the same types: a small house design, a large house design, an office design—all using the same wall, window, and door types.

**Technical explanation**

**Design structure**:

```typescript
// ============================================================
// EXAMPLE 1: DESIGN INTERFACE - ASSEMBLY OF TYPES
// ============================================================
// Purpose: Define an assembly (building) made from types
// Relates to: Graph theory, spatial modeling
// Design = nodes (pieces) + edges (connections)
//
// js/semio/semio.ts

interface Design {
  guid: Guid;                   // Unique identifier
  name: string;                 // e.g., "Apartment Building A"
  variant?: string;             // e.g., "South Facing"
  
  // COMPONENTS - The building blocks
  pieces: Piece[];              // Instances of types (nodes)
  connections: Connection[];    // How pieces attach (edges)
  
  // ORGANIZATION - Structure
  layers: Layer[];              // Visual/logical grouping (like CAD layers)
  groups: Group[];              // Semantic grouping (e.g., "ground floor")
  
  // ANALYTICS - Computed data
  stats: Stat[];                // Computed metrics (area, cost, etc.)
  
  // SETTINGS - Behavior
  canScale?: boolean;           // Can whole design be scaled?
  canMirror?: boolean;          // Can whole design be mirrored?
  
  // METADATA
  view?: Camera;                // Default 3D view for previews
  authors: Author[];            // Who designed this
  attributes: Attribute[];      // Custom data
}

// ANALOGY:
// Design is like a LEGO model you built:
// - pieces = the individual bricks you placed
// - connections = how bricks snap together
// - layers = instruction manual pages
// - stats = "uses 47 bricks, weighs 200g"
```

**Pieces and Connections**:

```typescript
// ============================================================
// EXAMPLE 2: PIECE AND CONNECTION - GRAPH NODES AND EDGES
// ============================================================
// Purpose: Define placed instances and their relationships
// Relates to: Graph theory, spatial computing
// Pieces are nodes, Connections are edges in a graph
//
// js/semio/semio.ts

interface Piece {
  id: string;                   // Unique within design (e.g., "room-1")
  name?: string;                // Human name (e.g., "Living Room Module")
  
  // WHAT IT IS - Reference to type or sub-design
  type?: TypeReference;         // Points to a type (common case)
  design?: DesignReference;     // Or points to a sub-design (nesting)
  
  // PLACEMENT - Where is it?
  plane?: Plane;                // Fixed position/orientation in 3D
  center?: Point;               // Diagram position in 2D
  
  // MODIFICATIONS - How is it changed?
  scale?: number;               // Size multiplier (1.0 = normal)
  mirrorPlane?: Plane;          // Mirror across this plane
  
  // STATE - UI state
  isHidden?: boolean;           // Hide in views
  isLocked?: boolean;           // Prevent editing
  color?: string;               // Override color
}

// CONNECTION: How two pieces attach
interface Connection {
  connected: Side;              // One piece + its connector
  connecting: Side;             // Other piece + its connector
  
  // ADJUSTMENTS - Fine-tuning the connection
  gap?: number;                 // Y offset (forward/back)
  shift?: number;               // X offset (left/right)
  rise?: number;                // Z offset (up/down)
  rotation?: number;            // Rotate around Y axis
  turn?: number;                // Rotate around Z axis
  tilt?: number;                // Rotate around X axis
}

// SIDE: References a specific connector on a piece
interface Side {
  piece: string;                // Piece ID
  connector: string;            // Connector ID on that piece's type
}

// HOW CONNECTIONS WORK:
// Piece A (type: RoomModule, connector: "east")
//    ↓ connects to ↓
// Piece B (type: RoomModule, connector: "west")
// = Two room modules attached at their east/west sides
```

**Design graph**:

```
Design: "Apartment Building"

PIECES (nodes)                    CONNECTIONS (edges)
┌──────────┐                      
│ Lobby    │◄────────── connected ──────────► ┌──────────┐
│ (Piece 1)│     via "Main-Entry"/"Corridor"  │ Hallway  │
│ Type: L1 │                                  │ (Piece 2)│
└──────────┘                                  │ Type: H1 │
                                              └──────────┘
                                                   │
                                              connected via
                                              "Room-Entry"/"Door"
                                                   │
                     ┌─────────────────────────────┴──────────────┐
                     ▼                                            ▼
               ┌──────────┐                                 ┌──────────┐
               │ Unit A   │                                 │ Unit B   │
               │ (Piece 3)│                                 │ (Piece 4)│
               │ Type: U1 │                                 │ Type: U1 │
               └──────────┘                                 └──────────┘
```

**Why designs are separate from types**

Separation of concerns:

- Types define **what components are** (reusable)
- Designs define **how components are assembled** (project-specific)

The same types power many designs, and designs can evolve without changing types.

**What it enables**

- Reuse types across many designs
- Change types without recreating designs
- Hierarchical designs (designs containing designs)
- Automatic placement from connections
- Multiple views of same assembly
- Design exploration and iteration

**What it limits**

- Types must exist before placing pieces
- Connection constraints limit flexibility
- Complex graphs are hard to visualize
- Circular dependencies possible
- Performance with many pieces
- Validation complexity

---

#### 12.8 Collaboration: Multiple Users

**Plain explanation**

Collaboration means multiple people working on the same design at the same time—like Google Docs for architecture. User A adds a room while User B adjusts a connection, and both see each other's changes instantly. No emailing files back and forth, no "which version is current?"

But unlike text documents, designs have structure. Two people might try to move the same piece to different places. The system must handle these conflicts automatically.

**Technical explanation**

**Y.js CRDT-based collaboration**:

```typescript
// ============================================================
// EXAMPLE 1: Y.JS CRDT - AUTOMATIC CONFLICT RESOLUTION
// ============================================================
// Purpose: Allow simultaneous editing without conflicts
// Relates to: Distributed systems, real-time collaboration
// CRDT = Conflict-free Replicated Data Type
//
// js/semio/sketchpad/Sketchpad.tsx

// Y.js uses CRDTs - changes merge automatically
// No central coordinator needed

// SCENARIO 1: Different properties - both apply
// User A's action (at 10:00:00.001)
yPieces.get('piece-123').set('name', 'Living Room');  // Changes name
// User B's action (at 10:00:00.002, same piece)
yPieces.get('piece-123').set('color', '#ff0000');     // Changes color

// RESULT: Both changes apply - no conflict!
// { name: 'Living Room', color: '#ff0000' }
// Both users see this exact same result

// SCENARIO 2: Same property - deterministic winner
// User A's action
yPieces.get('piece-123').set('name', 'Kitchen');    // User A renames
// User B's action (simultaneously)
yPieces.get('piece-123').set('name', 'Bedroom');    // User B renames

// RESULT: Last-writer-wins (determined by vector clocks)
// BOTH users see the SAME final value (either Kitchen or Bedroom)
// The result is deterministic - same on every computer

// WHY THIS IS AMAZING:
// - No "file is locked by another user"
// - No merge conflicts dialog
// - No lost work
// - Works even offline (syncs when reconnected)
```

**Collaboration architecture**:

```
┌─────────────────────────────────────────────────────────────┐
│                    COLLABORATION FLOW                        │
│                                                              │
│  User A (Browser)          User B (Browser)                 │
│  ┌─────────────────┐       ┌─────────────────┐              │
│  │   Sketchpad     │       │   Sketchpad     │              │
│  │   Y.Doc (local) │       │   Y.Doc (local) │              │
│  └────────┬────────┘       └────────┬────────┘              │
│           │                         │                        │
│           └──────────┬──────────────┘                        │
│                      │                                       │
│                      ▼                                       │
│           ┌─────────────────┐                               │
│           │   Y.js Server   │                               │
│           │   (WebSocket)   │                               │
│           │                 │                               │
│           │ Broadcasts diffs │                               │
│           │ to all clients   │                               │
│           └────────┬────────┘                               │
│                    │                                         │
│                    ▼                                         │
│           ┌─────────────────┐                               │
│           │    Database     │                               │
│           │ (persistence)   │                               │
│           └─────────────────┘                               │
└─────────────────────────────────────────────────────────────┘
```

**Presence awareness**:

```typescript
// ============================================================
// EXAMPLE 2: PRESENCE AWARENESS - SEE OTHER USERS
// ============================================================
// Purpose: Show who else is editing and where
// Relates to: Real-time systems, user experience
// Like seeing other people's cursors in Google Docs
//
// js/semio/sketchpad/Sketchpad.tsx

import { Awareness } from 'y-protocols/awareness';           // Y.js awareness protocol

// Create awareness for this document
const awareness = new Awareness(yDoc);                       // Attach to Y.js doc

// TELL OTHERS ABOUT YOURSELF:
// Share your name, color, cursor position, and selection
awareness.setLocalStateField('user', {                       // Set local user info
  name: 'Alice',                                             // Your name
  color: '#ff6b6b',                                          // Your cursor color
  cursor: { x: 100, y: 200 },                                // Where your mouse is
  selection: ['piece-123', 'piece-456']                      // What you've selected
});

// LISTEN TO OTHERS:
// Get notified when other users' state changes
awareness.on('change', () => {                               // Subscribe to changes
  const users = Array.from(awareness.getStates().values());  // Get all users
  // users = [
  //   { name: 'Alice', color: '#ff6b6b', cursor: {...}, selection: [...] },
  //   { name: 'Bob', color: '#4dabf7', cursor: {...}, selection: [...] },
  // ]
  
  // Render colored cursors and highlights for each user
  renderCollaboratorCursors(users);                          // Show their cursors
  renderCollaboratorSelections(users);                       // Show their selections
});

// RESULT:
// - You see Bob's blue cursor moving across the canvas
// - You see which pieces Bob has selected (blue outline)
// - Bob sees your red cursor and selections
```

**Why collaboration is hard**

- **Conflicts**: Two users editing same thing
- **Latency**: Network delays cause temporary divergence
- **Ordering**: Actions must have consistent order
- **Presence**: Knowing who's doing what
- **Permissions**: Who can edit what

CRDTs solve many of these at the data structure level.

**What it enables**

- Real-time multi-user editing
- See other users' cursors and selections
- Automatic conflict resolution
- Work offline, sync when connected
- No version conflicts
- Faster design iteration

**What it limits**

- Server infrastructure required
- Complex debugging (distributed state)
- Last-writer-wins may surprise users
- Performance with many users
- Semantic conflicts not prevented
- Undo/redo across users is complex

---

#### 12.9 Validation: Ensuring Correctness

**Plain explanation**

Validation is the system checking your work—like a spell checker for design. It catches errors: "This piece references a type that doesn't exist," "These two pieces have the same ID," "This connector isn't compatible with that connector." Better to find problems while designing than when construction starts.

Validation runs continuously, highlighting problems as you work.

**Technical explanation**

**Validation system in `semio.ts`**:

```typescript
// ============================================================
// EXAMPLE 1: VALIDATION TYPES - DEFINING PROBLEMS
// ============================================================
// Purpose: Describe what can go wrong and how to fix it
// Relates to: Domain validation, error handling
// Pure domain logic - works in browser, server, everywhere
//
// js/semio/semio.ts

// PROBLEM: Describes what's wrong
interface Problem {
  constraintId: string;           // Which rule was broken
  severity: 'error' | 'warning';  // How serious is it
  message: string;                // Human-readable description
  location: {                     // WHERE is the problem
    entityKind: 'Type' | 'Design' | 'Piece' | 'Connection' | ...;
    entityGuid?: Guid;            // Which entity
    field?: string;               // Which field on entity
  };
  fixes: Fix[];                   // Suggested corrections
}

// FIX: Describes how to solve the problem
interface Fix {
  title: string;                  // "Rename to 'Wall 2'"
  diff: KitDiff;                  // The actual change to make
}

// VALIDATION FUNCTION: Check a whole kit
function validateSemioKit(kit: Kit): ValidationResult {
  const problems: Problem[] = [];                // Collect all problems
  const ctx = buildValidationContext(kit);       // Build lookup tables
  
  for (const constraint of constraints) {        // Run each constraint
    problems.push(...constraint(ctx));           // Add any problems found
  }
  
  return { problems };                           // Return all problems
}
```

**Built-in constraints**:

| Constraint | Description |
|------------|-------------|
| `guid-unique` | All GUIDs unique across kit |
| `type-name-unique` | Type names unique among siblings |
| `design-name-unique` | Design names unique among siblings |
| `piece-name-unique` | Piece names unique within design |
| `connector-name-unique` | Connector names unique within type |
| `connector-compatibility` | Connected connectors have compatible interfaces |

**Diff-based fixes**:

```typescript
// ============================================================
// EXAMPLE 2: FIX WITH DIFF - AUTOMATED CORRECTIONS
// ============================================================
// Purpose: Provide one-click fixes for problems
// Relates to: User experience, automation
// Fixes are diffs - they describe exact changes to make
//
// js/semio/semio.ts

// EXAMPLE PROBLEM with fix:
const problem = {
  constraintId: 'type-name-unique',               // Rule broken
  severity: 'error',                              // Serious problem
  message: 'Duplicate type name "Wall" among siblings',  // Explanation
  location: {                                     // Where
    entityKind: 'Type',                           // It's a type
    entityGuid: 'abc-123'                         // This specific type
  },
  fixes: [                                        // How to fix it
    {
      title: 'Rename to "Wall 2"',                // Button text
      diff: {                                     // The change
        types: {
          updated: [{ guid: 'abc-123', name: 'Wall 2' }]  // New name
        }
      }
    }
  ]
};

// APPLYING A FIX:
// User clicks "Rename to Wall 2" button
const fixedKit = applyKitDiff(kit, problem.fixes[0].diff);
// Now the kit has no duplicate name error!

// WHY DIFF-BASED FIXES:
// - Same fix works in UI, CLI, and API
// - Fixes are undoable (invert the diff)
// - Fixes are testable (apply and validate again)
// - No separate "fix" implementation per platform
```

**Cross-platform validation**:

```
┌─────────────────────────────────────────────────────────────┐
│                   VALIDATION EVERYWHERE                      │
│                                                              │
│   TypeScript          Python             C#                  │
│   (Sketchpad)         (Engine)           (Grasshopper)       │
│        │                  │                  │               │
│        │                  │                  │               │
│        └──────────────────┴──────────────────┘               │
│                           │                                  │
│                     Same Logic                               │
│                   Same Problems                              │
│                     Same Fixes                               │
└─────────────────────────────────────────────────────────────┘
```

**Why validation matters**

Without validation:

- Errors discovered late are expensive
- Bad data propagates to other systems
- Users lose trust in the tool
- Debugging requires deep knowledge

With validation:

- Immediate feedback catches errors early
- Suggested fixes reduce friction
- Consistent rules across all platforms
- Self-documenting constraints

**What it enables**

- Real-time error detection
- Automated fix suggestions
- Consistent rules everywhere
- Self-documenting constraints
- Integration with editors (VS Code)
- Confidence in data quality

**What it limits**

- Validation can be slow for large kits
- False positives frustrate users
- Custom rules require development
- Validation runs on every change
- Complex rules are hard to express
- Some errors only visible at runtime

---

#### 12.10 The Monorepo: Everything Together

**Plain explanation**

A monorepo keeps all code in one repository—like having your entire company in one building instead of scattered offices. When you need to coordinate a change across the website, the mobile app, and the backend, everyone's in the same place. No "I'll send you the file" or "Which version are you using?"

Semio's monorepo contains TypeScript, Python, C#, Go, and more—all synchronized.

**Technical explanation**

**Semio monorepo structure**:

```
# ============================================================
# EXAMPLE 1: MONOREPO LAYOUT - EVERYTHING IN ONE PLACE
# ============================================================
# Purpose: Keep all code synchronized in one repository
# Relates to: Project organization, build systems
# Like having all departments in one building
#
# Root of semio repository

semio/                          # Single Git repository
├── package.json                # Root: Nx workspace config
├── nx.json                     # Nx build orchestration rules
├── tsconfig.json               # TypeScript configuration
│
├── js/                         # JavaScript/TypeScript packages
│   ├── semio/                  # @semio/js - core domain models
│   ├── sketchpad/              # @semio/sketchpad - UI application
│   ├── vscode/                 # @semio/vscode - VS Code extension
│   └── docs/                   # @semio/docs - documentation site
│
├── py/                         # Python packages
│   └── engine/                 # @semio/engine - backend server
│
├── net/                        # C#/.NET packages
│   ├── Semio/                  # Core library (domain logic)
│   └── Semio.Grasshopper/      # Grasshopper plugin (Rhino)
│
├── go/                         # Go packages
│   ├── repo/                   # Repository CLI tool
│   └── mcp/                    # MCP server (AI integration)
│
├── jsonschema/                 # Generated from TypeScript → JSON
├── graphql/                    # API schemas (generated)
└── sql/                        # Database schemas (generated)

# WHY THIS LAYOUT:
# - All code in one `git clone`
# - Change TypeScript → all other languages update
# - One PR can fix bug across web + desktop + plugins
# - No "which version?" problems
```

**Nx build orchestration**:

```bash
# ============================================================
# EXAMPLE 2: NX COMMANDS - SMART BUILD SYSTEM
# ============================================================
# Purpose: Build only what's needed, in the right order
# Relates to: Build systems, dependency management
# Nx understands which packages depend on which
#
# Terminal commands

# SINGLE COMMANDS for everything:
npm run build          # Build ALL packages, respecting dependencies
                       # Builds @semio/js first, then things that depend on it
npm run test           # Run ALL tests across all packages
npm run dev            # Start ALL dev servers (web, docs, etc.)

# AFFECTED-ONLY COMMANDS (smart builds):
nx affected:build      # Only build what changed since last commit
                       # If you only changed @semio/docs, only that builds
nx affected:test       # Only test what changed
                       # If you changed @semio/js, tests js + everything using js

# VISUALIZE DEPENDENCIES:
nx graph               # Opens browser with interactive dependency graph
                       # See which packages depend on which

# EXAMPLE:
# You change js/semio/semio.ts (core domain)
# nx affected:build will build:
#   1. @semio/js (you changed it)
#   2. @semio/sketchpad (uses @semio/js)
#   3. @semio/vscode (uses @semio/js)
#   4. @semio/docs (uses @semio/js)
# It will NOT build:
#   - @semio/assets (doesn't use @semio/js)
```

**Schema synchronization**:

```
TypeScript Definitions (source of truth)
        │
        ▼
┌─────────────────────────────────────────────────────────────┐
│              Schema Generation Pipeline                      │
│                                                              │
│  js/semio/semio.ts  ────► jsonschema/kit.json               │
│                     ────► graphql/semio/schema.graphql      │
│                     ────► sql/sqlite/schema.sql             │
│                     ────► (Python/C# types generated)       │
└─────────────────────────────────────────────────────────────┘
```

**Cross-language changes**:

```bash
# ============================================================
# EXAMPLE 3: ATOMIC COMMITS - CHANGE EVERYTHING AT ONCE
# ============================================================
# Purpose: Keep all languages synchronized
# Relates to: Version control, coordination
# One commit updates all affected code across languages
#
# Terminal commands

# SCENARIO: Adding 'location' field to Type
# This affects TypeScript, Python, C#, and database schema

# STEP 1: Make changes in all affected files
git add js/semio/semio.ts    # Add field to TypeScript Type interface
git add jsonschema/          # JSON Schema regenerated
git add py/engine/           # Python Type class updated
git add net/Semio/           # C# Type class updated
git add sql/sqlite/          # SQL schema updated

# STEP 2: Single commit with all changes
git commit -m "Add 'location' field to Type"

# RESULT:
# - All languages have the new field
# - No version mismatch possible
# - If CI fails, the whole change is rejected
# - Easy to review (see all impacts in one place)

# WITHOUT MONOREPO:
# - Commit to TypeScript repo
# - Open PR to Python repo
# - Open PR to C# repo
# - Wait for all to merge
# - Hope versions match
# - Debug when they don't
```

**Why this organization works**

The alternative—separate repositories:

- Change propagation takes days
- Version mismatches cause bugs
- Coordination overhead
- No atomic changes

Monorepo benefits:

- Single version of truth
- Atomic cross-language changes
- Shared CI/CD
- Easier refactoring

**What it enables**

- Atomic changes across languages
- Consistent schemas everywhere
- Single CI/CD pipeline
- Easy cross-package refactoring
- Shared tooling and configuration
- Simplified dependency management

**What it limits**

- Repository size (cloning is slow)
- Build system complexity
- Language tool conflicts
- CI/CD pipeline complexity
- Requires monorepo expertise
- Merge conflicts span languages

---

#### 12.11 The Plugin System: Extensibility

**Plain explanation**

Plugins let you add new features without changing the core system—like apps on a phone. The phone's operating system provides the foundation; apps add functionality. Semio's plugin system allows new apps (Design, Type, Quality), new panels, new tools, and new integrations—all without modifying the Sketchpad core.

This follows the "open-closed principle": open for extension, closed for modification.

**Technical explanation**

**App plugin architecture**:

```typescript
// ============================================================
// EXAMPLE 1: APP PLUGIN INTERFACE - EXTENSION WITHOUT MODIFICATION
// ============================================================
// Purpose: Add new apps without changing core code
// Relates to: Open-closed principle, extensibility
// Each app registers itself - Sketchpad doesn't know about them
//
// js/semio/sketchpad/shared.ts

// Each app registers as a plugin
interface AppPlugin {
  id: string;                     // "design", "type", "quality"
  namespace: string;              // "DESIGN", "TYPE", "QUALITY"
  
  machine: {                      // XState machine contributions
    // XState contributions
    actions: Record<string, ActionFunction>;     // Functions to run
    guards: Record<string, GuardFunction>;       // Conditions to check
    eventHandlers: Record<string, EventHandler>;
    selectors: Record<string, SelectorFunction>;
  };
  
  createDefaultState: () => AppState;   // Initial state factory
  registerStores?: () => void;          // Optional store setup
}

// REGISTRATION: Each app registers itself on load
// js/semio/sketchpad/Design.tsx
const designAppPlugin: AppPlugin = {
  id: 'design',                          // Unique identifier
  namespace: 'DESIGN',                   // Event prefix
  machine: {
    eventHandlers: {                     // Handle events starting with DESIGN.
      'DESIGN.SELECT_PIECE': {           // When piece is selected
        action: (ctx, event) => ({       // Update state
          designApp: {
            ...ctx.designApp,            // Keep other state
            selection: { pieces: [...event.guids] }  // Set new selection
          }
        })
      }
    }
  },
  createDefaultState: () => ({ ... }),   // Factory for initial state
};

registerAppPlugin(designAppPlugin);      // Add to registry
// Now Sketchpad knows how to handle DESIGN.* events!
```

**Adding a new app** (no core changes):

```typescript
// ============================================================
// EXAMPLE 2: CREATING A NEW APP - ZERO CORE MODIFICATIONS
// ============================================================
// Purpose: Add new functionality without touching Sketchpad.tsx
// Relates to: Open-closed principle, modularity
// Just create a file and register - done!
//
// Steps to add a new app:

// STEP 1: Create app file: js/semio/sketchpad/MyApp.tsx

// STEP 2: Define plugin (what state and events does your app have?)
const myAppPlugin: AppPlugin = {
  id: 'myapp',                           // Unique identifier
  namespace: 'MYAPP',                    // Event prefix (MYAPP.*)
  machine: {
    eventHandlers: {
      'MYAPP.DO_SOMETHING': {            // Handle custom event
        action: (ctx, event) => ({ ... })  // Update state
      }
    }
  },
  createDefaultState: () => ({ ... }),   // Initial state
};

// STEP 3: Register (only on client side)
if (typeof window !== 'undefined') {     // Not on server
  registerAppPlugin(myAppPlugin);        // Add to registry
}

// STEP 4: Export React component
export default function MyApp() {
  return <div>My custom app</div>;       // Your UI
}

// THAT'S IT!
// No changes to Sketchpad.tsx required
// No changes to machine.ts required
// No changes to any other file required
// Your app just works!
```

**Panel section registration**:

```typescript
// ============================================================
// EXAMPLE 3: DYNAMIC PANELS - APPS ADD UI SECTIONS
// ============================================================
// Purpose: Apps can add content to shared panels
// Relates to: Composition, dependency injection
// Details panel shows different content per app
//
// Inside your app component:

useEffect(() => {
  // ADD A SECTION when app mounts
  addSection('details', {             // Add to "details" panel
    id: 'my-section',                 // Unique section ID
    label: t('mySection'),            // Translated title
    content: () => <MyComponent />,   // What to render
    order: 1,                         // Where in the list (1 = near top)
  });
  
  // REMOVE when app unmounts
  return () => removeSection('details', 'my-section');
}, []);

// RESULT:
// When user navigates to your app:
//   → Details panel shows your section
// When user navigates away:
//   → Your section disappears
// Other apps' sections appear based on which app is active
```

**Event dispatch without core knowledge**:

```
┌─────────────────────────────────────────────────────────────┐
│                  Sketchpad (Core)                            │
│                                                              │
│  sketchpadMachine:                                           │
│    on: {                                                     │
│      "*": { actions: "dispatchAppEvent" }  ← Wildcard       │
│    }                                                         │
│                                                              │
│  dispatchAppEvent:                                           │
│    Looks up registered handler for event.type               │
│    Executes handler from plugin registry                    │
│                                                              │
└─────────────────────────────────────────────────────────────┘
         ▲                    ▲                    ▲
         │                    │                    │
┌────────┴──────┐  ┌──────────┴────────┐  ┌───────┴────────┐
│ HomePlugin    │  │ DesignPlugin      │  │ MyPlugin       │
│ HOME.*        │  │ DESIGN.*          │  │ MYAPP.*        │
└───────────────┘  └───────────────────┘  └────────────────┘
```

**Why plugins are powerful**

Plugins enable:

- Teams adding features independently
- Third-party extensions
- Feature flags (enable/disable apps)
- Gradual migration
- A/B testing new UIs

Without modifying stable core code.

**What it enables**

- Add features without core changes
- Independent development and deployment
- Feature toggle/disable capability
- Third-party integrations
- Clean separation of concerns
- Testable in isolation

**What it limits**

- Plugin API must be designed carefully
- Breaking changes affect all plugins
- Cross-plugin communication complexity
- Plugin discovery/loading overhead
- Documentation needed for plugin authors
- Version compatibility challenges

---

#### 12.12 State Management: Keeping Everything in Sync

**Plain explanation**

State management is keeping track of "what's happening now" across the entire application. What's selected? What's being edited? What's the user's theme preference? With multiple views (3D scene, 2D diagram, property panel), all showing the same design, changes in one place must instantly update everywhere else.

It's like a spreadsheet: change one cell, and every formula referencing it updates automatically.

**Technical explanation**

**Semio's state architecture**:

```
┌─────────────────────────────────────────────────────────────┐
│                    STATE OWNERSHIP                           │
│                                                              │
│  ┌───────────────────────────────────────────────────────┐  │
│  │            XSTATE (UI State - Single Source)          │  │
│  │                                                       │  │
│  │  Sketchpad State:                                     │  │
│  │  - theme, language, expertise                         │  │
│  │  - navigation (which app active)                      │  │
│  │  - panelVisibility                                    │  │
│  │                                                       │  │
│  │  App States (per-app slices):                         │  │
│  │  - designApp: { selection, hover, activeTool, ... }   │  │
│  │  - typeApp: { selection, hover, camera, ... }         │  │
│  │  - kitApp: { filter, sort, ... }                      │  │
│  └───────────────────────────────────────────────────────┘  │
│                           │                                  │
│               actor.send() / useSelector()                  │
│                           │                                  │
│  ┌───────────────────────────────────────────────────────┐  │
│  │           Y.JS (Kit Data - Collaborative)             │  │
│  │                                                       │  │
│  │  KitStore (per kit, Y.Doc):                           │  │
│  │  - types: Y.Array<Type>                               │  │
│  │  - designs: Y.Array<Design>                           │  │
│  │  - qualities: Y.Array<Quality>                        │  │
│  │                                                       │  │
│  │  Real-time sync across users via CRDT                 │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

**XState for predictable UI state**:

```typescript
// ============================================================
// EXAMPLE 1: XSTATE MACHINE - VALID TRANSITIONS ONLY
// ============================================================
// Purpose: Define what state changes are allowed
// Relates to: Finite state machines, predictability
// Machine rejects invalid transitions automatically
//
// js/semio/sketchpad/Sketchpad.tsx

// STATE MACHINE defines valid states and transitions
const sketchpadMachine = createMachine({
  context: {                                                 // All state data
    theme: 'system',                                         // Current theme
    designApp: { selection: { pieces: [] }, hover: null },   // Design app state
    // ... more app states
  },
  on: {                                                      // Event handlers
    'SET_THEME': {                                           // Handle theme change
      actions: assign({ theme: (_, e) => e.theme })          // Update theme
    },
    'DESIGN.SELECT_PIECE': {                                 // Handle piece selection
      actions: assign({
        designApp: (ctx, e) => ({                            // Update design app
          ...ctx.designApp,                                  // Keep other state
          selection: { pieces: e.guids }                     // New selection
        })
      })
    },
  }
});

// READING STATE: Components use selectors (like database queries)
const selection = useSelector(actor, s => s.context.designApp.selection);
// ^ Only re-renders when selection actually changes

// WRITING STATE: Components send events (like commands)
const selectPiece = (guid) => actor.send({                   // Send event
  type: 'DESIGN.SELECT_PIECE',                               // Event type
  guids: [guid]                                              // Event data
});
// ^ Machine validates this is allowed, then updates state
```

**Triadic hook pattern** (`[value, setValue, canSet]`):

```typescript
// ============================================================
// EXAMPLE 2: TRIADIC HOOKS - CONSISTENT API EVERYWHERE
// ============================================================
// Purpose: All hooks return [value, setter, canSet]
// Relates to: API consistency, React patterns
// Components always know if they can modify state
//
// js/semio/sketchpad/Design.tsx

// ALL HOOKS FOLLOW THIS PATTERN:
function useDesignAppSelection(): HookResult<Selection> {
  const actor = useSketchpadActor();                         // Get state machine
  
  // 1. VALUE: Read current selection
  const selection = useSelector(actor,                       // Subscribe to state
    s => s.context.designApp?.selection ?? EMPTY_SELECTION   // Extract selection
  );
  
  // 2. CAN SET: Check if we're allowed to change it
  const canSet = useSelector(actor,                          // Subscribe to guards
    s => s.can({ type: 'DESIGN.SELECT_PIECE', guids: [] })   // Is event allowed?
  );
  
  // 3. SETTER: Function to update (only if allowed)
  const setSelection = canSet                                // If allowed
    ? (sel) => actor.send({ type: 'DESIGN.SELECT_PIECE', guids: sel.pieces })
    : undefined;                                             // Otherwise no setter
  
  return [selection, setSelection, canSet];                  // Return triple
}

// USAGE IN COMPONENTS:
const [selection, setSelection, canSetSelection] = useDesignAppSelection();

if (canSetSelection) {                                       // Check if allowed
  setSelection({ pieces: ['abc'] });                         // Safe to call
}

// WHY THIS PATTERN:
// - Components know when UI should be disabled
// - No runtime errors from calling undefined setter
// - Consistent across ALL hooks in the codebase
```

**Why this approach was chosen**

**Problem**: React's useState causes unpredictable updates in complex UIs with many views of the same data.

**Solution**:
- **XState**: Finite state machine ensures valid transitions only
- **Y.js**: CRDTs handle collaboration conflicts automatically
- **Separation**: UI state (XState) vs domain data (Y.js)

**What it enables**

- Predictable state transitions
- Time-travel debugging
- Automatic conflict resolution
- Consistent UI across views
- Easy testing (deterministic)
- Clear state ownership

**What it limits**

- Learning curve for XState/Y.js
- More boilerplate than useState
- Two state systems to understand
- Bridge complexity between XState and Y.js
- Performance tuning requires expertise
- Debugging distributed state

---

## Appendix

### A. Quick Reference: Key Concepts

This appendix provides a condensed reference for the core concepts covered in this manual, organized alphabetically for quick lookup.

| Concept | Definition | Category |
|---------|-----------|----------|
| **API** | Application Programming Interface - contracts for how software components communicate | Communication |
| **Backend** | Server-side code handling data storage, business logic, security | Architecture |
| **Branch** | Independent line of development in version control | Collaboration |
| **Cache** | Temporary storage for frequently accessed data to improve speed | Performance |
| **CI/CD** | Continuous Integration/Continuous Deployment - automated build and release | DevOps |
| **Client** | Software that requests services from a server | Networking |
| **Commit** | Snapshot of changes in version control | Collaboration |
| **Compiler** | Translates source code to machine code before execution | Execution |
| **CRDT** | Conflict-free Replicated Data Type - enables automatic merge | Collaboration |
| **CPU** | Central Processing Unit - executes program instructions | Hardware |
| **Database** | Organized storage for structured data | Storage |
| **Diff** | Representation of changes between two states | Data |
| **Docker** | Container platform for packaging applications | DevOps |
| **Event** | Occurrence that triggers a response in a program | Programming |
| **Framework** | Pre-built structure providing common functionality | Development |
| **Frontend** | User-facing part of an application | Architecture |
| **Function** | Reusable block of code performing a specific task | Programming |
| **Git** | Distributed version control system | Collaboration |
| **HTTP** | Hypertext Transfer Protocol - web communication standard | Networking |
| **Interpreter** | Executes source code line by line | Execution |
| **JSON** | JavaScript Object Notation - text data format | Data |
| **Library** | Collection of pre-written code for reuse | Development |
| **Memory** | Temporary storage while program runs | Hardware |
| **Microservice** | Small, independent service doing one thing | Architecture |
| **Monolith** | Single, unified application | Architecture |
| **Monorepo** | Single repository containing multiple projects | Development |
| **Process** | Running instance of a program | Execution |
| **Query** | Request for data from a database | Storage |
| **Queue** | Data structure for ordered message processing | Communication |
| **Repository** | Storage location for version-controlled code | Collaboration |
| **REST** | Architectural style for web APIs | Communication |
| **Schema** | Structure definition for data | Data |
| **Server** | Computer or software providing services to clients | Networking |
| **State** | Current values and conditions of a program | Programming |
| **Thread** | Lightweight execution unit within a process | Execution |
| **Type** | Classification of data determining valid operations | Programming |
| **Variable** | Named storage location for data | Programming |
| **WebSocket** | Protocol for real-time bidirectional communication | Networking |

---

### B. Glossary: Technical Terms Explained

**Abstraction**: Hiding complex implementation details behind a simpler interface.

**API Gateway**: Entry point that routes requests to appropriate backend services.

**Asynchronous**: Operations that don't block execution while waiting for results.

**Authentication**: Verifying identity (who you are).

**Authorization**: Verifying permissions (what you can do).

**Boilerplate**: Repetitive code required by a framework or language.

**Build System**: Tools that compile, bundle, and prepare code for deployment.

**CDN (Content Delivery Network)**: Distributed servers caching content near users.

**Container**: Lightweight, portable unit packaging application and dependencies.

**Dependency**: External code that your code relies upon.

**Deployment**: Process of making software available for use.

**DNS (Domain Name System)**: Translates domain names to IP addresses.

**Event Loop**: Mechanism handling asynchronous operations in single-threaded environments.

**GraphQL**: Query language for APIs offering flexible data fetching.

**gRPC**: High-performance RPC framework using Protocol Buffers.

**IDE (Integrated Development Environment)**: Software for writing and debugging code.

**Immutable**: Data that cannot be changed after creation.

**Index**: Data structure enabling fast lookups in databases or search systems.

**Infrastructure as Code**: Managing infrastructure through configuration files.

**Latency**: Delay between request and response.

**Load Balancer**: Distributes traffic across multiple servers.

**Logging**: Recording events and data for debugging and monitoring.

**Merge**: Combining changes from different branches.

**Middleware**: Software connecting components or handling cross-cutting concerns.

**ORM (Object-Relational Mapping)**: Bridging object-oriented code and databases.

**Package Manager**: Tool for installing and managing dependencies.

**Persistence**: Storing data beyond program execution.

**Refactoring**: Restructuring code without changing behavior.

**Regression**: Bug reintroduced after previously being fixed.

**Replica**: Copy of data or service for redundancy and scaling.

**Runtime**: Environment where program executes.

**Scalability**: Ability to handle increased load.

**SDK (Software Development Kit)**: Tools and libraries for building on a platform.

**Serialization**: Converting data structures to transferable format.

**Sharding**: Distributing data across multiple databases.

**Singleton**: Design pattern ensuring only one instance exists.

**Socket**: Endpoint for network communication.

**SSL/TLS**: Encryption protocols for secure communication.

**Staging**: Environment mimicking production for testing.

**State Machine**: Model with defined states and transitions.

**Synchronous**: Operations that block until complete.

**Telemetry**: Automated collection of metrics and logs.

**Throughput**: Amount of work processed per unit time.

**Timeout**: Maximum wait before operation is cancelled.

**Token**: String representing authentication or data.

**Transaction**: Atomic operation that succeeds or fails completely.

**Type Safety**: Compile-time checking of data types.

**Unit Test**: Test validating a small piece of code in isolation.

**Vendor Lock-in**: Dependency on a specific provider.

**Virtual Machine**: Emulated computer running within another.

**Webhook**: User-defined HTTP callback triggered by events.

**YAML**: Human-readable data format for configuration.

---

### C. The Technology Stack: Languages and Frameworks

**Languages Used in Semio**:

| Language | Purpose | Where Used |
|----------|---------|------------|
| **TypeScript** | UI, domain models, tooling | Sketchpad, VS Code extension |
| **JavaScript** | Runtime, build tools | Node.js scripts |
| **Python** | Backend, ML, automation | Engine, schema generation |
| **C#** | CAD plugins, .NET integration | Grasshopper, Rhino |
| **Go** | CLI tools, MCP server | repo CLI, mcp server |
| **SQL** | Database queries | SQLite storage |

**Key Frameworks and Libraries**:

| Technology | Purpose | Used For |
|------------|---------|----------|
| **React** | UI components | Sketchpad interface |
| **XState** | State machines | UI state management |
| **Y.js** | CRDTs | Real-time collaboration |
| **Three.js** | 3D graphics | 3D scene rendering |
| **Vite** | Build tool | Fast development server |
| **FastAPI** | Python web framework | Engine API |
| **Nx** | Monorepo management | Build orchestration |
| **Electron** | Desktop apps | Sketchpad desktop |
| **Playwright** | E2E testing | Integration tests |
| **Vitest** | Unit testing | JavaScript tests |
| **pytest** | Python testing | Engine tests |

---

### D. Further Learning: Resources and Next Steps

**For Programming Fundamentals**:

- *Eloquent JavaScript* by Marijn Haverbeke (free online)
- *The Missing Semester of Your CS Education* (MIT course)
- *CS50: Introduction to Computer Science* (Harvard, free)

**For Web Development**:

- MDN Web Docs (mozilla.org)
- *Fullstack Open* (University of Helsinki, free)
- React documentation (react.dev)

**For System Design**:

- *Designing Data-Intensive Applications* by Martin Kleppmann
- *System Design Interview* by Alex Xu
- High Scalability blog

**For Version Control**:

- *Pro Git* by Scott Chacon (free online)
- GitHub Learning Lab
- Atlassian Git tutorials

**For Architecture**:

- *Clean Architecture* by Robert C. Martin
- *Building Microservices* by Sam Newman
- *Patterns of Enterprise Application Architecture* by Martin Fowler

**Practice Platforms**:

- LeetCode, HackerRank (algorithms)
- Frontend Mentor (UI projects)
- Exercism (language practice)

---

### E. Visual Diagrams: How Systems Connect

**The Complete Stack**:

```
┌─────────────────────────────────────────────────────────────────┐
│                         USER                                     │
└───────────────────────────┬─────────────────────────────────────┘
                            │ HTTP/WebSocket
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                        CDN                                       │
│            (Static files cached near user)                       │
└───────────────────────────┬─────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                    LOAD BALANCER                                 │
│              (Distributes traffic)                               │
└───────────────────────────┬─────────────────────────────────────┘
                            │
           ┌────────────────┼────────────────┐
           ▼                ▼                ▼
    ┌──────────────┐ ┌──────────────┐ ┌──────────────┐
    │ Web Server 1 │ │ Web Server 2 │ │ Web Server 3 │
    │  (Frontend)  │ │  (Frontend)  │ │  (Frontend)  │
    └──────┬───────┘ └──────┬───────┘ └──────┬───────┘
           │                │                │
           └────────────────┼────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────────┐
│                     API GATEWAY                                  │
│            (Authentication, routing)                             │
└───────────────────────────┬─────────────────────────────────────┘
                            │
           ┌────────────────┼────────────────┐
           ▼                ▼                ▼
    ┌──────────────┐ ┌──────────────┐ ┌──────────────┐
    │   Service A  │ │   Service B  │ │   Service C  │
    │  (Backend)   │ │  (Backend)   │ │  (Backend)   │
    └──────┬───────┘ └──────┬───────┘ └──────┬───────┘
           │                │                │
           └────────────────┼────────────────┘
                            │
           ┌────────────────┼────────────────┐
           ▼                ▼                ▼
    ┌──────────────┐ ┌──────────────┐ ┌──────────────┐
    │   Database   │ │    Cache     │ │    Queue     │
    │ (PostgreSQL) │ │   (Redis)    │ │   (Kafka)    │
    └──────────────┘ └──────────────┘ └──────────────┘
```

**Development Workflow**:

```
Developer's Machine                      Shared Infrastructure
┌────────────────────┐                  ┌────────────────────┐
│                    │                  │                    │
│  Code Editor       │                  │  GitHub/GitLab     │
│  (VS Code)         │                  │  (Repository)      │
│        │           │                  │        │           │
│        ▼           │     git push     │        ▼           │
│  Local Git  ───────┼─────────────────►│  Remote Git        │
│        │           │                  │        │           │
│        │           │                  │        ▼           │
│  Local Tests       │                  │  CI/CD Pipeline    │
│        │           │                  │  (Tests, Build)    │
│        ▼           │                  │        │           │
│  Development       │                  │        ▼           │
│  Server            │                  │  Staging Env       │
│  (localhost:3000)  │                  │        │           │
│                    │                  │        ▼           │
└────────────────────┘                  │  Production        │
                                        │  (users access)    │
                                        │                    │
                                        └────────────────────┘
```

**Semio Architecture Summary**:

```
┌─────────────────────────────────────────────────────────────────┐
│                         SEMIO                                    │
│                                                                  │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │                 FRONTEND (Sketchpad)                      │  │
│  │                                                           │  │
│  │  React ─── XState ─── Y.js ─── Three.js                  │  │
│  │    │         │          │          │                      │  │
│  │    UI      State    Collab      3D View                   │  │
│  └───────────────────────────────────────────────────────────┘  │
│                            │                                     │
│                       HTTP/gRPC                                  │
│                            │                                     │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │                  BACKEND (Engine)                         │  │
│  │                                                           │  │
│  │  FastAPI ─── Pydantic ─── SQLite                         │  │
│  │    │           │            │                             │  │
│  │   API       Models       Storage                          │  │
│  └───────────────────────────────────────────────────────────┘  │
│                            │                                     │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │               INTEGRATIONS                                │  │
│  │                                                           │  │
│  │  Grasshopper ─── Rhino ─── VS Code                       │  │
│  │      (C#)        (C#)       (TS)                          │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                  │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │                    BUILD SYSTEM                           │  │
│  │                                                           │  │
│  │  Nx ─── TypeScript ─── Schema Gen                        │  │
│  │   │         │             │                               │  │
│  │  Build   Types      JSON/SQL/GraphQL                      │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

---

*End of The Programming & Systems Mind Atlas*
