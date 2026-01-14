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
// Function to generate a unique identifier
export const guid = () => uuidv7();

// Function to check deep equality
export const deepEqual = (a: any, b: any): boolean => {
  if (a === b) return true;
  if (a == null && b == null) return true;
  if (a == null || b == null) return false;
  if (typeof a !== typeof b) return false;
  // ... more comparison logic
  return false;
};

// Function to compute diff between two Points
export const getPointDiff = (before: Point, after: Point): PointDiff => {
  return {
    x: after.x - before.x,
    y: after.y - before.y,
    z: after.z - before.z,
  };
};

// Function to apply a diff to a Point
export const applyPointDiff = (base: Point, diff: PointDiff): Point => {
  const x = diff.x ?? 0;
  const y = diff.y ?? 0;
  const z = diff.z ?? 0;
  return {
    x: base.x + x,
    y: base.y + y,
    z: base.z + z,
  };
};

// Function to generate unique names
export const generateUniqueName = (
  baseName: string, 
  existingNames: string[], 
  separator: string = " "
): string => {
  if (!existingNames.includes(baseName)) return baseName;
  let counter = 2;
  while (existingNames.includes(`${baseName}${separator}${counter}`)) {
    counter++;
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
// Conditional statements - branching (from semio.ts)
export const getPointDiff = (before: Point, after: Point): PointDiff => {
  const diff: PointDiff = {};
  // Only include differences that exist
  if (before.x !== after.x) diff.x = after.x - before.x;
  if (before.y !== after.y) diff.y = after.y - before.y;
  if (before.z !== after.z) diff.z = after.z - before.z;
  return diff;
};

// Loop - iterate over all attributes
const getAttributesDiff = (before: Attribute[], after: Attribute[]): AttributesDiff => {
  const beforeGuids = new Set(before.map((a) => a.guid));
  const afterGuids = new Set(after.map((a) => a.guid));
  
  // Find removed attributes
  const removed = before
    .filter((a) => !afterGuids.has(a.guid))
    .map((a) => ({ guid: a.guid }));
  
  // Find added attributes
  const added = after.filter((a) => !beforeGuids.has(a.guid));
  
  // Find updated attributes
  const updated = after
    .filter((a) => beforeGuids.has(a.guid))
    .map((a) => ({
      attribute: { guid: a.guid },
      diff: getAttributeDiff(before.find((b) => b.guid === a.guid)!, a)
    }))
    .filter((u) => Object.keys(u.diff).length > 0);
    
  return { removed, added, updated };
};
```

**Control flow keywords**:
- `if/else if/else`: Conditional execution
- `while`: Loop while condition is true
- `for`: Iterate over a sequence
- `break`: Exit loop early
- `continue`: Skip to next iteration
- `return`: Exit function and return value

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
// Arrays for collections within entities
interface Kit {
  types?: Type[];        // Array of types
  designs?: Design[];    // Array of designs
  qualities?: Quality[]; // Array of qualities
  files?: File[];       // Array of files
}

// Python lists
class Kit(BaseModel):
    types: list[Type] = []
    designs: list[Design] = []
```

**Map/Dictionary**: Key-value pairs for fast lookup
```typescript
// TypeScript Map for GUID-based lookups
const typesByGuid: Map<Guid, Type> = new Map();
kit.types?.forEach(t => typesByGuid.set(t.guid, t));

// Python dict
types_by_guid: dict[str, Type] = {t.guid: t for t in kit.types}

// Y.js Map for reactive state
const yTypes = yDoc.getMap<YType>("types");
yTypes.set(type.guid, yType);
```

**Set**: Unique values, fast membership check
```typescript
// Set for tracking selected items
const selection = new Set<Guid>();
selection.add(piece.guid);
if (selection.has(piece.guid)) { /* is selected */ }

// Set for computing diffs
const beforeGuids = new Set(before.map(a => a.guid));
const afterGuids = new Set(after.map(a => a.guid));
const removed = before.filter(a => !afterGuids.has(a.guid));
```

**Tree**: Hierarchical structure
```typescript
// Type hierarchy (parent-child via reference)
interface Type {
  guid: Guid;
  name: string;
  parent?: TypeId;  // Reference to parent type
}

// To get all subtypes of a type:
const getSubtypes = (types: Type[], parentGuid: Guid): Type[] => {
  return types.filter(t => t.parent?.guid === parentGuid);
};

// Layer hierarchy uses path (implicit tree)
interface Layer {
  path: string;  // "Structure", "Structure/Walls", "Structure/Walls/External"
}
```

**Graph**: Pieces connected by connections
```typescript
// Design is an undirected graph
interface Design {
  pieces?: Piece[];        // Nodes
  connections?: Connection[]; // Edges
}

// Connection links two pieces
interface Connection {
  connected: Side;   // { piece: PieceId, connector: ConnectorId }
  connecting: Side;  // { piece: PieceId, connector: ConnectorId }
}

// Graph traversal to find component
const findConnectedPieces = (startGuid: Guid, connections: Connection[]): Set<Guid> => {
  const visited = new Set<Guid>();
  const queue = [startGuid];
  while (queue.length > 0) {
    const current = queue.shift()!;
    if (visited.has(current)) continue;
    visited.add(current);
    // Find all adjacent pieces
    connections.forEach(c => {
      if (c.connected.piece.guid === current) 
        queue.push(c.connecting.piece.guid);
      if (c.connecting.piece.guid === current)
        queue.push(c.connected.piece.guid);
    });
  }
  return visited;
};
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
// js/semio/semio.ts - Data as interfaces
interface Piece {
  guid: Guid;
  name?: string;
  type?: TypeId;
  plane?: Plane;
  center?: Point;
  isHidden?: boolean;
  isLocked?: boolean;
}

// Behavior as standalone functions
const getPieceDiff = (before: Piece, after: Piece): PieceDiff => { ... };
const applyPieceDiff = (base: Piece, diff: PieceDiff): Piece => { ... };
const areSamePieceId = (a: PieceId, b: PieceId): boolean => a.guid === b.guid;
```

**Python (class-based with Pydantic)**:
```python
# py/engine/engine.py - Classes with validation
class Piece(BaseModel):
    guid: str = Field(default_factory=lambda: str(uuid7()))
    name: str | None = None
    type: TypeId | None = None
    plane: Plane | None = None
    center: Point | None = None
    is_hidden: bool | None = None
    is_locked: bool | None = None
    
    def to_placed(self, kit: "Kit") -> "PlacedPiece":
        """Compute the placed representation of this piece."""
        ...
```

**C# (traditional OOP)**:
```csharp
// net/Semio/Semio.cs - Full OOP with properties
public class Piece : ISerializable
{
    public Guid Guid { get; set; }
    public string? Name { get; set; }
    public TypeId? Type { get; set; }
    public Plane? Plane { get; set; }
    public Point? Center { get; set; }
    public bool? IsHidden { get; set; }
    public bool? IsLocked { get; set; }
    
    public Plane ComputePlane(Kit kit, Design design)
    {
        // Method that uses instance state
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
// Inheritance - Type can extend parent Type
interface Type {
  parent?: TypeId;  // Inherit from parent type
  // Subtypes inherit connectors, props from parent
}

// Polymorphism - Different entity types share validation
const validateKit = (kit: Kit): ValidationResult => {
  const problems: Problem[] = [];
  kit.types?.forEach(t => problems.push(...validateType(t)));
  kit.designs?.forEach(d => problems.push(...validateDesign(d)));
  return { problems };
};

// Encapsulation - Store hides Y.js internals
class KitStore {
  private readonly yDoc: Y.Doc;
  private readonly yTypes: Y.Array<YType>;
  
  // Public API hides implementation
  addType(type: Type): void { ... }
  removeType(guid: Guid): void { ... }
  snapshot(): Kit { ... }
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
const computePiecePlane = (piece: Piece, connections: Connection[]): Plane => {
  let currentPlane = piece.plane;  // Local - safe
  // Modify currentPlane during computation
  return currentPlane;  // Returns when done
};
```

**Object/Store state**: Fields within stores
```typescript
// KitStore - persists with Y.js document
class KitStore {
  private readonly yTypes: Y.Array<YType>;  // Reactive state
  private snapshot: Kit | null = null;      // Cached state
}

// Design app state - persists in XState machine
interface DesignAppState {
  selection: { pieces: Guid[]; connections: Guid[] };
  hover: { pieces: Guid[]; connectors: Guid[] };
  camera: Camera;
  activeTool: ToolKind;
}
```

**Global/Application state**: XState machine context
```typescript
// Sketchpad.tsx - centralized state machine
const sketchpadMachine = createMachine({
  context: {
    theme: "system",
    language: "en",
    expertise: "normal",
    kits: [],              // All loaded kits
    homeApp: { ... },      // Home app state
    kitApp: { ... },       // Kit app state
    designApp: { ... },    // Design app state
    typeApp: { ... },      // Type app state
    // ...
  },
  // State transitions via events
  on: {
    "DESIGN.SELECT_PIECE": { actions: "selectPiece" },
    "SET_THEME": { actions: "setTheme" },
  }
});
```

**State management patterns in semio**:

```typescript
// XState for UI state (selection, hover, tools)
const actor = useSketchpadActor();
actor.send({ type: "DESIGN.SELECT_PIECE", guid: piece.guid });

// Y.js for collaborative data (kits, types, pieces)
yTypes.observe((event) => {
  // React to changes from any source (local or remote)
  invalidateSnapshot();
});

// Diffs for persistence and undo
const diff = getKitDiff(beforeKit, afterKit);
undoStack.push(diff);  // Can be inverted for redo
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
// Export types for other modules to use
export type Guid = string;
export interface Point { x: number; y: number; z: number; }
export interface Piece { guid: Guid; name?: string; plane?: Plane; }

// Export functions
export const guid = () => uuidv7();
export const getPointDiff = (before: Point, after: Point): PointDiff => { ... };

// Private helpers (not exported)
const normalizeVector = (v: Vector): Vector => { ... };

// Import from other modules
import { z } from "zod";
import { uuidv7 } from "uuidv7";
```

**Python (py/engine/engine.py)**:
```python
# Export via __all__ or by convention
from pydantic import BaseModel

class Point(BaseModel):
    x: float
    y: float
    z: float

class Kit(BaseModel):
    name: str
    types: list[Type] = []
    designs: list[Design] = []

# Private convention: underscore prefix
def _normalize_vector(v: Vector) -> Vector:
    ...
```

**C# (net/Semio/Semio.cs)**:
```csharp
// Namespace groups related code
namespace Semio
{
    // Public exports
    public class Kit { ... }
    public class Piece { ... }
    
    // Internal: visible only within assembly
    internal static class Helpers { ... }
    
    // Private: visible only within class
    private void ComputeInternal() { ... }
}
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
// js/semio/package.json
{
  "name": "@semio/js",
  "version": "0.1.0",
  "type": "module",
  "main": "index.ts",
  "dependencies": {
    "zod": "^3.24.4",
    "uuidv7": "^1.0.2",
    "three": "^0.175.0"
  },
  "devDependencies": {
    "typescript": "^5.8.3",
    "vitest": "^3.2.3"
  }
}
```

**Python (pyproject.toml)**:
```toml
# py/engine/pyproject.toml
[project]
name = "semio-engine"
version = "0.1.0"
dependencies = [
    "pydantic>=2.10.6",
    "fastapi>=0.115.0",
    "sqlmodel>=0.0.22",
]

[project.optional-dependencies]
dev = ["pytest>=8.0", "ruff>=0.8.6"]
```

**C# (NuGet)**:
```xml
<!-- net/Semio/Semio.csproj -->
<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <TargetFramework>net48</TargetFramework>
  </PropertyGroup>
  <ItemGroup>
    <PackageReference Include="Newtonsoft.Json" Version="13.0.3" />
    <PackageReference Include="Microsoft.Data.Sqlite" Version="8.0.0" />
  </ItemGroup>
</Project>
```

**Monorepo workspace (npm)**:
```json
// package.json (root)
{
  "name": "semio",
  "workspaces": [
    "assets",
    "js/*",
    "py/engine",
    "yak"
  ]
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
// js/semio/semio.ts

// Zod - Schema validation and parsing
import { z } from "zod";
const PointSchema = z.object({
  x: z.number(),
  y: z.number(),
  z: z.number(),
});

// Three.js - 3D graphics
import * as THREE from "three";
const scene = new THREE.Scene();
const geometry = new THREE.BoxGeometry(1, 1, 1);

// Y.js - CRDT for collaboration
import * as Y from "yjs";
const yDoc = new Y.Doc();
const yTypes = yDoc.getArray("types");

// XState - State machines
import { createMachine, assign } from "xstate";
const sketchpadMachine = createMachine({ ... });
```

**Python libraries**:
```python
# py/engine/engine.py

# Pydantic - Data validation
from pydantic import BaseModel, Field

# FastAPI - Web API
from fastapi import FastAPI

# SQLModel - SQL ORM
from sqlmodel import SQLModel, Session

# Graphene - GraphQL
import graphene
```

**C# libraries**:
```csharp
// net/Semio/Semio.cs

// Newtonsoft.Json - JSON serialization
using Newtonsoft.Json;

// QuikGraph - Graph algorithms
using QuikGraph;

// FluentValidation - Validation
using FluentValidation;
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
// js/semio/sketchpad/Design.tsx

// React calls your component when state changes
export function DesignCanvas({ designGuid }: { designGuid: Guid }) {
  // React hook - framework manages state
  const [selection, setSelection] = useDesignAppSelection();
  
  // React event handling - framework calls on user action
  const handleClick = (pieceGuid: Guid) => {
    setSelection({ pieces: [pieceGuid], connections: [] });
  };
  
  // React rendering - framework diffs and updates DOM
  return (
    <Canvas>
      {pieces.map(piece => (
        <PieceNode key={piece.guid} onClick={() => handleClick(piece.guid)} />
      ))}
    </Canvas>
  );
}
```

**XState (state machine framework)**:
```typescript
// js/semio/sketchpad/Sketchpad.tsx

// XState defines states, transitions, actions
const sketchpadMachine = createMachine({
  initial: "home",
  states: {
    home: {
      on: { "KIT.INIT": { target: "kit" } }
    },
    kit: {
      on: { 
        "DESIGN.INIT": { target: "design" },
        "TYPE.INIT": { target: "type" }
      }
    },
    design: { /* ... */ },
    type: { /* ... */ },
  }
});

// Framework interprets machine, calls your actions
```

**FastAPI (Python web framework)**:
```python
# py/engine/engine.py

from fastapi import FastAPI, HTTPException

app = FastAPI()

# FastAPI calls your function when route matches
@app.post("/kit/validate")
async def validate_kit(kit: Kit) -> ValidationResult:
    result = validate(kit)
    if not result.valid:
        raise HTTPException(status_code=400, detail=result.errors)
    return result
```

**Grasshopper (CAD framework)**:
```csharp
// net/Semio.Grasshopper/Semio.Grasshopper.cs

// Grasshopper calls SolveInstance when inputs change
public class TypeComponent : GH_Component
{
    protected override void SolveInstance(IGH_DataAccess DA)
    {
        // Framework provides DA, you read inputs and set outputs
        DA.GetData(0, ref name);
        DA.SetData(0, new Type { Name = name });
    }
}
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
// The Sketchpad runs in V8 (browser)
// V8 provides: memory management, event loop, Promise handling
// From js/semio/sketchpad/Sketchpad.tsx

// React components use V8's event loop for rendering
export function SketchpadProvider({ children }: { children: React.ReactNode }) {
  // V8 garbage collector manages all these objects automatically
  const [actor, send] = useMachine(sketchpadMachine);
  // ...
}
```

```python
# The Engine runs in CPython
# CPython provides: asyncio event loop, memory management, dynamic typing
# From py/engine/engine.py

from fastapi import FastAPI
import asyncio

app = FastAPI()  # FastAPI uses asyncio (Python's async runtime)

@app.post("/validate")
async def validate_kit(kit: Kit) -> ValidationResult:
    # CPython's asyncio handles concurrent requests
    # GIL means only one thread executes Python at a time
    # But async I/O can still handle many connections
    return await validate(kit)
```

```csharp
// Grasshopper runs in .NET CLR
// CLR provides: garbage collection, exception handling, reflection
// From net/Semio.Grasshopper/Semio.Grasshopper.cs

public class ConnectorComponent : GH_Component
{
    // CLR manages object lifetimes automatically
    // JIT compiles this C# to native code on first run
    protected override void SolveInstance(IGH_DataAccess DA)
    {
        // CLR's garbage collector handles Connector allocation
        var connector = new Connector { Id = "C1", Point = new Point(0, 0, 0) };
    }
}
```

```go
// CLI runs in Go's runtime
// Go provides: goroutines, garbage collection, fast startup
// From go/repo/main.go

func main() {
    // Go runtime starts very fast (milliseconds)
    // Perfect for CLI tools that run briefly
    cmd := &cobra.Command{Use: "repo"}
    cmd.Execute()
}
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
// The entire Sketchpad runs on V8's event loop
// From js/semio/sketchpad/Sketchpad.tsx

// User clicks a piece - click event queued
document.addEventListener('click', handleClick);

// XState state machine uses event loop for state transitions
const sketchpadMachine = createMachine({
  on: {
    'DESIGN.SELECT_PIECE': {
      // This handler runs when event is dequeued
      actions: assign({
        selection: (ctx, event) => [...ctx.selection, event.pieceId]
      })
    }
  }
});

// Animation frame requests queue render events
function renderLoop() {
  // Event loop dequeues this at 60 FPS
  renderer.render(scene, camera);
  requestAnimationFrame(renderLoop);  // Queue next frame
}

// Network response comes as event
async function loadKit(url: string) {
  const response = await fetch(url);  // Suspends, queues callback
  // Event loop resumes here when response arrives
  const kit = await response.json();
  return kit;
}
```

**Event loop visualization for Sketchpad**:
```
Event Queue: [click, mousemove, fetch-response, timer, render]
                ↓
            Event Loop
                ↓
           Current Event: "click"
                ↓
           handleClick() executes
                ↓
           Back to waiting for next event
```

**Python Engine (asyncio event loop)**:
```python
# FastAPI uses asyncio's event loop
# From py/engine/engine.py

import asyncio
from fastapi import FastAPI

app = FastAPI()

@app.post("/validate")
async def validate_kit(kit_json: dict) -> dict:
    # When request arrives, event loop calls this handler
    kit = Kit.from_dict(kit_json)
    
    # If we need to call external service:
    async with httpx.AsyncClient() as client:
        # await suspends - event loop handles other requests
        external_data = await client.get("https://api.example.com/data")
    
    # Event loop resumes when response arrives
    return validate(kit, external_data)

# uvicorn runs the asyncio event loop
# uvicorn engine:app --port 2507
# Loop handles hundreds of concurrent requests single-threaded
```

**XState event-driven architecture**:
```typescript
// Sketchpad uses XState - a state machine that reacts to events
// From js/semio/sketchpad/Sketchpad.tsx

// All user actions become events
actor.send({ type: 'DESIGN.SELECT_PIECE', pieceId: 'abc-123' });
actor.send({ type: 'DESIGN.SET_HOVER', target: { pieceId: 'xyz-789' } });
actor.send({ type: 'NAVIGATE', path: '/kit/types' });

// Machine definition declares event handlers
const sketchpadMachine = createMachine({
  initial: 'home',
  states: {
    home: {
      on: {
        'HOME.SELECT_KIT': { actions: 'selectKit' },
        'NAVIGATE': { target: 'kit', cond: 'isKitPath' }
      }
    },
    design: {
      on: {
        'DESIGN.SELECT_PIECE': { actions: 'selectPiece' },
        'DESIGN.DELETE_SELECTED': { actions: 'deleteSelected' }
      }
    }
  }
});

// Event loop processes events in order they arrive
// User click → event queued → machine transitions → UI re-renders
```

**Y.js real-time collaboration events**:
```typescript
// Y.js uses events for collaboration
// From js/semio/sketchpad/Sketchpad.tsx

// Local change creates event
yDoc.on('update', (update: Uint8Array) => {
  // Event loop handles this after transaction
  broadcastUpdate(update);  // Send to other users
});

// Remote change arrives as event
provider.on('update', (update: Uint8Array) => {
  // Event loop queues this when WebSocket message arrives
  Y.applyUpdate(yDoc, update);
  // Triggers re-render events
});
```

**Blocking the event loop - what NOT to do**:
```typescript
// BAD: This blocks the event loop
function processHugeKit(kit: Kit) {
  // Takes 5 seconds - UI frozen!
  for (const piece of kit.design.pieces) {
    complexCalculation(piece);  // Blocks event loop
  }
}

// GOOD: Chunked processing with yielding
async function processHugeKitAsync(kit: Kit) {
  const chunks = chunkArray(kit.design.pieces, 100);
  
  for (const chunk of chunks) {
    for (const piece of chunk) {
      complexCalculation(piece);
    }
    // Yield to event loop - UI stays responsive
    await new Promise(resolve => setTimeout(resolve, 0));
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
// Sketchpad calls Engine API over HTTP
// From js/semio/sketchpad/Sketchpad.tsx

async function validateKitWithEngine(kit: Kit): Promise<ValidationResult> {
  // TCP connection to localhost:2507
  const response = await fetch('http://localhost:2507/validate', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(kitToJson(kit))
  });
  
  // Response travels back over same TCP connection
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
# From py/engine/engine.py

from fastapi import FastAPI
import uvicorn

app = FastAPI(title="semio Engine", version="1.0.0")

# Server endpoints
@app.post("/validate")
async def validate_kit(kit_json: dict) -> dict:
    """Validate a kit and return problems"""
    kit = Kit.from_dict(kit_json)
    result = validate(kit)
    return result.to_dict()

@app.post("/place")
async def place_pieces(design_json: dict) -> dict:
    """Compute piece placements from connections"""
    design = Design.from_dict(design_json)
    placed = compute_placements(design)
    return placed.to_dict()

@app.get("/health")
async def health_check() -> dict:
    """Health endpoint for monitoring"""
    return {"status": "healthy", "version": "1.0.0"}

# Run server (event-driven via uvicorn)
if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=2507)
    # Handles hundreds of concurrent requests single-threaded
```

**Vite Dev Server (Node.js)**:
```javascript
// Vite serves Sketchpad during development
// Event-driven, handles many connections efficiently

// vite.config.ts
export default defineConfig({
  server: {
    port: 5173,
    hmr: true,  // Hot Module Replacement
    watch: {
      // Watch for file changes and push updates
      usePolling: false
    }
  }
})

// Vite server flow:
// 1. Browser connects to localhost:5173
// 2. Vite serves index.html
// 3. Browser requests modules
// 4. Vite transforms TypeScript → JavaScript on-the-fly
// 5. File change → WebSocket push → browser updates
```

**Collaboration Server (Liveblocks/Y.js)**:
```typescript
// WebSocket server for real-time collaboration
// Stateful: maintains room state and user presence

// Server maintains:
// - Y.js document state per room
// - User awareness (cursor position, selection)
// - Operation history for conflict resolution

// From js/semio/sketchpad/Sketchpad.tsx
const provider = new WebsocketProvider(
  'wss://collab.semio.dev',  // Server URL
  `kit-${kitGuid}`,           // Room ID
  yDoc                        // Y.js document
);

// Server handles:
// - New client connects → sends current state
// - Client sends update → broadcasts to others
// - Client disconnects → updates awareness
// - Reconnection → syncs missed updates
```

**Go MCP Server (for AI integration)**:
```go
// From go/mcp/main.go

// MCP server for AI assistants (Claude, Copilot)
// Runs as stdio server, not TCP

func main() {
    server := mcp.NewServer("semio-repo")
    
    // Register tools
    server.RegisterTool("ticket_open", ticketOpenHandler)
    server.RegisterTool("analyze", analyzeHandler)
    server.RegisterTool("fix", fixHandler)
    
    // Stdio-based communication
    // AI sends JSON requests via stdin
    // Server responds via stdout
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
// From js/semio/sketchpad/Sketchpad.tsx

// Rich client with substantial client-side logic
// React + XState + Y.js + Three.js

function SketchpadApp() {
  // All state management happens client-side
  const [actor] = useMachine(sketchpadMachine);
  
  // Local kit storage
  const kitStore = useKitStore(kitGuid);
  
  // 3D rendering in browser
  const { scene, camera, renderer } = useThreeScene();
  
  // Collaboration syncs to server
  const { yDoc, provider } = useYjsProvider(roomId);
  
  return (
    <Canvas>
      <Scene pieces={kitStore.pieces} />
      <Diagram connections={kitStore.connections} />
    </Canvas>
  );
}

// Client handles:
// - Full 3D rendering (Three.js)
// - State machine logic (XState)
// - Local persistence (IndexedDB)
// - Undo/redo history
// - Offline editing (Y.js local-first)
```

**VS Code Extension (Thick client)**:
```typescript
// From js/vscode/extension.ts

// VS Code extension runs entirely on client
// Server communication only for specific operations

export function activate(context: vscode.ExtensionContext) {
  // Local file watching
  const watcher = vscode.workspace.createFileSystemWatcher('**/*.kit.json');
  
  // Local validation (semio.ts runs in extension)
  watcher.onDidChange(async (uri) => {
    const content = await vscode.workspace.fs.readFile(uri);
    const kit = parseKit(content.toString());
    const result = validateKit(kit);  // Client-side validation
    updateDiagnostics(uri, result);
  });
  
  // Shell out to Go CLI for complex operations
  const analysis = await spawnRepoCommand(['analyze', file.fsPath]);
}
```

**Desktop App (Thick client - Electron)**:
```typescript
// From js/desktop/main.ts

// Electron app bundles browser + Node.js
// Full offline capability

const mainWindow = new BrowserWindow({
  webPreferences: {
    nodeIntegration: true  // Access file system
  }
});

// Load local kit files directly
ipcMain.handle('open-kit-file', async () => {
  const { filePaths } = await dialog.showOpenDialog({
    filters: [{ name: 'Kit', extensions: ['zip', 'kit.json'] }]
  });
  return fs.readFile(filePaths[0]);
});

// No server required for basic operations
```

**Grasshopper Plugin (Thick client - .NET)**:
```csharp
// From net/Semio.Grasshopper/Semio.Grasshopper.cs

// Grasshopper components run entirely in Rhino process
// No server communication needed

public class PieceComponent : GH_Component
{
    protected override void SolveInstance(IGH_DataAccess DA)
    {
        // All computation happens client-side in Rhino
        var type = DA.GetData<Type>("Type");
        var plane = DA.GetData<Plane>("Plane");
        
        var piece = new Piece { Type = type.Name, Plane = plane };
        DA.SetData("Piece", piece);
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
# Validate a kit
POST /validate HTTP/1.1
Host: localhost:2507
Content-Type: application/json

{
  "name": "Metabolism",
  "types": [...],
  "designs": [...]
}

---

HTTP/1.1 200 OK
Content-Type: application/json

{
  "problems": [
    {"constraintId": "guid-unique", "severity": "error", ...}
  ]
}
```

```http
# Get kit info
GET /kit/abc-123 HTTP/1.1
Host: kits.semio.dev
Accept: application/json

---

HTTP/1.1 200 OK
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
// From js/semio/sketchpad/Sketchpad.tsx

// POST request to validate kit
async function validateKitWithEngine(kit: Kit): Promise<ValidationResult> {
  const response = await fetch('http://localhost:2507/validate', {
    method: 'POST',  // Create/submit data
    headers: {
      'Content-Type': 'application/json',
      'Accept': 'application/json'
    },
    body: JSON.stringify(kitToJson(kit))
  });
  
  if (!response.ok) {
    // Handle HTTP errors
    if (response.status === 400) {
      throw new Error('Invalid kit format');
    } else if (response.status === 500) {
      throw new Error('Server error during validation');
    }
  }
  
  return response.json();
}

// GET request to fetch remote kit
async function fetchRemoteKit(url: string): Promise<Kit> {
  const response = await fetch(url, {
    method: 'GET',  // Retrieve data
    headers: { 'Accept': 'application/zip' }
  });
  
  if (response.status === 404) {
    throw new Error('Kit not found');
  }
  
  const blob = await response.blob();
  return parseKitZip(blob);
}
```

**Engine HTTP server (FastAPI)**:
```python
# From py/engine/engine.py

from fastapi import FastAPI, HTTPException
from fastapi.responses import JSONResponse

app = FastAPI()

@app.post("/validate")
async def validate_kit(kit_json: dict) -> JSONResponse:
    try:
        kit = Kit.from_dict(kit_json)
        result = validate(kit)
        # HTTP 200 with JSON body
        return JSONResponse(content=result.to_dict())
    except ValidationError as e:
        # HTTP 400 for client errors
        raise HTTPException(status_code=400, detail=str(e))
    except Exception as e:
        # HTTP 500 for server errors
        raise HTTPException(status_code=500, detail="Internal error")

@app.get("/kit/{kit_id}")
async def get_kit(kit_id: str) -> JSONResponse:
    kit = load_kit(kit_id)
    if kit is None:
        # HTTP 404 for not found
        raise HTTPException(status_code=404, detail="Kit not found")
    return JSONResponse(content=kit.to_dict())
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
// From js/semio/sketchpad/Sketchpad.tsx
// Sketchpad asking Engine to validate a kit

async function validateKit(kit: Kit): Promise<ValidationResult> {
  // REQUEST: Sketchpad asks Engine to validate
  const response = await fetch('http://localhost:2507/validate', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(kitToJson(kit))
  });
  
  // Execution PAUSES here until Engine responds
  // This is the "waiting for answer" part
  
  // RESPONSE: Engine provides the answer
  const result = await response.json();
  return ValidationResult.fromJson(result);
}
```

**Stateless by default**: Each request is independent

```python
# From py/engine/engine.py
# Each request to Engine is independent - no session memory

@app.post("/validate")
async def validate_kit(kit_json: dict) -> JSONResponse:
    # This request knows nothing about previous requests
    # Client includes everything needed (the full kit)
    kit = Kit.from_dict(kit_json)
    result = validate(kit)
    return JSONResponse(content=result.to_dict())
    # After response, Engine "forgets" this request
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
// Standard request-response: Validation
const validationResult = await fetch('/validate', { method: 'POST', body });

// Streaming response: Large kit download
const response = await fetch('/kit/metabolism.zip');
const reader = response.body.getReader();
while (true) {
  const { done, value } = await reader.read();
  if (done) break;
  progressCallback(value.length);  // Chunks arrive over time
}

// Batch request: Multiple validations
const results = await fetch('/validate-batch', {
  method: 'POST',
  body: JSON.stringify({ kits: [kit1, kit2, kit3] })
});
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
GET  /kit/metabolism      → Kit JSON
POST /validate           → ValidationResult JSON
POST /place              → PlacementResult JSON
```

**Library APIs (Function calls)**: Core domain functions

```typescript
// From js/semio/semio.ts - TypeScript API

import { Kit, validateKit, applyKitDiff } from '@semio/js';

const kit = loadKit('./metabolism.zip');      // Load API
const result = validateKit(kit);              // Validation API
const newKit = applyKitDiff(kit, diff);       // Diff API
```

```python
# From py/engine/engine.py - Python API

from semio import Kit, validate, place_pieces

kit = Kit.from_file('metabolism.zip')         # Load API
result = validate(kit)                        # Validation API
placements = place_pieces(design)             # Placement API
```

```csharp
// From net/Semio/Semio.cs - C# API

using Semio;

var kit = Kit.FromFile("metabolism.zip");     // Load API
var result = Validator.Validate(kit);         // Validation API
var planes = Placer.ComputePlanes(design);    // Placement API
```

**System APIs (OS services)**: File access

```typescript
// Node.js file system API used by CLI
import { readFile, writeFile } from 'fs/promises';

const kitData = await readFile('kit.json', 'utf8');
await writeFile('kit.json', JSON.stringify(kit));
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
/kits              - collection of all kits
/kits/metabolism   - specific kit named "Metabolism"
/kits/metabolism/types     - types in that kit
/kits/metabolism/designs   - designs in that kit
/kits/metabolism/types/capsule    - specific type
/kits/metabolism/designs/tower    - specific design
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
# From py/engine/engine.py
# Every request includes full kit - no session state

@app.post("/validate")
async def validate_kit(kit: KitInput) -> ValidationResult:
    # Request contains the entire kit to validate
    # Engine doesn't remember previous requests
    return validate(Kit.from_input(kit))
```

**Representations**: Resources have JSON representations

```json
GET /kits/metabolism

{
  "name": "Metabolism",
  "version": "1.0.0",
  "types": [
    {"guid": "abc-123", "name": "Capsule"},
    {"guid": "def-456", "name": "Frame"}
  ],
  "designs": [
    {"guid": "ghi-789", "name": "Nakagin Tower"}
  ]
}
```

**HATEOAS**: Responses include links to related resources

```json
GET /kits/metabolism

{
  "name": "Metabolism",
  "_links": {
    "self": "/kits/metabolism",
    "types": "/kits/metabolism/types",
    "designs": "/kits/metabolism/designs",
    "validate": "/kits/metabolism/validate",
    "download": "/kits/metabolism.zip"
  }
}
```

**RESTful Engine implementation**:

```python
# From py/engine/engine.py

from fastapi import FastAPI, HTTPException
from fastapi.responses import JSONResponse, FileResponse

app = FastAPI()

# GET - Read (safe, cacheable)
@app.get("/kits/{kit_id}")
async def get_kit(kit_id: str) -> JSONResponse:
    kit = load_kit(kit_id)
    if not kit:
        raise HTTPException(404, "Kit not found")
    return JSONResponse(content=kit.to_dict())

# POST - Create (not idempotent)
@app.post("/kits")
async def create_kit(kit: KitInput) -> JSONResponse:
    saved = save_kit(Kit.from_input(kit))
    return JSONResponse(content=saved.to_dict(), status_code=201)

# PUT - Replace (idempotent)
@app.put("/kits/{kit_id}")
async def replace_kit(kit_id: str, kit: KitInput) -> JSONResponse:
    if not kit_exists(kit_id):
        raise HTTPException(404, "Kit not found")
    saved = replace_kit(kit_id, Kit.from_input(kit))
    return JSONResponse(content=saved.to_dict())

# DELETE - Remove (idempotent)
@app.delete("/kits/{kit_id}")
async def delete_kit(kit_id: str) -> JSONResponse:
    if not kit_exists(kit_id):
        raise HTTPException(404, "Kit not found")
    remove_kit(kit_id)
    return JSONResponse(content={"deleted": kit_id})
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
{
  "name": "Metabolism",
  "version": "1.0.0",
  "types": [
    {
      "guid": "abc-123-def-456",
      "name": "Capsule",
      "connectors": [
        {
          "id": "bottom",
          "point": {"x": 0, "y": 0, "z": 0},
          "direction": {"x": 0, "y": 0, "z": -1}
        }
      ],
      "models": []
    }
  ],
  "designs": [],
  "isVirtual": false,
  "canScale": true,
  "canMirror": false
}
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
// TypeScript (js/semio/semio.ts)
import { z } from 'zod';

// Parse JSON string to Kit object
const kitJson = '{"name": "Metabolism", "types": [...]}';
const kitData = JSON.parse(kitJson);
const kit = KitSchema.parse(kitData);  // Zod validates

// Serialize Kit object to JSON string
const outputJson = JSON.stringify(kitToJson(kit), null, 2);
```

```python
# Python (py/engine/engine.py)
import json
from pydantic import BaseModel

# Parse JSON string to Kit object
kit_json = '{"name": "Metabolism", "types": [...]}'
kit_data = json.loads(kit_json)
kit = Kit.model_validate(kit_data)  # Pydantic validates

# Serialize Kit object to JSON string
output_json = json.dumps(kit.model_dump(), indent=2)
```

```csharp
// C# (net/Semio/Semio.cs)
using System.Text.Json;

// Parse JSON string to Kit object
var kitJson = "{\"name\": \"Metabolism\", \"types\": [...]}";
var kit = JsonSerializer.Deserialize<Kit>(kitJson);

// Serialize Kit object to JSON string
var outputJson = JsonSerializer.Serialize(kit, new JsonSerializerOptions { 
    WriteIndented = true 
});
```

```go
// Go (go/semio/semio.go)
import "encoding/json"

// Parse JSON string to Kit object
kitJson := `{"name": "Metabolism", "types": [...]}`
var kit Kit
err := json.Unmarshal([]byte(kitJson), &kit)

// Serialize Kit object to JSON string
outputJson, err := json.MarshalIndent(kit, "", "  ")
```

**JSON Schema for validation**:

```json
// From jsonschema/kit.json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "properties": {
    "name": { "type": "string", "minLength": 1 },
    "version": { "type": "string", "pattern": "^\\d+\\.\\d+\\.\\d+$" },
    "types": {
      "type": "array",
      "items": { "$ref": "#/definitions/Type" }
    }
  },
  "required": ["name", "types"]
}
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
# From graphql/repo/schema.graphql
# Query exactly what you need

query {
  repo {
    tickets(year: 2025) {
      slug
      status
      summary
      author {
        name
        email
      }
    }
  }
}
```

Response matches query shape exactly:

```json
{
  "data": {
    "repo": {
      "tickets": [
        {
          "slug": "VALIDATION-SYSTEM",
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
# From graphql/semio/schema.graphql

type Kit {
  guid: ID!
  name: String!
  version: String
  types: [Type!]!
  designs: [Design!]!
}

type Type {
  guid: ID!
  name: String!
  connectors: [Connector!]!
  models: [Model!]!
  isVirtual: Boolean!
  canScale: Boolean!
  canMirror: Boolean!
}

type Design {
  guid: ID!
  name: String!
  pieces: [Piece!]!
  connections: [Connection!]!
}

type Query {
  kit(guid: ID!): Kit
  validateKit(kit: KitInput!): ValidationResult!
  placeDesign(design: DesignInput!): PlacementResult!
}

type Mutation {
  createKit(input: KitInput!): Kit!
  updateKit(guid: ID!, input: KitInput!): Kit!
  deleteKit(guid: ID!): Boolean!
}
```

**Query examples for common semio operations**:

```graphql
# Get only type names and connector counts (no over-fetching)
query TypeOverview {
  kit(guid: "abc-123") {
    name
    types {
      name
      connectors {
        id
      }
    }
  }
}

# Get design with pieces and their placements
query DesignWithPlacements {
  kit(guid: "abc-123") {
    designs {
      name
      pieces {
        guid
        type { name }
        plane {
          origin { x y z }
          xAxis { x y z }
        }
      }
    }
  }
}

# Mutation: Add a new type
mutation AddType {
  addType(kitGuid: "abc-123", input: {
    name: "NewModule",
    connectors: [],
    isVirtual: false
  }) {
    guid
    name
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
1. Sketchpad HTTP request with upgrade header
2. Liveblocks server agrees to upgrade
3. Connection becomes WebSocket
4. Bidirectional Y.js sync until close
```

```typescript
// From js/semio/sketchpad/Sketchpad.tsx
// Y.js + Liveblocks WebSocket connection

import { createClient } from '@liveblocks/client';
import { LiveblocksProvider } from '@liveblocks/yjs';

const client = createClient({
  publicApiKey: 'pk_live_xxxxx',
});

// Create Y.js document for kit
const yDoc = new Y.Doc();
const yKit = yDoc.getMap('kit');

// Connect via WebSocket
const room = client.enter('metabolism-kit', {
  initialPresence: { cursor: null, selection: [] }
});

// Sync Y.js over WebSocket
const provider = new LiveblocksProvider(room, yDoc);

// Now changes flow automatically:
// Local edit → Y.js → WebSocket → Liveblocks → WebSocket → Other clients
```

**Real-time collaboration flow**:

```
┌──────────────────┐     WebSocket      ┌─────────────────┐
│   Sketchpad A    │ ←──────────────→   │   Liveblocks    │
│   (Browser 1)    │                    │    Server       │
│                  │                    │                 │
│  yDoc.getMap()   │     Y.js sync      │   Room State    │
│  yKit.set(...)   │ ←──────────────→   │                 │
└──────────────────┘                    └────────┬────────┘
                                                 │
                                                 │ WebSocket
                                                 ▼
                                        ┌─────────────────┐
                                        │   Sketchpad B   │
                                        │   (Browser 2)   │
                                        │                 │
                                        │  yDoc.getMap()  │
                                        │  Changes appear │
                                        └─────────────────┘
```

**Y.js change propagation**:

```typescript
// From js/semio/sketchpad/Sketchpad.tsx

// Subscribe to remote changes
yKit.observe((event) => {
  event.changes.keys.forEach((change, key) => {
    if (change.action === 'add' || change.action === 'update') {
      console.log(`Remote change: ${key} updated`);
      // UI automatically updates via React subscription
    }
  });
});

// Local change automatically syncs to others
function addPiece(piece: Piece) {
  const yPieces = yDesign.get('pieces') as Y.Array<Piece>;
  yPieces.push([piece]);
  // This triggers:
  // 1. Local Y.js observer
  // 2. WebSocket send to Liveblocks
  // 3. Liveblocks broadcasts to all other clients
  // 4. Their Y.js observers fire
  // 5. Their UIs update
}
```

**Presence for cursor awareness**:

```typescript
// Show other users' cursors and selections

const presence = room.getPresence();

// Broadcast local cursor position
function onMouseMove(e: MouseEvent) {
  room.updatePresence({
    cursor: { x: e.clientX, y: e.clientY }
  });
}

// Subscribe to others' presence
room.subscribe('others', (others) => {
  others.forEach((user) => {
    if (user.presence.cursor) {
      renderCursor(user.id, user.presence.cursor);
    }
    if (user.presence.selection) {
      highlightSelection(user.id, user.presence.selection);
    }
  });
});
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
// From js/semio/sketchpad/Sketchpad.tsx

// React component structure
export function Sketchpad({ providers }: SketchpadProps) {
  return (
    <SketchpadProvider>
      <Navbar />          {/* Navigation, breadcrumbs, panel toggles */}
      <Canvas>            {/* Main working area */}
        <Scene3D />       {/* Three.js 3D rendering */}
        <Diagram />       {/* 2D graph visualization */}
        <Table />         {/* Tabular data view */}
      </Canvas>
      <Panels>            {/* Side panels */}
        <Workbench />     {/* Type/design browser */}
        <Details />       {/* Selected item properties */}
        <Settings />      {/* User preferences */}
      </Panels>
      <Footer />          {/* Status, actions */}
    </SketchpadProvider>
  );
}
```

**CSS with Tailwind in Sketchpad**:

```tsx
// From js/semio/sketchpad/elements.tsx

// Tailwind utility classes for styling
export function Action({ icon, text, onClick }: ActionProps) {
  return (
    <button
      onClick={onClick}
      className="
        h-small w-small           /* 5-unit sizing */
        flex items-center gap-1   /* Layout */
        hover:bg-active           /* Semantic hover color */
        border border-element     /* Border with hover color */
        text-tiny                 /* 3-unit text size */
      "
    >
      {icon && <span className="h-tiny w-tiny">{icon}</span>}
      {text}
    </button>
  );
}
```

**Three.js for 3D rendering**:

```tsx
// From js/semio/sketchpad/Design.tsx

import { Canvas, useThree } from '@react-three/fiber';
import { OrbitControls, useGLTF } from '@react-three/drei';

function PieceModel({ piece, type }: PieceModelProps) {
  const model = useGLTF(type.models[0]?.url ?? '');
  const plane = computePlane(piece);
  
  return (
    <group
      position={[plane.origin.x, plane.origin.z, -plane.origin.y]}
      rotation={quaternionFromPlane(plane)}
    >
      <primitive object={model.scene.clone()} />
      {/* Connectors shown as spheres */}
      {type.connectors.map(c => (
        <Sphere key={c.id} position={[c.point.x, c.point.z, -c.point.y]} />
      ))}
    </group>
  );
}
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

from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
import numpy as np

app = FastAPI(title="semio Engine", version="1.0.0")

# Validation endpoint
@app.post("/validate")
async def validate_kit(kit: KitInput) -> ValidationResult:
    kit_obj = Kit.from_input(kit)
    
    # Run all validation constraints
    problems = []
    problems.extend(check_guid_uniqueness(kit_obj))
    problems.extend(check_name_uniqueness(kit_obj))
    problems.extend(check_connector_references(kit_obj))
    problems.extend(check_connection_validity(kit_obj))
    
    return ValidationResult(
        valid=len(problems) == 0,
        problems=problems
    )

# Placement algorithm endpoint
@app.post("/place")
async def place_pieces(design: DesignInput) -> PlacementResult:
    design_obj = Design.from_input(design)
    
    # Hierarchical placement algorithm
    placements = {}
    for piece in get_fixed_pieces(design_obj):
        placements[piece.guid] = piece.plane
    
    # BFS through connections
    for piece, parent, connection in traverse_connections(design_obj):
        parent_plane = placements[parent.guid]
        piece_plane = compute_connected_plane(
            parent_plane, connection, parent, piece
        )
        placements[piece.guid] = piece_plane
    
    return PlacementResult(placements=placements)
```

**Backend computation examples**:

```python
# 3D plane transformation (compute_connected_plane)
def compute_connected_plane(
    parent_plane: Plane,
    connection: Connection,
    parent_piece: Piece,
    child_piece: Piece
) -> Plane:
    # Get connector planes
    parent_connector = get_connector(parent_piece, connection.connected)
    child_connector = get_connector(child_piece, connection.connecting)
    
    # Apply connection parameters
    translation = np.array([
        connection.shift,  # X offset
        connection.gap,    # Y offset  
        connection.rise    # Z offset
    ])
    
    rotation = rotation_matrix(
        connection.rotation,  # Around Y
        connection.turn,      # Around Z
        connection.tilt       # Around X
    )
    
    # Compute final plane
    child_plane = transform_plane(
        parent_plane,
        parent_connector,
        child_connector,
        translation,
        rotation
    )
    
    return child_plane
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
// From js/semio/sketchpad/Sketchpad.tsx

import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { useActor } from '@xstate/react';

function SketchpadApp() {
  const [state, send] = useActor(sketchpadMachine);
  
  return (
    <BrowserRouter>
      <Routes>
        {/* No page reloads - React handles all navigation */}
        <Route path="/" element={<HomeApp />} />
        <Route path="/kit/:kitGuid" element={<KitApp />} />
        <Route path="/kit/:kitGuid/design/:designGuid" element={<DesignApp />} />
        <Route path="/kit/:kitGuid/type/:typeGuid" element={<TypeApp />} />
        <Route path="/kit/:kitGuid/quality/:qualityGuid" element={<QualityApp />} />
        <Route path="/docs/*" element={<DocsApp />} />
        <Route path="/feedback" element={<FeedbackApp />} />
      </Routes>
    </BrowserRouter>
  );
}
```

**Vite development server**:

```typescript
// vite.config.ts for Sketchpad

import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig({
  plugins: [react()],
  server: {
    port: 5173,          // Dev server port
    hmr: true,           // Hot Module Replacement
  },
  build: {
    rollupOptions: {
      output: {
        manualChunks: {
          // Code splitting for performance
          'three': ['three', '@react-three/fiber'],
          'xstate': ['xstate', '@xstate/react'],
          'yjs': ['yjs', '@liveblocks/client'],
        }
      }
    }
  }
});
```

**Progressive Web App capabilities (future)**:

```json
// manifest.json for installable PWA
{
  "name": "semio Sketchpad",
  "short_name": "Sketchpad",
  "start_url": "/",
  "display": "standalone",
  "background_color": "#ffffff",
  "theme_color": "#000000",
  "icons": [
    {
      "src": "/icons/icon-192.png",
      "sizes": "192x192",
      "type": "image/png"
    }
  ]
}
```

```typescript
// Service worker for offline support
// sw.ts

self.addEventListener('install', (event) => {
  event.waitUntil(
    caches.open('sketchpad-v1').then((cache) => {
      return cache.addAll([
        '/',
        '/index.html',
        '/main.js',
        '/styles.css',
        // Cache static assets
      ]);
    })
  );
});

self.addEventListener('fetch', (event) => {
  event.respondWith(
    caches.match(event.request).then((cached) => {
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
// From js/desktop/main.ts

import { app, BrowserWindow, ipcMain, Menu } from 'electron';
import path from 'path';

let mainWindow: BrowserWindow | null = null;

app.whenReady().then(() => {
  mainWindow = new BrowserWindow({
    width: 1400,
    height: 900,
    webPreferences: {
      nodeIntegration: false,
      contextIsolation: true,
      preload: path.join(__dirname, 'preload.js'),
    },
  });

  // Load Sketchpad (same React app as web)
  if (process.env.NODE_ENV === 'development') {
    mainWindow.loadURL('http://localhost:5173');
  } else {
    mainWindow.loadFile('dist/index.html');
  }

  // Native menu
  const menu = Menu.buildFromTemplate([
    {
      label: 'File',
      submenu: [
        { label: 'New Kit', accelerator: 'CmdOrCtrl+N', click: newKit },
        { label: 'Open Kit...', accelerator: 'CmdOrCtrl+O', click: openKit },
        { label: 'Save', accelerator: 'CmdOrCtrl+S', click: saveKit },
        { type: 'separator' },
        { label: 'Exit', role: 'quit' },
      ],
    },
  ]);
  Menu.setApplicationMenu(menu);
});

// IPC bridge to renderer (Sketchpad)
ipcMain.handle('file:save', async (event, kitData) => {
  const { dialog } = require('electron');
  const result = await dialog.showSaveDialog({
    filters: [{ name: 'Kit', extensions: ['zip'] }],
  });
  if (!result.canceled) {
    await fs.writeFile(result.filePath, kitData);
  }
  return result.filePath;
});
```

**Grasshopper plugin implementation**:

```csharp
// From net/Semio.Grasshopper/Semio.Grasshopper.cs

using Grasshopper.Kernel;
using Rhino.Geometry;
using Semio;

public class KitComponent : GH_Component
{
    public KitComponent() 
        : base("Kit", "Kit", 
               "Construct a semio Kit", 
               "semio", "Kit") { }

    protected override void RegisterInputParams(GH_InputParamManager pManager)
    {
        pManager.AddTextParameter("Name", "N", "Kit name", GH_ParamAccess.item);
        pManager.AddParameter(new TypeParam(), "Types", "T", 
                             "Types in kit", GH_ParamAccess.list);
        pManager.AddParameter(new DesignParam(), "Designs", "D", 
                             "Designs in kit", GH_ParamAccess.list);
    }

    protected override void RegisterOutputParams(GH_OutputParamManager pManager)
    {
        pManager.AddParameter(new KitParam(), "Kit", "K", 
                             "Constructed kit", GH_ParamAccess.item);
    }

    protected override void SolveInstance(IGH_DataAccess DA)
    {
        string name = "";
        var types = new List<TypeGoo>();
        var designs = new List<DesignGoo>();
        
        DA.GetData(0, ref name);
        DA.GetDataList(1, types);
        DA.GetDataList(2, designs);

        var kit = new Kit
        {
            Name = name,
            Types = types.Select(t => t.Value).ToList(),
            Designs = designs.Select(d => d.Value).ToList(),
        };

        DA.SetData(0, new KitGoo(kit));
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
// Hypothetical React Native semio app
// js/mobile/KitViewer.tsx

import { View, FlatList, TouchableOpacity } from 'react-native';
import { useKit } from '@semio/js';  // Shared domain logic

export function KitViewer({ kitGuid }: { kitGuid: Guid }) {
  const kit = useKit(kitGuid);
  
  return (
    <View style={styles.container}>
      <FlatList
        data={kit.types}
        renderItem={({ item }) => (
          <TouchableOpacity 
            onPress={() => navigateToType(item.guid)}
            style={styles.typeCard}
          >
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
# py/engine/Dockerfile

# Start with Python base
FROM python:3.11-slim

# Set working directory
WORKDIR /app

# Install system dependencies (cached layer)
RUN apt-get update && apt-get install -y \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Install Python dependencies (cached layer)
COPY requirements.txt ./
RUN pip install --no-cache-dir -r requirements.txt

# Copy application code (changes frequently)
COPY engine.py ./
COPY models/ ./models/

# Expose Engine port
EXPOSE 2507

# Health check
HEALTHCHECK --interval=30s --timeout=10s \
  CMD curl -f http://localhost:2507/health || exit 1

# Run FastAPI server
CMD ["uvicorn", "engine:app", "--host", "0.0.0.0", "--port", "2507"]
```

**Docker Compose for development**:

```yaml
# docker-compose.yml

version: '3.8'

services:
  engine:
    build: ./py/engine
    ports:
      - "2507:2507"
    volumes:
      - ./py/engine:/app  # Hot reload in development
    environment:
      - DEBUG=true
    
  liveblocks-proxy:
    image: semio/liveblocks-proxy
    ports:
      - "4000:4000"
    environment:
      - LIVEBLOCKS_SECRET_KEY=${LIVEBLOCKS_SECRET_KEY}
```

**Running semio with Docker**:

```bash
# Build Engine container
docker build -t semio/engine ./py/engine

# Run Engine container
docker run -d \
  --name semio-engine \
  -p 2507:2507 \
  semio/engine

# Check logs
docker logs semio-engine

# Interactive shell for debugging
docker exec -it semio-engine bash

# Stop and remove
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
# k8s/engine-deployment.yaml

apiVersion: apps/v1
kind: Deployment
metadata:
  name: semio-engine
  labels:
    app: engine
spec:
  replicas: 3  # Start with 3 pods
  selector:
    matchLabels:
      app: engine
  template:
    metadata:
      labels:
        app: engine
    spec:
      containers:
        - name: engine
          image: semio/engine:1.0.0
          ports:
            - containerPort: 2507
          resources:
            requests:
              memory: "256Mi"
              cpu: "250m"
            limits:
              memory: "512Mi"
              cpu: "500m"
          livenessProbe:
            httpGet:
              path: /health
              port: 2507
            initialDelaySeconds: 10
          readinessProbe:
            httpGet:
              path: /health
              port: 2507
            initialDelaySeconds: 5
---
apiVersion: v1
kind: Service
metadata:
  name: engine
spec:
  selector:
    app: engine
  ports:
    - port: 2507
      targetPort: 2507
  type: ClusterIP
---
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: engine-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: semio-engine
  minReplicas: 2
  maxReplicas: 10
  metrics:
    - type: Resource
      resource:
        name: cpu
        target:
          type: Utilization
          averageUtilization: 70
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
// ❌ IF semio used microservices (unnecessary complexity):

// Type Service (port 3001)
app.post('/types/:typeGuid/validate', async (req, res) => {
  const type = await fetch('http://kit-service:3000/types/' + req.params.typeGuid);
  const connectors = await fetch('http://connector-service:3002/...');
  // Network latency: ~5ms × 3 services = ~15ms overhead
});

// ✅ What semio actually does (in-process):
function validateType(type: Type): ValidationResult {
  // Direct function call: ~0.001ms
  const connectorProblems = validateConnectors(type.connectors);
  return { problems: connectorProblems };
}
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
// BENEFIT 1: Direct function calls (fast, simple)

// In Sketchpad (TypeScript):
import { validateKit, applyKitDiff } from '@semio/js';

function handleKitChange(kit: Kit, diff: KitDiff): Kit {
  const result = validateKit(kit);         // Direct call: ~1ms
  if (result.problems.length === 0) {
    return applyKitDiff(kit, diff);         // No network needed
  }
  return kit;
}

// BENEFIT 2: Shared types (type safety across modules)
// Same Kit type used in js/semio/, js/sketchpad/, js/vscode/
```

```typescript
// BENEFIT 3: Atomic refactoring
// Rename "Connector" to "Port" everywhere at once:

// 1. Change in js/semio/semio.ts:
export interface Port {  // was Connector
  point: Point;
  direction: Vector;
}

// 2. TypeScript catches ALL usages across modules
// 3. Single PR, single deploy, no versioning headaches
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
# Example: Adding a new Connector property

git add js/semio/semio.ts             # TypeScript changes
git add py/engine/engine.py           # Python schema sync
git add net/Semio/Semio.cs            # C# model update
git add jsonschema/kit.json           # Schema regeneration

git commit -m "feat(connector): add mandatory flag for required connections"
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
# What changed in semio.ts recently?
git log --oneline js/semio/semio.ts

# Who added the Quality type?
git blame js/semio/semio.ts | grep "Quality"

# What was Kit like 6 months ago?
git show HEAD~100:js/semio/semio.ts | grep "interface Kit"

# All changes affecting connectors
git log --all --oneline --grep="connector"
```

**semio diff example**:

```diff
# git diff HEAD~1 js/semio/semio.ts

 export interface Connector {
   id: string;
   point: Point;
   direction: Vector;
+  mandatory?: boolean;     // NEW: Connector must be connected
   interface?: InterfaceId;
 }
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
# 1. Check current state
git status
#   modified: js/semio/semio.ts
#   modified: py/engine/engine.py
#   untracked: new-feature.ts

# 2. Stage specific changes
git add js/semio/semio.ts py/engine/engine.py

# 3. Commit with conventional message
git commit -m "feat(validation): add constraint for unique connector names"

# 4. Push to GitHub
git push origin main
```

**semio-specific Git commands**:

```bash
# Clone semio repository
git clone https://github.com/semio/semio.git
cd semio

# See what files were touched in last 10 commits
git log --oneline --name-only -10

# Find when Kit interface was last modified
git log -1 --format="%H %s" -- js/semio/semio.ts

# View specific commit details
git show a7f3b2c

# Compare current with last release
git diff v1.0.0..HEAD -- js/semio/semio.ts

# Find all commits by author
git log --author="usalu" --oneline

# Search commit messages for "connector"
git log --grep="connector" --oneline
```

**semio's .gitignore**:

```gitignore
# Dependencies (install fresh from package.json)
node_modules/
.venv/

# Build outputs (regenerate from source)
dist/
build/
*.pyc
__pycache__/
bin/
obj/

# Generated files (scripts regenerate these)
reports/*.json
jsonschema/*.json

# IDE settings
.idea/
.vscode/settings.json

# Environment secrets
.env
.env.local

# OS files
.DS_Store
Thumbs.db

# Semio temporary files
temp/
*.log
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
# For releases that need fixes after main progressed:
git checkout -b release/r25.01-1

# For experimental features (rare):
git checkout -b experiment/wasm-backend

# Merge back to main (squashed):
git checkout main
git merge --squash experiment/wasm-backend
git commit -m "feat: add WASM backend for browser performance"
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
# Small fix (screwdriver)
git commit -m "fix(validation): handle null connector list 🪛"

# Medium feature (hammer)  
git commit -m "feat(connector): add mandatory flag for required connections 🔨"

# Large feature (tools)
git commit -m "feat(quality): add benchmark system with benchmarks 🛠️"

# Major refactor (construction)
git commit -m "refactor(diff): consolidate diff system across all languages 🏗️"
```

**Anatomy of a semio commit**:

```
commit 3b4f5a6d8e2c1f0a9b8c7d6e5f4a3b2c1d0e9f8a
Author: usalu <ueli@semio.design>
Date:   Mon Jan 13 10:00:00 2025

    feat(connector): add Interface for connector compatibility 🔨
    
    Connectors now reference an Interface for explicit compatibility
    control instead of implicit name matching.
    
    - Add Interface entity to Kit with compatible_interfaces list
    - Add interface field to Connector
    - Update validation to check Interface compatibility
    - Sync schema across TypeScript, Python, C#, Go
    
    Closes #234
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
# Adding Interface system requires syncing across languages:

git add js/semio/semio.ts           # TypeScript Interface type
git add py/engine/engine.py         # Python Pydantic model
git add net/Semio/Semio.cs          # C# class
git add go/semio/semio.go           # Go struct
git add jsonschema/kit.json         # JSON schema update
git add sql/sqlite/schema.sql       # SQLite table

git commit -m "feat(interface): add Interface for connector compatibility 🛠️"
# Single atomic commit keeps all languages in sync
```

**Finding semio commits**:

```bash
# Find all commits touching validation
git log --grep="validation" --oneline

# Find when Interface was added
git log --all -S "interface Interface" -- js/semio/semio.ts

# Who last touched Connector?
git blame js/semio/semio.ts | grep "Connector"

# What changed in the Quality system?
git log --oneline -- js/semio/semio.ts | grep "quality"
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
┌─────────────────────────────────────────────────────────────────────────┐
│                    semio CONTRIBUTION WORKFLOW                           │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  Internal Development (ticket-based, no PR):                            │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  1. Open ticket: repo ticket open FEATURE-NAME                  │    │
│  │  2. Work on main branch directly                                │    │
│  │  3. Commit with conventional message                            │    │
│  │  4. Close ticket: repo ticket close FEATURE-NAME                │    │
│  │  5. Push squashed commit to main                                │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
│  External Contribution (PR-based):                                       │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  1. Fork semio repository                                       │    │
│  │  2. Create feature branch                                       │    │
│  │  3. Make changes, push to fork                                  │    │
│  │  4. Open Pull Request to semio/main                             │    │
│  │  5. CI runs preflight checks                                    │    │
│  │  6. Maintainer reviews                                          │    │
│  │  7. Squash merge to main                                        │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

**semio PR template**:

```markdown
## Description

<!-- What does this PR do? -->
Adds Interface entity for explicit connector compatibility control.

## Type of Change

- [ ] Bug fix (non-breaking change that fixes an issue)
- [x] New feature (non-breaking change that adds functionality)
- [ ] Breaking change (fix or feature that breaks existing APIs)
- [ ] Documentation update

## Changes

- Added `Interface` entity to Kit with `compatible_interfaces` list
- Added `interface` field to `Connector`
- Updated validation to check Interface compatibility
- Synced schema across TypeScript, Python, C#, Go

## Testing

- [ ] Added/updated unit tests
- [ ] Ran `npm run preflight` successfully
- [ ] Tested in Sketchpad manually

## Schema Changes

- [ ] Updated `js/semio/semio.ts`
- [ ] Updated `py/engine/engine.py`
- [ ] Updated `net/Semio/Semio.cs`
- [ ] Regenerated `jsonschema/kit.json`

## Related Issues

Closes #234
```

**semio PR checks (CI)**:

```yaml
# .github/workflows/ci.yml (simplified)

name: CI
on: [pull_request]

jobs:
  preflight:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
      - run: npm ci
      - run: npm run preflight  # Runs fix, analyze, test
      
  schema-sync:
    runs-on: ubuntu-latest
    steps:
      - run: npm run schema    # Verify schemas in sync
      - run: |
          if [ -n "$(git status --porcelain)" ]; then
            echo "Schema out of sync!"
            exit 1
          fi
```

**PR review focus for semio**:

| Focus Area              | What Reviewers Check                           |
|-------------------------|------------------------------------------------|
| Domain correctness      | Does Kit/Type/Design model make sense?         |
| Schema sync             | All 4 languages updated consistently?          |
| Diff system             | Can changes be diffed and undone?              |
| Validation              | New constraints for new fields?                |
| i18n                    | New UI text in both en.json and de.json?       |
| Test coverage           | New validation constraints have tests?         |

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
┌─────────────────────────────────────────────────────────────────────────┐
│                    semio CODE REVIEW LAYERS                              │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  Layer 1: Automated Checks (runs on every commit)                       │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  npm run preflight                                              │    │
│  │  ├── fix (Prettier, Ruff)          → Auto-format code           │    │
│  │  ├── analyze (i18n, code, ts, eslint) → Detect problems        │    │
│  │  └── test                          → Run unit tests             │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
│  Layer 2: Type System (compile-time checks)                             │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  TypeScript: Catches type mismatches across js/                 │    │
│  │  Pydantic: Validates Python models at runtime                   │    │
│  │  C# compiler: Catches .NET type errors                          │    │
│  │  Go compiler: Catches Go type errors                            │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
│  Layer 3: AI Review (development-time)                                  │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  Copilot/Claude reviews code as you write                       │    │
│  │  MCP tools check policy compliance                              │    │
│  │  AGENTS.md provides context for AI reviewers                    │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
│  Layer 4: Human Review (external PRs, major features)                   │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  External contributors get human review                         │    │
│  │  Breaking schema changes get maintainer review                  │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

**semio code policy checks (automated review)**:

```typescript
// hooks/code.ts checks for these violations:

// ❌ Inline comments (forbidden by policy)
const x = 5; // This is a comment - VIOLATION

// ❌ Block comments (forbidden)
/* This is a block comment */ // VIOLATION

// ❌ Empty regions
//#region Empty // VIOLATION - nothing inside
//#endregion

// ✅ License header (required)
// SPDX-License-Identifier: AGPL-3.0-or-later  // OK

// ✅ DEBUG logs (allowed, but flagged for cleanup)
console.log("[DEBUG] [TICKET-123] piece position:", piece.center);
```

**Review focus areas for semio**:

| Area                   | Automated Check                              | Human Check         |
|------------------------|----------------------------------------------|---------------------|
| Code formatting        | Prettier, Ruff (auto-fixed)                  | Never               |
| Type correctness       | TypeScript, Pydantic, C#, Go compilers       | Never               |
| Comment policy         | hooks/code.ts                                 | Never               |
| i18n completeness      | hooks/i18n.ts                                 | Never               |
| Schema sync            | npm run schema (CI failure if out of sync)   | Never               |
| Domain design          | Cannot automate                              | Yes (major changes) |
| API breaking changes   | Cannot automate                              | Yes (maintainer)    |
| UX decisions           | Cannot automate                              | Yes (design review) |

**AI-assisted review examples**:

```typescript
// Copilot/Claude might suggest during development:

// Original code:
const connector = type.connectors.find(c => c.id === id);
if (connector) {
  // do something
}

// AI suggestion:
// "Consider using optional chaining and early return for cleaner code"
const connector = type.connectors.find(c => c.id === id);
if (!connector) return;
// do something with connector (now definitely defined)
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
┌─────────────────────────────────────────────────────────────────────────┐
│                    semio CI PIPELINE                                     │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  LOCAL CI (pre-commit, runs before every commit):                       │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  .husky/pre-commit                                              │    │
│  │  ├── Prettier       → Format JS/TS/JSON/YAML/MD                 │    │
│  │  ├── Ruff           → Format + lint Python                      │    │
│  │  ├── i18n check     → Validate translations                     │    │
│  │  ├── TypeScript     → Type check (tsc --noEmit)                 │    │
│  │  ├── ESLint         → JS/TS linting                             │    │
│  │  └── Code policies  → Comments, headers, regions                │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
│  REMOTE CI (GitHub Actions, runs on push/PR):                           │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  .github/workflows/ci.yml                                       │    │
│  │  ├── Build          → npm run build                             │    │
│  │  ├── Test           → npm run test (Vitest, Playwright)         │    │
│  │  ├── Schema sync    → Verify generated files match              │    │
│  │  └── Cross-platform → Run on Windows, macOS, Linux              │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

**semio pre-commit hook** (`.husky/pre-commit`):

```bash
#!/bin/sh
. "$(dirname "$0")/_/husky.sh"

# Run formatters (auto-fix)
npx tsx hooks/prettier.ts
npx tsx hooks/ruff.ts

# Run linters (generate reports)
npx tsx hooks/i18n.ts
npx tsx hooks/code.ts
npx tsx hooks/typescript.ts
npx tsx hooks/eslint.ts

# Check if any linter reported errors
if [ -s reports/typescript.json ] || [ -s reports/eslint.json ]; then
  echo "❌ Linting errors found. Check reports/ folder."
  exit 1
fi
```

**semio GitHub Actions workflow**:

```yaml
# .github/workflows/ci.yml

name: CI
on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  preflight:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'
          cache: 'npm'
      
      - name: Install dependencies
        run: npm ci
      
      - name: Run preflight
        run: npm run preflight
      
      - name: Upload reports
        uses: actions/upload-artifact@v4
        with:
          name: reports
          path: reports/

  build:
    needs: preflight
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
      - run: npm ci
      - run: npm run build
      
  test:
    needs: build
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
      - run: npm ci
      - run: npm test
```

**semio CI commands**:

| Command               | What It Does                                 |
|-----------------------|----------------------------------------------|
| `npm run fix`         | Prettier + Ruff formatting                   |
| `npm run analyze`     | i18n + code + TypeScript + ESLint checks     |
| `npm run preflight`   | fix + analyze (runs both)                    |
| `npm run test`        | preflight + nx run-many -t test              |
| `npm run build`       | test + nx run-many -t build                  |

**Skip mechanism for development**:

```bash
# Skip checks during rapid iteration
npm run test -- --skip=preflight

# Skip specific checks
npm run test -- --skip=fix,analyze

# Pass args to Nx
npm run test -- --nx --projects=@semio/js
```

**Generated reports**:

```
reports/
├── i18n.json        # Translation validation
├── eslint.json      # ESLint problems
├── code.json        # Policy violations
├── typescript.json  # TypeScript errors
└── ruff.json        # Python linting
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
┌─────────────────────────────────────────────────────────────────────────┐
│                    semio DEPLOYMENT TARGETS                              │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  Documentation (js/docs) → GitHub Pages                                 │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  Push to main → Build Astro → Deploy to gh-pages branch         │    │
│  │  URL: https://semio.design                                      │    │
│  │  Strategy: Instant replacement (static site)                    │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
│  VS Code Extension (js/vscode) → Marketplace                            │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  npm run publish:vscode → Build VSIX → Upload to marketplace    │    │
│  │  Strategy: Manual trigger (breaking changes need changelog)     │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
│  Grasshopper Plugin (net/Semio.Grasshopper) → Yak (Rhino package)       │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  npm run publish:yak → Build DLL → Upload to yak.rhino3d.com    │    │
│  │  Strategy: Manual trigger (Rhino version compatibility)        │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
│  Desktop App (js/desktop) → GitHub Releases                             │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  npm run publish:desktop → electron-builder → Upload release    │    │
│  │  Strategy: Manual trigger (platform signing, notarization)     │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

**GitHub Pages deployment (docs)**:

```yaml
# .github/workflows/gh-pages.yml

name: Deploy Documentation
on:
  push:
    branches: [main]
    paths:
      - 'js/docs/**'
      - 'README.md'

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'
      
      - name: Install and build
        run: |
          npm ci
          npm run build -w @semio/docs
      
      - name: Deploy to GitHub Pages
        uses: peaceiris/actions-gh-pages@v4
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./js/docs/dist
```

**Yak (Grasshopper) deployment**:

```typescript
// yak/publish.ts

import { execSync } from 'child_process';

// Build the Grasshopper plugin
execSync('dotnet build net/Semio.Grasshopper -c Release');

// Create Yak package
execSync('yak build', { cwd: 'yak' });

// Publish to Yak repository
execSync('yak push *.yak', { cwd: 'yak' });
```

**semio deployment strategies by target**:

| Target           | Strategy            | Trigger       | Rollback                    |
|------------------|---------------------|---------------|------------------------------|
| Docs (gh-pages)  | Instant replacement | Auto on push  | Revert commit, redeploy      |
| VS Code          | Marketplace upload  | Manual        | Publish previous version     |
| Grasshopper (Yak)| Package upload      | Manual        | Yank version, upload previous|
| Desktop          | GitHub Release      | Manual        | Delete release, create new   |
| Engine (Docker)  | Rolling update      | Manual        | kubectl rollout undo         |

**Feature flags in Sketchpad**:

```typescript
// js/semio/sketchpad/Sketchpad.tsx

const featureFlags = {
  enableAIChat: import.meta.env.VITE_ENABLE_AI_CHAT === 'true',
  enableCollaboration: import.meta.env.VITE_ENABLE_COLLAB === 'true',
  enableExperimentalQuality: false,  // Hard-coded until stable
};

// Usage in component:
{featureFlags.enableAIChat && <AIChatPanel />}
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
┌─────────────────────────────────────────────────────────────────────────┐
│                    semio DATABASE ARCHITECTURE                           │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  Static Kit (zip file):                                                 │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  my-kit.zip                                                     │    │
│  │  ├── .semio/                                                    │    │
│  │  │   └── kit.db     ← SQLite database with all kit data        │    │
│  │  ├── models/                                                    │    │
│  │  │   ├── wall.glb   ← 3D model files                           │    │
│  │  │   └── beam.glb                                               │    │
│  │  └── images/                                                    │    │
│  │      └── preview.png                                            │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
│  Browser (dynamic kit):                                                  │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  IndexedDB (browser storage)                                    │    │
│  │  ├── Y.js documents for collaborative editing                   │    │
│  │  └── Kit snapshots for persistence                              │    │
│  └─────────────────────────────────────────────────────────────────┘    │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

**Why SQLite for semio**:

| Requirement              | SQLite Solution                              |
|--------------------------|----------------------------------------------|
| Portable kit files       | Single .db file inside .zip                  |
| No server needed         | Embedded, runs in-process                    |
| Offline support          | File-based, no network required              |
| Cross-platform           | Works on Windows, macOS, Linux, browser      |
| Relational queries       | Full SQL support for complex joins           |
| ACID transactions        | Kit saves are atomic                         |

**semio SQLite schema** (`sql/sqlite/schema.sql`):

```sql
-- Core kit metadata
CREATE TABLE kit (
    guid TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    version TEXT,
    description TEXT,
    remote_url TEXT,
    homepage_url TEXT,
    license TEXT,
    icon TEXT,
    image TEXT
);

-- Types in the kit
CREATE TABLE type (
    guid TEXT PRIMARY KEY,
    kit_guid TEXT REFERENCES kit(guid),
    parent_guid TEXT REFERENCES type(guid),
    name TEXT NOT NULL,
    variant TEXT,
    is_virtual BOOLEAN DEFAULT FALSE,
    can_scale BOOLEAN DEFAULT TRUE,
    can_mirror BOOLEAN DEFAULT TRUE,
    unit TEXT,
    available_count INTEGER,
    description TEXT,
    icon TEXT,
    image TEXT
);

-- Connectors on types
CREATE TABLE connector (
    guid TEXT PRIMARY KEY,
    type_guid TEXT REFERENCES type(guid),
    id TEXT NOT NULL,
    name TEXT,
    point_x REAL DEFAULT 0,
    point_y REAL DEFAULT 0,
    point_z REAL DEFAULT 0,
    direction_x REAL DEFAULT 0,
    direction_y REAL DEFAULT 1,
    direction_z REAL DEFAULT 0,
    t REAL DEFAULT 0,
    mandatory BOOLEAN DEFAULT FALSE,
    interface_guid TEXT REFERENCES interface(guid)
);

-- Designs in the kit
CREATE TABLE design (
    guid TEXT PRIMARY KEY,
    kit_guid TEXT REFERENCES kit(guid),
    parent_guid TEXT REFERENCES design(guid),
    name TEXT NOT NULL,
    variant TEXT,
    description TEXT,
    icon TEXT,
    image TEXT
);

-- Pieces in designs
CREATE TABLE piece (
    guid TEXT PRIMARY KEY,
    design_guid TEXT REFERENCES design(guid),
    type_guid TEXT REFERENCES type(guid),
    subdesign_guid TEXT REFERENCES design(guid),
    id TEXT NOT NULL,
    name TEXT,
    -- Plane (9 values: origin xyz, x-axis xyz, y-axis xyz)
    plane_origin_x REAL,
    plane_origin_y REAL,
    plane_origin_z REAL,
    plane_x_axis_x REAL,
    plane_x_axis_y REAL,
    plane_x_axis_z REAL,
    plane_y_axis_x REAL,
    plane_y_axis_y REAL,
    plane_y_axis_z REAL,
    scale REAL DEFAULT 1.0,
    is_hidden BOOLEAN DEFAULT FALSE,
    is_locked BOOLEAN DEFAULT FALSE,
    color TEXT
);

-- Connections between pieces
CREATE TABLE connection (
    guid TEXT PRIMARY KEY,
    design_guid TEXT REFERENCES design(guid),
    connected_piece_guid TEXT REFERENCES piece(guid),
    connected_connector_id TEXT,
    connecting_piece_guid TEXT REFERENCES piece(guid),
    connecting_connector_id TEXT,
    gap REAL DEFAULT 0,
    shift REAL DEFAULT 0,
    rise REAL DEFAULT 0,
    rotation REAL DEFAULT 0,
    turn REAL DEFAULT 0,
    tilt REAL DEFAULT 0
);
```

**Database types comparison**:

| Type          | semio Usage               | Example                   |
|---------------|---------------------------|---------------------------|
| SQLite        | Kit storage (primary)     | `.semio/kit.db`           |
| IndexedDB     | Browser persistence       | Y.js documents            |
| PostgreSQL    | (Not used yet)            | Future cloud backend      |
| Key-Value     | (Not used)                | Redis for caching         |

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
-- Find a type by guid
SELECT * FROM type WHERE guid = 'abc123-def456';

-- Get all connectors for a type
SELECT c.* 
FROM connector c
WHERE c.type_guid = 'abc123-def456';

-- Find all pieces using a specific type
SELECT p.id, p.name, d.name as design_name
FROM piece p
JOIN design d ON p.design_guid = d.guid
WHERE p.type_guid = 'abc123-def456';

-- Get connection graph for a design
SELECT 
    c.connected_piece_guid,
    c.connecting_piece_guid,
    c.gap, c.shift, c.rise,
    c.rotation, c.turn, c.tilt
FROM connection c
WHERE c.design_guid = 'xyz789-uvw012';
```

**Engine SQL queries** (`py/engine/engine.py`):

```python
import sqlite3
from pathlib import Path

def load_kit_from_db(kit_path: Path) -> Kit:
    """Load a Kit from SQLite database."""
    conn = sqlite3.connect(kit_path / '.semio' / 'kit.db')
    conn.row_factory = sqlite3.Row  # Access columns by name
    cursor = conn.cursor()
    
    # Load kit metadata
    cursor.execute("SELECT * FROM kit LIMIT 1")
    kit_row = cursor.fetchone()
    
    # Load all types
    cursor.execute("SELECT * FROM type WHERE kit_guid = ?", (kit_row['guid'],))
    types = []
    for type_row in cursor.fetchall():
        # Load connectors for this type
        cursor.execute(
            "SELECT * FROM connector WHERE type_guid = ?", 
            (type_row['guid'],)
        )
        connectors = [
            Connector(
                id=c['id'],
                point=Point(c['point_x'], c['point_y'], c['point_z']),
                direction=Vector(c['direction_x'], c['direction_y'], c['direction_z']),
                mandatory=c['mandatory']
            )
            for c in cursor.fetchall()
        ]
        types.append(Type(
            guid=type_row['guid'],
            name=type_row['name'],
            connectors=connectors
        ))
    
    conn.close()
    return Kit(guid=kit_row['guid'], name=kit_row['name'], types=types)
```

**SQL operations in semio**:

| Operation       | SQL Statement            | semio Use Case              |
|-----------------|--------------------------|------------------------------|
| Create table    | `CREATE TABLE`           | Schema initialization        |
| Insert row      | `INSERT INTO`            | Add new type/design/piece    |
| Update row      | `UPDATE SET WHERE`       | Modify connector position    |
| Delete row      | `DELETE FROM WHERE`      | Remove piece from design     |
| Query single    | `SELECT WHERE guid =`    | Load specific type           |
| Query join      | `SELECT JOIN ON`         | Get pieces with type info    |
| Aggregate       | `SELECT COUNT GROUP BY`  | Count pieces per type        |

**Complex semio queries**:

```sql
-- Find all "orphan" pieces (not connected to anything)
SELECT p.id, p.name
FROM piece p
LEFT JOIN connection c1 ON p.guid = c1.connected_piece_guid
LEFT JOIN connection c2 ON p.guid = c2.connecting_piece_guid
WHERE c1.guid IS NULL AND c2.guid IS NULL
  AND p.design_guid = 'xyz789';

-- Count types used in each design
SELECT d.name, COUNT(DISTINCT p.type_guid) as type_count
FROM design d
JOIN piece p ON p.design_guid = d.guid
GROUP BY d.guid
ORDER BY type_count DESC;

-- Find types with mandatory connectors that are never connected
SELECT t.name, c.id as connector_id
FROM type t
JOIN connector c ON c.type_guid = t.guid
WHERE c.mandatory = TRUE
  AND NOT EXISTS (
    SELECT 1 FROM connection conn
    WHERE (conn.connected_connector_id = c.id AND conn.connected_piece_guid IN 
           (SELECT guid FROM piece WHERE type_guid = t.guid))
       OR (conn.connecting_connector_id = c.id AND conn.connecting_piece_guid IN 
           (SELECT guid FROM piece WHERE type_guid = t.guid))
  );
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
// js/semio/sketchpad/Sketchpad.tsx

import { IndexeddbPersistence } from 'y-indexeddb';

function createKitStore(kitGuid: Guid): KitStore {
  const yDoc = new Y.Doc();
  
  // Persist Y.Doc to IndexedDB
  const persistence = new IndexeddbPersistence(
    `kit:${kitGuid}`,  // Key in IndexedDB
    yDoc                // Y.Doc to persist
  );
  
  persistence.on('synced', () => {
    console.log('[DEBUG] Kit loaded from IndexedDB');
  });
  
  return new KitStore(yDoc, persistence);
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
// CREATE: Add a new Type to a Kit
function createType(kit: Kit, typeData: Partial<Type>): Kit {
  const newType: Type = {
    guid: crypto.randomUUID(),
    name: typeData.name ?? "New Type",
    connectors: typeData.connectors ?? [],
    models: typeData.models ?? [],
    ...typeData
  };
  return { ...kit, types: [...kit.types, newType] };
}

// READ: Find a Type by GUID
function readType(kit: Kit, guid: Guid): Type | undefined {
  return kit.types.find(t => t.guid === guid);
}

// UPDATE: Modify a Type (using diff system)
function updateType(kit: Kit, guid: Guid, diff: TypeDiff): Kit {
  return {
    ...kit,
    types: kit.types.map(t => 
      t.guid === guid ? applyTypeDiff(t, diff) : t
    )
  };
}

// DELETE: Remove a Type
function deleteType(kit: Kit, guid: Guid): Kit {
  return {
    ...kit,
    types: kit.types.filter(t => t.guid !== guid)
  };
}
```

**semio CRUD in SQL** (Engine):

```sql
-- CREATE: Add a new connector
INSERT INTO connector (guid, type_guid, id, name, point_x, point_y, point_z)
VALUES ('new-guid', 'type-guid', 'top', 'Top Connector', 0, 1, 0);

-- READ: Get all connectors for a type
SELECT * FROM connector WHERE type_guid = 'type-guid';

-- UPDATE: Change connector position
UPDATE connector 
SET point_x = 0.5, point_y = 1.0, point_z = 0.0
WHERE guid = 'connector-guid';

-- DELETE: Remove a connector
DELETE FROM connector WHERE guid = 'connector-guid';
```

**semio CRUD in Y.js** (Sketchpad):

```typescript
// CREATE: Add piece to design (collaborative)
function createPiece(designStore: DesignStore, pieceData: Piece): void {
  const yPiece = yMapFromPiece(pieceData);
  designStore.yPieces.push([yPiece]);
}

// READ: Get piece from Y.js
function readPiece(designStore: DesignStore, guid: Guid): Piece | undefined {
  const yPiece = designStore.yPieces
    .toArray()
    .find(yp => yp.get("guid") === guid);
  return yPiece ? pieceFromYMap(yPiece) : undefined;
}

// UPDATE: Modify piece (triggers Y.js sync)
function updatePiece(designStore: DesignStore, guid: Guid, updates: Partial<Piece>): void {
  const index = designStore.yPieces.toArray().findIndex(
    yp => yp.get("guid") === guid
  );
  if (index !== -1) {
    const yPiece = designStore.yPieces.get(index);
    yPiece.doc?.transact(() => {
      Object.entries(updates).forEach(([key, value]) => {
        yPiece.set(key, value);
      });
    });
  }
}

// DELETE: Remove piece
function deletePiece(designStore: DesignStore, guid: Guid): void {
  const index = designStore.yPieces.toArray().findIndex(
    yp => yp.get("guid") === guid
  );
  if (index !== -1) {
    designStore.yPieces.delete(index);
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
// Zod schema with optional new fields (backward compatible)
export const ConnectorSchema = z.object({
  id: z.string(),
  name: z.string().optional(),                // Added in v2
  point: PointSchema,
  direction: VectorSchema,
  t: z.number().optional(),                   // Added in v3
  mandatory: z.boolean().optional(),          // Added in v4
  interface: GuidSchema.optional(),           // Added in v5
  props: z.array(PropSchema).optional(),      // Added in v6
  description: z.string().optional(),
  attributes: z.array(AttributeSchema).optional()
});

// Loading old kit (missing new fields)
const oldKitJson = {
  types: [{
    connectors: [{
      id: "top",
      point: { x: 0, y: 0, z: 1 },
      direction: { x: 0, y: 0, z: 1 }
      // No mandatory, interface, props fields
    }]
  }]
};

// Parsing applies defaults
const kit = KitSchema.parse(oldKitJson);
// kit.types[0].connectors[0].mandatory === undefined (optional)
```

**SQL schema migrations**:

```sql
-- Migration: Add mandatory column to connector
-- Version 4 → Version 5

-- Step 1: Add column with default
ALTER TABLE connector 
ADD COLUMN mandatory BOOLEAN DEFAULT FALSE;

-- Step 2: Add interface reference
ALTER TABLE connector 
ADD COLUMN interface_guid TEXT REFERENCES interface(guid);

-- Step 3: Update schema version
UPDATE meta SET value = '5' WHERE key = 'schema_version';
```

**C# migration handling** (`net/Semio/Semio.cs`):

```csharp
// JSON deserialization with missing properties
public class Connector
{
    public string Id { get; set; }
    public Point Point { get; set; }
    public Vector Direction { get; set; }
    
    // New properties with defaults (null if missing in old JSON)
    public bool? Mandatory { get; set; } = false;
    public Guid? Interface { get; set; }
    
    // Called after deserialization to apply defaults
    public void ApplyDefaults()
    {
        Mandatory ??= false;
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
export class Store<TState> {
  protected _cachedSnapshot: TState | null = null;
  protected _cachedHash: string | null = null;

  snapshot(): TState {
    const currentHash = this.hash(this.buildSnapshot());
    
    // Return cached if hash matches
    if (this._cachedSnapshot && this._cachedHash === currentHash) {
      return this._cachedSnapshot;
    }
    
    // Rebuild and cache
    this._cachedSnapshot = this.buildSnapshot();
    this._cachedHash = currentHash;
    return this._cachedSnapshot;
  }

  invalidateCache(): void {
    this._cachedSnapshot = null;
    this._cachedHash = null;
  }
}
```

**DerivedStore for computed values** (`js/semio/sketchpad/Sketchpad.tsx`):

```typescript
// Cache expensive piece metadata computations
const piecesMetadataNode = derivedStore.getOrCreate(
  "piecesMetadata",           // Cache key
  [{ store: designStore, path: [yPathMapKey("pieces")] }],  // Dependencies
  () => computePiecesMetadata(designStore.snapshot())       // Compute function
);

// Only recomputes when pieces change
const metadata = piecesMetadataNode.snapshot();
```

**Nx build caching** (`nx.json`):

```json
{
  "tasksRunnerOptions": {
    "default": {
      "runner": "nx/tasks-runners/default",
      "options": {
        "cacheableOperations": ["build", "lint", "test"],
        "parallel": 3
      }
    }
  },
  "namedInputs": {
    "default": ["{projectRoot}/**/*"],
    "production": [
      "default",
      "!{projectRoot}/**/*.test.ts"
    ]
  }
}
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
// Validation types
type Severity = "error" | "warning";
type SemioEntityKind = "Kit" | "Type" | "Design" | "Piece" | "Connector" | ...;

interface SemioDomainLocation {
  entityKind: SemioEntityKind;
  entityGuid?: Guid;
  field?: string;
}

interface Fix {
  title: string;
  diff: KitDiff;  // Diff-based fix
}

interface Problem {
  constraintId: string;
  severity: Severity;
  message: string;
  location: SemioDomainLocation;
  fixes: Fix[];
}

// Constraint function type
type Constraint = (ctx: ValidationContext) => Problem[];

// Example constraint: unique type names
const typeNameUniqueConstraint: Constraint = (ctx) => {
  const problems: Problem[] = [];
  const nameGroups = groupBy(ctx.kit.types, t => t.name);
  
  for (const [name, types] of Object.entries(nameGroups)) {
    if (types.length > 1) {
      // Generate fix: rename duplicates
      const fix = semioMakeFix(ctx.kit, {
        types: {
          updated: types.slice(1).map((t, i) => ({
            guid: t.guid,
            diff: { name: `${name} ${i + 2}` }
          }))
        }
      });
      
      problems.push({
        constraintId: "type-name-unique",
        severity: "error",
        message: `Duplicate type name "${name}"`,
        location: { entityKind: "Type", entityGuid: types[1].guid },
        fixes: [{ title: `Rename to "${name} 2"`, diff: fix }]
      });
    }
  }
  return problems;
};

// Run all validations
function validateSemioKit(kit: Kit): ValidationResult {
  const ctx = buildValidationContext(kit);
  const problems = defaultConstraints.flatMap(c => c(ctx));
  return { problems };
}
```

**Zod schema validation**:

```typescript
// Parse-time validation using Zod
const TypeSchema = z.object({
  guid: GuidSchema,
  name: z.string().min(1),        // Non-empty
  connectors: z.array(ConnectorSchema).default([]),
  models: z.array(ModelSchema).default([])
}).refine(
  t => t.connectors.every(c => c.id), 
  "Every connector must have an id"
);

// Throws ZodError if invalid
const type = TypeSchema.parse(jsonData);
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
import * as Y from 'yjs';
import { WebsocketProvider } from 'y-websocket';
import { IndexeddbPersistence } from 'y-indexeddb';

function createSyncedKitStore(kitGuid: Guid): KitStore {
  const yDoc = new Y.Doc();
  
  // Local persistence (works offline)
  const indexedDb = new IndexeddbPersistence(`kit:${kitGuid}`, yDoc);
  
  // Remote sync (when online)
  const wsProvider = new WebsocketProvider(
    'wss://sync.semio.design',
    `kit:${kitGuid}`,
    yDoc
  );
  
  // Connection status
  wsProvider.on('status', (event) => {
    console.log(`[DEBUG] Sync status: ${event.status}`);
  });
  
  return new KitStore(yDoc, { indexedDb, wsProvider });
}
```

**CRDT operations**:

```typescript
// Changes are automatically synced
yDoc.transact(() => {
  // Add new piece (visible to all connected users)
  const yPiece = new Y.Map();
  yPiece.set("guid", crypto.randomUUID());
  yPiece.set("name", "New Piece");
  yPiece.set("typeGuid", selectedTypeGuid);
  yPieces.push([yPiece]);
});

// Y.js merges changes from all users automatically
// No manual conflict resolution needed for most operations
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
// Changes work offline, sync when connected
function handlePieceMove(pieceGuid: Guid, newPosition: Point): void {
  const yPiece = findYPiece(pieceGuid);
  
  // This works offline (stored in IndexedDB)
  yDoc.transact(() => {
    yPiece.set("center", newPosition);
  });
  
  // When online, Y.js automatically syncs to server and other clients
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
import JSZip from 'jszip';

async function exportKitToZip(kit: Kit, files: Map<FileGuid, Blob>): Promise<Blob> {
  const zip = new JSZip();
  
  // Add kit database
  const db = await createKitDatabase(kit);
  zip.folder('.semio')?.file('kit.db', db);
  
  // Add model files
  const modelsFolder = zip.folder('models');
  for (const type of kit.types) {
    for (const model of type.models ?? []) {
      const fileBlob = files.get(model.file);
      if (fileBlob) {
        modelsFolder?.file(model.file, fileBlob);
      }
    }
  }
  
  // Generate zip
  return await zip.generateAsync({ type: 'blob' });
}

// Usage: download as file
const zipBlob = await exportKitToZip(kit, fileStore);
const url = URL.createObjectURL(zipBlob);
const a = document.createElement('a');
a.href = url;
a.download = `${kit.name}.zip`;
a.click();
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
// Check that all references point to existing entities
function validateReferences(kit: Kit): Problem[] {
  const problems: Problem[] = [];
  const typeGuids = new Set(kit.types.map(t => t.guid));
  const interfaceGuids = new Set(kit.interfaces?.map(i => i.guid) ?? []);
  
  for (const design of kit.designs) {
    for (const piece of design.pieces ?? []) {
      // Check piece → type reference
      if (piece.typeGuid && !typeGuids.has(piece.typeGuid)) {
        problems.push({
          constraintId: "piece-type-reference",
          severity: "error",
          message: `Piece "${piece.name}" references non-existent type`,
          location: { entityKind: "Piece", entityGuid: piece.guid },
          fixes: []
        });
      }
    }
  }
  
  for (const type of kit.types) {
    for (const connector of type.connectors ?? []) {
      // Check connector → interface reference
      if (connector.interface && !interfaceGuids.has(connector.interface)) {
        problems.push({
          constraintId: "connector-interface-reference",
          severity: "error",
          message: `Connector "${connector.id}" references non-existent interface`,
          location: { entityKind: "Connector", entityGuid: connector.id },
          fixes: []
        });
      }
    }
  }
  
  return problems;
}
```

**SQL foreign keys**:

```sql
-- SQLite foreign key constraints
PRAGMA foreign_keys = ON;

CREATE TABLE piece (
    guid TEXT PRIMARY KEY,
    design_guid TEXT NOT NULL REFERENCES design(guid) ON DELETE CASCADE,
    type_guid TEXT REFERENCES type(guid) ON DELETE SET NULL
);

-- Inserting piece with non-existent type fails
INSERT INTO piece (guid, design_guid, type_guid) 
VALUES ('piece-1', 'design-1', 'non-existent-type');
-- Error: FOREIGN KEY constraint failed
```

**Cascading deletes**:

```typescript
// When deleting a type, handle pieces using it
function deleteTypeWithCascade(kit: Kit, typeGuid: Guid): Kit {
  // Option 1: Delete pieces using this type
  const updatedDesigns = kit.designs.map(d => ({
    ...d,
    pieces: d.pieces?.filter(p => p.typeGuid !== typeGuid)
  }));
  
  // Option 2: Set piece.typeGuid to null (orphan)
  // Option 3: Prevent deletion if type is in use
  
  return {
    ...kit,
    types: kit.types.filter(t => t.guid !== typeGuid),
    designs: updatedDesigns
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
// PRESENTATION (Design.tsx) → DOMAIN (semio.ts)
import { applyKitDiff, getPieceDiff, computePiecePlane } from '../semio';

// Never imports from data layer directly
// ❌ import { yTypes } from './KitStore';  // Bad: UI accessing data
// ✅ const kit = useKit();                 // Good: through hooks

// DOMAIN (semio.ts) → has no imports from UI or data
// Pure functions only
export function computePiecePlane(
  piece: Piece,
  type: Type,
  connections: Connection[]
): Plane {
  // Pure calculation, no UI or DB access
}

// DATA (KitStore) → DOMAIN (semio.ts)
// Data layer uses domain types for snapshots
import { Kit, Type, Design } from '../semio';

export class KitStore extends Store<Kit> {
  snapshot(): Kit {
    return kitFromYDoc(this.yDoc);  // Convert Y.js to domain type
  }
}
```

**Cross-cutting concerns**:

```typescript
// Logging crosses all layers
console.log('[DEBUG] [Design.tsx] Piece dropped');
console.log('[DEBUG] [semio.ts] Computing placement');
console.log('[DEBUG] [KitStore] Y.js transaction');

// i18n crosses presentation layer
const { t } = useTranslation();
const label = t('semio.sketchpad.navbar.back');
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
// HIGH-LEVEL ABSTRACTION: User sees "place piece"
// js/semio/sketchpad/Design.tsx
function handlePieceDropped(typeGuid: Guid, position: Point) {
  executeCommand(
    "semio.designApp.addPiece",
    "semio.sketchpad.canvas.drop",
    { typeGuid, center: position }
  );
}

// MID-LEVEL ABSTRACTION: Command creates piece and updates state
// js/semio/sketchpad/Design.tsx
registerCommand("semio.designApp.addPiece", (ctx, args) => {
  const newPiece = createPiece(args.typeGuid, args.center);
  return {
    kitDiff: { designs: { updated: [{ guid: designGuid, 
      diff: { pieces: { added: [newPiece] } } }] } }
  };
});

// LOW-LEVEL: Diff application handles Y.js updates
// js/semio/sketchpad/Sketchpad.tsx
function applyKitDiffToYDoc(yDoc: Y.Doc, diff: KitDiff): void {
  yDoc.transact(() => {
    for (const added of diff.designs?.updated?.[0]?.diff?.pieces?.added ?? []) {
      const yPiece = yMapFromPiece(added);
      yPieces.push([yPiece]);
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
// Users of Connector don't need to know about Interface internals
interface Connector {
  id: string;
  point: Point;
  direction: Vector;
  interface?: InterfaceId;  // Just a GUID reference
}

// Interface compatibility hidden behind function
function areConnectorsCompatible(
  a: Connector, 
  b: Connector, 
  interfaces: Interface[]
): boolean {
  // Complex compatibility logic hidden
  if (!a.interface && !b.interface) return true;
  if (a.interface === b.interface) return true;
  const aInterface = interfaces.find(i => i.guid === a.interface);
  const bInterface = interfaces.find(i => i.guid === b.interface);
  // ... more complex logic
  return checkCompatibility(aInterface, bInterface);
}
```

**Data abstraction**: Hide representation behind operations

```typescript
// Point hides that it's x,y,z coordinates
// Users work with points without knowing internal structure
function translatePoint(p: Point, v: Vector): Point {
  return { x: p.x + v.x, y: p.y + v.y, z: p.z + v.z };
}

// Plane hides the complexity of coordinate systems
function planeToMatrix4(plane: Plane): THREE.Matrix4 {
  // Complex conversion hidden
  const origin = new THREE.Vector3(plane.origin.x, plane.origin.z, -plane.origin.y);
  // ... coordinate system transformation
  return matrix;
}
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
// TIGHT COUPLING: Direct import (within same package - OK)
// js/semio/sketchpad/Design.tsx
import { Kit, Type, Piece, applyKitDiff } from '../semio';

// LOOSE COUPLING: JSON interface (between packages - preferred)
// py/engine/engine.py
def load_kit(kit_json: str) -> Kit:
    data = json.loads(kit_json)  # JSON boundary
    return Kit.from_dict(data)

// DEPENDENCY INJECTION: Allow swapping implementations
// js/semio/sketchpad/Sketchpad.tsx
interface RemoteProviders {
  yProvider?: (guid: Guid) => YProvider;
  fileProvider?: FileProvider;
}

export function Sketchpad({ remoteProviders }: { remoteProviders?: RemoteProviders }) {
  // Uses injected providers, doesn't know concrete implementation
}
```

**Event-based decoupling** (XState):

```typescript
// Instead of direct function calls, use events
// TIGHT: designStore.setSelection(pieces)
// LOOSE: actor.send({ type: 'DESIGN.SET_SELECTION', pieces })

// Events decouple sender from receiver
actor.send({ type: 'DESIGN.SELECT_PIECE', pieceGuid });
// Handler can be changed without affecting sender
registerEventHandler('DESIGN.SELECT_PIECE', {
  action: (context, event) => ({
    designApp: {
      ...context.designApp,
      selection: { pieces: [event.pieceGuid] }
    }
  })
});
```

**Interface decoupling**:

```typescript
// FileProvider interface enables swappable implementations
interface FileProvider {
  upload: (kitId: string, fileId: string, path: string, blob: Blob) => Promise<string>;
  download: (kitId: string, fileId: string, path: string) => Promise<Blob>;
  delete: (kitId: string, fileId: string, path: string) => Promise<void>;
  getUrl: (kitId: string, fileId: string, path: string) => string;
}

// Can use MemoryFileProvider (testing), LocalFileProvider (IndexedDB), 
// or RemoteFileProvider (cloud) without changing consumer code
class MemoryFileProvider implements FileProvider { ... }
class LocalFileProvider implements FileProvider { ... }
class RemoteFileProvider implements FileProvider { ... }
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
// Everything in semio.ts relates to kit-of-parts domain
// HIGH COHESION: All functions work with Kit/Type/Design/Piece

// Schema definitions
export interface Kit { types: Type[]; designs: Design[]; ... }
export interface Type { connectors: Connector[]; models: Model[]; ... }
export interface Design { pieces: Piece[]; connections: Connection[]; ... }

// Diff operations (all work on same domain types)
export function getKitDiff(before: Kit, after: Kit): KitDiff { ... }
export function applyKitDiff(kit: Kit, diff: KitDiff): Kit { ... }
export function inverseKitDiff(kit: Kit, diff: KitDiff): KitDiff { ... }
export function mergeKitDiff(a: KitDiff, b: KitDiff): KitDiff { ... }

// Validation (all validate domain types)
export function validateKit(kit: Kit): ValidationResult { ... }
export function areConnectorsCompatible(...): boolean { ... }

// Placement (all compute piece positions)
export function computePiecePlane(...): Plane { ... }
export function computeConnectionPlane(...): Plane { ... }
```

**Low cohesion anti-pattern** (what semio avoids):

```typescript
// BAD: Random utilities file mixing unrelated concerns
// ❌ utils.ts (low cohesion)
export function formatDate(d: Date): string { ... }
export function calculateKitStats(kit: Kit): Stats { ... }
export function throttle(fn: Function): Function { ... }
export function validateEmail(email: string): boolean { ... }

// GOOD: Each concern in its own cohesive module
// ✅ semio.ts - kit domain only
// ✅ i18n.ts - localization only
// ✅ elements.tsx - UI primitives only
```

**App cohesion pattern**:

```typescript
// Each app file (Design.tsx, Type.tsx, Kit.tsx) is cohesive:
// - State for that app
// - Commands for that app
// - Components for that app
// - Hooks for that app

// Design.tsx has everything about design editing
interface DesignAppState { selection; hover; camera; activeTool; }
function useDesignAppSelection() { ... }
function useDesignAppHover() { ... }
registerCommand("semio.designApp.addPiece", ...);
registerEventHandler("DESIGN.SELECT_PIECE", ...);
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
// js/semio/sketchpad/Sketchpad.tsx
// FileProvider interface - contract for file storage
interface FileProvider {
  upload: (kitId: string, fileId: string, path: string, blob: Blob) => Promise<string>;
  download: (kitId: string, fileId: string, path: string) => Promise<Blob>;
  delete: (kitId: string, fileId: string, path: string) => Promise<void>;
  getUrl: (kitId: string, fileId: string, path: string) => string;
}

// Multiple implementations fulfill the same contract
class MemoryFileProvider implements FileProvider {
  private files = new Map<string, Blob>();
  async upload(kitId, fileId, path, blob) {
    this.files.set(`${kitId}/${fileId}/${path}`, blob);
    return URL.createObjectURL(blob);
  }
  // ... other methods
}

class LocalFileProvider implements FileProvider {
  async upload(kitId, fileId, path, blob) {
    const db = await openDB('semio-files');
    await db.put('files', blob, `${kitId}/${fileId}/${path}`);
    return `indexeddb://${kitId}/${fileId}/${path}`;
  }
  // ... other methods
}

class RemoteFileProvider implements FileProvider {
  async upload(kitId, fileId, path, blob) {
    const response = await fetch(`/api/kits/${kitId}/files/${fileId}`, {
      method: 'PUT',
      body: blob
    });
    return response.json().url;
  }
  // ... other methods
}
```

**Store interface pattern**:

```typescript
// All stores implement the same base interface
abstract class Store<TState> {
  abstract snapshot(): TState;
  abstract hash(state: TState): string;
  abstract buildSnapshot(): TState;
  onChanged: Observable<void>;
  onChangedDeep: Observable<void>;
}

// KitStore, DesignAppStore, TypeAppStore all extend Store
class KitStore extends Store<Kit> { ... }
class DesignAppStore extends Store<DesignAppState> { ... }
class TypeAppStore extends Store<TypeAppState> { ... }
```

**Command interface**:

```typescript
// All commands follow the same interface pattern
interface CommandHandler<TContext, TResult> {
  (context: TContext, ...args: any[]): TResult | Promise<TResult>;
}

// Registry maps command names to handlers
const commandRegistry = new Map<string, CommandHandler<any, any>>();

function registerCommand<TContext, TResult>(
  name: string,
  handler: CommandHandler<TContext, TResult>
): void {
  commandRegistry.set(name, handler);
}

// All commands are invoked the same way
function executeCommand(name: string, origin: string, ...args: any[]) {
  const handler = commandRegistry.get(name);
  return handler(buildContext(), ...args);
}
```

**TypeScript interface vs C# interface**:

```typescript
// TypeScript: structural typing (shape matters, not name)
interface Point { x: number; y: number; z: number; }
const p = { x: 0, y: 0, z: 0 };  // Automatically satisfies Point

// C#: nominal typing (must explicitly implement)
public interface IPoint { double X { get; } double Y { get; } double Z { get; } }
public class Point3D : IPoint { ... }  // Must declare : IPoint
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
// js/semio/sketchpad/Sketchpad.tsx

// BASE: Store - any component with state
abstract class Store<TState> {
  abstract snapshot(): TState;
  abstract hash(state: TState): string;
  abstract buildSnapshot(): TState;
  onChanged: Observable<void>;
  onChangedDeep: Observable<void>;
}

// LEVEL 2: AppStore - adds transactions and undo/redo
abstract class AppStore<TState, TDiff, TEdit> extends Store<TState> {
  protected undoStack: TEdit[] = [];
  protected redoStack: TEdit[] = [];
  protected currentTransactionStack: TEdit[] = [];
  
  startTransaction(): void { ... }
  finalizeTransaction(): void { ... }
  abortTransaction(): void { ... }
  undo(): void { ... }
  redo(): void { ... }
}

// LEVEL 3: KitDiffAppStore - adds kit modification tracking
abstract class KitDiffAppStore<TState, TDiff, TEdit> extends AppStore<TState, TDiff, TEdit> {
  abstract kit(): KitStore;
  
  applyEdit(edit: KitDiffAppEdit): void {
    super.applyEdit(edit);  // Call parent
    if (edit.kitDiff) {
      this.kit().change(edit.kitDiff);  // Additional kit logic
    }
  }
}

// CONCRETE: DesignAppStore
class DesignAppStore extends KitDiffAppStore<DesignAppState, DesignAppDiff, DesignAppEdit> {
  private readonly designGuid: Guid;
  private readonly kitStore: KitStore;
  
  kit(): KitStore { return this.kitStore; }
  snapshot(): DesignAppState { ... }
  // Design-specific methods
}
```

**Grasshopper component inheritance** (`Semio.Grasshopper.cs`):

```csharp
// Base class for all model components
public abstract class ModelComponent<TParam, TGoo, TModel> : GH_Component
    where TGoo : ModelGoo<TModel>
{
    protected abstract void RegisterModelInputParams(GH_InputParamManager pManager);
    protected abstract void RegisterModelOutputParams(GH_OutputParamManager pManager);
    protected abstract void GetModelData(IGH_DataAccess DA, TModel model);
    protected abstract void SetModelData(IGH_DataAccess DA, TModel model);
}

// Specialized for Id types
public abstract class IdComponent<TId> : ModelComponent<IdParam<TId>, IdGoo<TId>, TId>
    where TId : Id
{
    // Specialized behavior for Id models
}

// Concrete implementation
public class TypeIdComponent : IdComponent<TypeId>
{
    protected override void RegisterModelInputParams(GH_InputParamManager pManager) {
        pManager.AddTextParameter("Guid", "G", "Type GUID", GH_ParamAccess.item);
    }
    // ... specific implementation
}
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
// js/semio/semio.ts
// Kit is COMPOSED OF types and designs (not inherited)
interface Kit {
  types: Type[];      // Kit HAS types
  designs: Design[];  // Kit HAS designs
  qualities: Quality[];
  files: File[];
  authors: Author[];
}

// Type is COMPOSED OF connectors and models
interface Type {
  connectors: Connector[];  // Type HAS connectors
  models: Model[];          // Type HAS models
  props: Prop[];
}

// Design is COMPOSED OF pieces and connections
interface Design {
  pieces: Piece[];           // Design HAS pieces
  connections: Connection[]; // Design HAS connections
  layers: Layer[];
  groups: Group[];
}

// Piece REFERENCES a type (composition via reference)
interface Piece {
  type: TypeId;  // Piece HAS-A reference to Type
  plane?: Plane;
  center?: Point;
}
```

**UI composition** (React component composition):

```tsx
// js/semio/sketchpad/Sketchpad.tsx
// Sketchpad is COMPOSED OF Canvas, Navbar, Footer
function Sketchpad({ remoteProviders }: SketchpadProps) {
  return (
    <Providers>
      <Canvas>                   {/* HAS Canvas */}
        <WindowLayout />         {/* HAS Windows */}
      </Canvas>
      <Navbar items={navItems} /> {/* HAS Navbar */}
      <Footer items={footerItems} /> {/* HAS Footer */}
      <Panels>                   {/* HAS Panels */}
        <WorkbenchPanel />
        <DetailsPanel />
        <SettingsPanel />
      </Panels>
    </Providers>
  );
}

// Window is COMPOSED OF different views
function Window({ kind }: WindowProps) {
  return (
    <WindowFrame>
      {kind === 'scene' && <Scene3D />}
      {kind === 'diagram' && <Diagram2D />}
      {kind === 'table' && <Table />}
    </WindowFrame>
  );
}
```

**Provider composition** (React Context):

```tsx
// Multiple providers composed together
function Providers({ children }: { children: ReactNode }) {
  return (
    <ThemeProvider>
      <I18nProvider>
        <XStateProvider actor={actor}>
          <LevelProvider level="base">
            <TransactionProvider>
              {children}
            </TransactionProvider>
          </LevelProvider>
        </XStateProvider>
      </I18nProvider>
    </ThemeProvider>
  );
}
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
// js/semio/sketchpad/Sketchpad.tsx
// Store notifies observers when data changes
class Store<TState> {
  private observers: Set<() => void> = new Set();
  
  subscribe(callback: () => void): () => void {
    this.observers.add(callback);
    return () => this.observers.delete(callback);
  }
  
  protected notify(): void {
    for (const observer of this.observers) {
      observer();
    }
  }
}

// React hook subscribes to store changes
function useSync<T>(store: Store<T>, selector: (s: T) => T = identitySelector): T {
  return useSyncExternalStore(
    (onStoreChange) => store.subscribe(onStoreChange),
    () => selector(store.snapshot())
  );
}
```

**Command pattern** (undo/redo):

```typescript
// Every action is an object with do/undo
interface AppEdit<TSelectionDiff> {
  do: AppStep<TSelectionDiff>;
  undo: AppStep<TSelectionDiff>;
}

// Commands are registered and executed by name
registerCommand("semio.designApp.addPiece", (ctx, args) => {
  const piece = createPiece(args);
  return {
    kitDiff: { designs: { updated: [{ diff: { pieces: { added: [piece] } } }] } }
  };
});

executeCommand("semio.designApp.addPiece", origin, { typeGuid, center });
```

**Factory pattern** (store creation):

```typescript
// Factories registered for creating stores on demand
registerDesignAppStoreFactory((kit: KitStore, designGuid: Guid) => {
  return new DesignAppStore(kit, designGuid);
});

// Factory invoked when needed
const store = getDesignAppStoreFactory()(kitStore, designGuid);
```

**Strategy pattern** (file providers):

```typescript
// Different algorithms for file storage, swappable at runtime
interface FileProvider {
  upload: (...) => Promise<string>;
  download: (...) => Promise<Blob>;
}

// Inject strategy based on environment
const provider = isElectron 
  ? new LocalFileProvider() 
  : new RemoteFileProvider();
```

**Registry pattern** (plugin architecture):

```typescript
// App plugins registered dynamically
const appPlugins = new Map<string, AppPlugin>();

function registerAppPlugin(plugin: AppPlugin): void {
  appPlugins.set(plugin.id, plugin);
}

// Event handlers registered dynamically
const eventHandlers = new Map<string, EventHandler>();

function registerEventHandler(eventType: string, handler: EventHandler): void {
  eventHandlers.set(eventType, handler);
}
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
// Each module has one job
// semio.ts: Domain logic ONLY (Kit, Type, Design, diffs, validation)
// Sketchpad.tsx: App orchestration ONLY (routing, state machine, providers)
// Design.tsx: Design editing ONLY (pieces, connections, selection)
// KitStore: Kit persistence ONLY (Y.js operations)

// NOT: Design.tsx handling database writes
// NOT: semio.ts containing React components
```

**O – Open/Closed Principle in semio**:

```typescript
// Apps are added by extension, not modification
// Add new app: create MyApp.tsx, register plugin
registerAppPlugin({
  id: "myapp",
  namespace: "MYAPP",
  machine: { ... },
  createDefaultState: () => ({ ... })
});

// Sketchpad.tsx never modified to add apps
// Event handlers extended, not modified
registerEventHandler("MYAPP.DO_THING", { action: ... });
```

**L – Liskov Substitution in semio**:

```typescript
// All FileProviders are interchangeable
function uploadModel(file: File, provider: FileProvider) {
  return provider.upload(kitId, fileId, file.name, file);
}

// Works with ANY provider
uploadModel(file, new MemoryFileProvider());
uploadModel(file, new LocalFileProvider());
uploadModel(file, new RemoteFileProvider());
```

**I – Interface Segregation in semio**:

```typescript
// Small, focused interfaces
interface FileProvider {
  upload: (...) => Promise<string>;
  download: (...) => Promise<Blob>;
  delete: (...) => Promise<void>;
  getUrl: (...) => string;
}

// NOT: one giant IKitService with 50 methods
// Clients only depend on what they use
```

**D – Dependency Inversion in semio**:

```typescript
// High-level Sketchpad depends on abstract FileProvider
// Not concrete LocalFileProvider

interface RemoteProviders {
  yProvider?: (guid: Guid) => YProvider;  // Abstraction
  fileProvider?: FileProvider;            // Abstraction
}

function Sketchpad({ remoteProviders }: SketchpadProps) {
  // Uses injected abstractions, not concrete implementations
}
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
// Don't render 10,000 pieces at once
// Use virtualization to render only visible items
import { useVirtualizer } from '@tanstack/react-virtual';

function PieceList({ pieces }: { pieces: Piece[] }) {
  const virtualizer = useVirtualizer({
    count: pieces.length,  // 10,000 items
    getScrollElement: () => containerRef.current,
    estimateSize: () => 40,  // Row height
  });
  
  return (
    <div ref={containerRef} style={{ overflow: 'auto', height: 400 }}>
      <div style={{ height: virtualizer.getTotalSize() }}>
        {virtualizer.getVirtualItems().map(virtualRow => (
          // Only render visible rows (~10-20 instead of 10,000)
          <PieceRow key={virtualRow.key} piece={pieces[virtualRow.index]} />
        ))}
      </div>
    </div>
  );
}
```

**DerivedStore caching** (expensive computations):

```typescript
// Cache computed values, recompute only when dependencies change
const piecesMetadataNode = derivedStore.getOrCreate(
  "piecesMetadata",  // Cache key
  [{ store: designStore, path: [yPathMapKey("pieces")] }],  // Dependencies
  () => computePiecesMetadata(designStore.snapshot())  // Expensive computation
);

// Use cached value
const metadata = piecesMetadataNode.snapshot();  // Fast: returns cached
```

**Nx build caching** (monorepo scaling):

```bash
# First build: computes everything
$ npx nx build @semio/js
> @semio/js:build [2m 30s]

# Second build: uses cache (nothing changed)
$ npx nx build @semio/js
> @semio/js:build [retrieved from cache, 0.1s]

# Only rebuild what changed
$ npx nx affected -t build
> Only rebuilding packages affected by changes
```

**Y.js document scaling** (large collaborative documents):

```typescript
// Y.js handles large documents with:
// - Efficient CRDT encoding
// - Delta updates (only send changes)
// - Garbage collection of old operations

// For very large kits, split into subdocuments
const ySubDoc = new Y.Doc();
yDoc.getMap('subDocs').set(designGuid, ySubDoc);
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
// Y.js + IndexedDB = works offline
const yDoc = new Y.Doc();
const persistence = new IndexeddbPersistence(kitGuid, yDoc);

// User edits locally
yDoc.transact(() => {
  yPieces.push([newPiece]);  // Works without network
});

// When network returns, sync automatically
const provider = new WebsocketProvider(serverUrl, kitGuid, yDoc);
provider.on('sync', () => {
  console.log('Synced with server');
});
```

**CRDT conflict resolution**:

```typescript
// Two users edit same design simultaneously
// User A: adds Piece 1
// User B: adds Piece 2
// No conflict! Both pieces added (CRDT merge)

// User A: changes piece.name to "Wall A"
// User B: changes piece.name to "Wall B"
// Last-writer-wins for same field (deterministic by timestamp)
```

**Graceful degradation**:

```typescript
// If remote file provider fails, fall back to local
async function uploadModel(blob: Blob): Promise<string> {
  try {
    return await remoteProvider.upload(kitId, fileId, path, blob);
  } catch (error) {
    console.warn('Remote upload failed, using local storage');
    return await localProvider.upload(kitId, fileId, path, blob);
  }
}

// If 3D rendering fails, show 2D fallback
function ModelView({ model }: { model: Model }) {
  return (
    <ErrorBoundary fallback={<Diagram2D />}>
      <Scene3D model={model} />
    </ErrorBoundary>
  );
}
```

**Data validation on load**:

```typescript
// Validate kit when loading to detect corruption
const kit = loadKitFromStorage(kitGuid);
const validation = validateKit(kit);

if (hasSemioErrors(validation)) {
  console.error('Kit has validation errors:', validation.problems);
  // Apply automatic fixes
  for (const problem of validation.problems) {
    if (problem.fixes.length > 0) {
      kit = applyKitDiff(kit, problem.fixes[0].diff);
    }
  }
}
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
// useMemo: cache expensive computation
const piecesMetadata = useMemo(() => {
  return computePiecesMetadata(pieces, connections);  // Expensive
}, [pieces, connections]);  // Only recompute when these change

// useCallback: cache function reference
const handleSelect = useCallback((pieceGuid: Guid) => {
  actor.send({ type: 'DESIGN.SELECT_PIECE', pieceGuid });
}, [actor]);
```

**Selector optimization** (granular subscriptions):

```typescript
// BAD: subscribe to entire state, re-render on any change
const state = useSelector(actor, (s) => s);

// GOOD: subscribe only to what's needed
const selection = useSelector(actor, (s) => s.context.designApp?.selection);
const theme = useSelector(actor, (s) => s.context.theme);
```

**Snapshot hash caching**:

```typescript
// Store caches snapshots, only rebuild when hash changes
class Store<TState> {
  private cachedSnapshot: TState | null = null;
  private cachedHash: string | null = null;
  
  snapshot(): TState {
    const currentHash = this.hash(this.buildSnapshot());
    if (currentHash === this.cachedHash) {
      return this.cachedSnapshot!;  // Return cached
    }
    this.cachedSnapshot = this.buildSnapshot();
    this.cachedHash = currentHash;
    return this.cachedSnapshot;
  }
}
```

**3D rendering optimization**:

```typescript
// Instanced rendering: one draw call for many identical meshes
const instancedMesh = new THREE.InstancedMesh(geometry, material, pieceCount);

pieces.forEach((piece, index) => {
  const matrix = planeToMatrix4(piece.plane);
  instancedMesh.setMatrixAt(index, matrix);
});

// Level of Detail: simpler geometry for distant objects
const lod = new THREE.LOD();
lod.addLevel(highDetailMesh, 0);     // Full detail up close
lod.addLevel(mediumDetailMesh, 50);  // Medium at 50 units
lod.addLevel(lowDetailMesh, 100);    // Simple at 100+ units
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
// Test pure domain functions in isolation
import { describe, it, expect } from 'vitest';
import { applyKitDiff, getKitDiff, inverseKitDiff } from './semio';

describe('Kit Diff Operations', () => {
  it('applies diff to add a type', () => {
    const kit = { types: [], designs: [] };
    const diff = { types: { added: [{ guid: 'type-1', name: 'Wall' }] } };
    
    const result = applyKitDiff(kit, diff);
    
    expect(result.types).toHaveLength(1);
    expect(result.types[0].name).toBe('Wall');
  });
  
  it('inverse diff reverses the operation', () => {
    const before = { types: [] };
    const after = { types: [{ guid: 'type-1', name: 'Wall' }] };
    
    const diff = getKitDiff(before, after);
    const inverse = inverseKitDiff(before, diff);
    
    const restored = applyKitDiff(after, inverse);
    expect(restored.types).toHaveLength(0);
  });
});
```

**E2E tests** (`js/semio/playwright/`):

```typescript
// Test real user workflows in browser
import { test, expect } from '@playwright/test';

test.describe('design', () => {
  test('seed', async ({ page }) => {
    await page.goto('http://localhost:5173');
    
    // Create temporary kit
    await page.locator('[id="semio.sketchpad.app.home.createTemporary"]').click();
    
    // Create design
    await page.locator('[id="semio.sketchpad.app.kit.kitApp.createDesign"]').click();
    
    // Verify design exists
    await expect(page.getByText('New Design')).toBeVisible();
  });
});
```

**Validation tests** (cross-platform consistency):

```typescript
// Test that TypeScript, Python, C# produce identical validation output
import { validateKit, serializeValidationResult } from './semio';
import invalidKit from '../assets/semio/kit_invalid.json';
import expectedOutput from '../assets/semio/validation.json';

test('validation output matches expected', () => {
  const result = validateKit(invalidKit);
  const serialized = serializeValidationResult(result);
  
  expect(serialized).toEqual(expectedOutput);
});
```

**Nx test orchestration**:

```bash
# Run all tests
npm run test

# Run tests for specific package
npx nx test @semio/js

# Run affected tests only
npx nx affected -t test
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
// All debug logs include [DEBUG] prefix for easy removal
console.log('[DEBUG] [PIECE-DRAG] Mounting Dropzone:', { pieceGuid });
console.log('[DEBUG] [YJSSYNC] Kit synced:', { typeCount: types.length });

// Performance logging for overfetching detection
enablePerformanceLogging(true);
// Logs: [PERF] Rapid re-render detected in <DesignCanvas>
```

**VS Code extension diagnostics**:

```typescript
// js/vscode/extension.ts
// Extension shows validation errors as diagnostics
function updateDiagnostics(document: TextDocument): void {
  const violations = analyzeFile(document.uri.fsPath);
  
  const diagnostics = violations.map(v => ({
    range: new Range(v.line, 0, v.line, 999),
    message: v.message,
    severity: DiagnosticSeverity.Error,
    source: 'semio-repo'
  }));
  
  diagnosticCollection.set(document.uri, diagnostics);
}
```

**Report generation** (CI hooks):

```bash
# hooks/code.ts generates reports/code.json
npx tsx hooks/code.ts

# Output:
{
  "violations": [
    {
      "file": "js/semio/sketchpad/Design.tsx",
      "line": 42,
      "rule": "code:comment:inline",
      "message": "Inline comment detected"
    }
  ]
}

# hooks/typescript.ts generates reports/typescript.json
npx tsx hooks/typescript.ts

# hooks/eslint.ts generates reports/eslint.json
npx tsx hooks/eslint.ts
```

**Performance monitoring hooks**:

```typescript
// Track component render counts
const renderCountRef = useRef(0);
renderCountRef.current++;
console.log(`[DEBUG] [PERF] DesignCanvas render #${renderCountRef.current}`);

// Track effect runs
useEffect(() => {
  console.log('[DEBUG] [EFFECT] Design pieces changed:', pieces.length);
}, [pieces]);
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
// TypeScript: js/semio/semio.ts
export interface Kit {
  guid: Guid;
  name: string;
  types: Type[];
  designs: Design[];
}

// Python: py/engine/engine.py
@dataclass
class Kit:
    guid: str
    name: str
    types: list[Type]
    designs: list[Design]

// C#: net/Semio/Semio.cs
public class Kit {
    public Guid Guid { get; set; }
    public string Name { get; set; }
    public List<Type> Types { get; set; }
    public List<Design> Designs { get; set; }
}

// All serialize to identical JSON:
{
  "guid": "...",
  "name": "Metabolism",
  "types": [...],
  "designs": [...]
}
```

**Schema generation** (single source of truth):

```bash
# Generate schemas from TypeScript definitions
npx tsx py/engine/generate-schemas.ts

# Produces:
# - jsonschema/kit.json (JSON Schema)
# - graphql/semio/schema.graphql (GraphQL)
# - sql/sqlite/schema.sql (SQLite)
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
// js/semio/sketchpad/Design.tsx
export function DesignAppCanvas() {
  // Frontend: State management
  const [pieces, setPieces] = useSyncField(designStore, "pieces");
  const [camera, setCamera] = useDesignAppCamera();
  const [selection, setSelection] = useDesignAppSelection();
  
  // Frontend: 3D rendering
  return (
    <Canvas camera={camera}>
      <Suspense fallback={<Loading3D />}>
        {pieces.map(piece => (
          <PieceGeometry 
            key={piece.guid}
            piece={piece}
            selected={selection.pieces.has(piece.guid)}
            onSelect={() => setSelection({ pieces: new Set([piece.guid]) })}
          />
        ))}
      </Suspense>
    </Canvas>
  );
}
```

**Backend code** (Engine API):

```python
# py/engine/engine.py

@app.post("/graphql")
async def graphql(request: GraphQLRequest) -> GraphQLResponse:
    """Backend: Handle GraphQL queries"""
    result = await schema.execute(
        request.query,
        variable_values=request.variables,
        context_value={"db": database}
    )
    return GraphQLResponse(data=result.data, errors=result.errors)

@app.get("/kit/{kit_id}")
async def get_kit(kit_id: str) -> Kit:
    """Backend: Retrieve kit from SQLite"""
    async with aiosqlite.connect(f".semio/kit.db") as db:
        cursor = await db.execute("SELECT * FROM kits WHERE guid = ?", [kit_id])
        row = await cursor.fetchone()
        return Kit.from_row(row)
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
CREATE TABLE kits (
    guid TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    version TEXT
);

CREATE TABLE types (
    guid TEXT PRIMARY KEY,
    kit_guid TEXT REFERENCES kits(guid),
    name TEXT NOT NULL,
    parent_guid TEXT REFERENCES types(guid)
);

CREATE TABLE designs (
    guid TEXT PRIMARY KEY,
    kit_guid TEXT REFERENCES kits(guid),
    name TEXT NOT NULL,
    parent_guid TEXT REFERENCES designs(guid)
);

CREATE TABLE pieces (
    guid TEXT PRIMARY KEY,
    design_guid TEXT REFERENCES designs(guid),
    type_guid TEXT REFERENCES types(guid),
    name TEXT,
    plane_origin_x REAL,
    plane_origin_y REAL,
    plane_origin_z REAL
);

CREATE TABLE connections (
    guid TEXT PRIMARY KEY,
    design_guid TEXT REFERENCES designs(guid),
    connected_piece_guid TEXT REFERENCES pieces(guid),
    connecting_piece_guid TEXT REFERENCES pieces(guid),
    gap REAL,
    shift REAL,
    rotation REAL
);
```

**IndexedDB for Y.js persistence**:

```typescript
// js/semio/sketchpad/Sketchpad.tsx
import { IndexeddbPersistence } from 'y-indexeddb';

function createKitStore(kitGuid: Guid): KitStore {
  const yDoc = new Y.Doc();
  
  // Persist to IndexedDB
  const persistence = new IndexeddbPersistence(`semio-kit-${kitGuid}`, yDoc);
  
  // Y.js document structure
  const yTypes = yDoc.getArray<Y.Map<any>>('types');
  const yDesigns = yDoc.getArray<Y.Map<any>>('designs');
  
  return new KitStore(yDoc, yTypes, yDesigns);
}
```

**File storage for assets**:

```typescript
// js/semio/sketchpad/Sketchpad.tsx

class FileProvider {
  async upload(kitId: string, fileId: string, path: string, blob: Blob): Promise<string> {
    const filePath = `.semio/files/${kitId}/${fileId}/${path}`;
    await fs.writeFile(filePath, Buffer.from(await blob.arrayBuffer()));
    return filePath;
  }
  
  async download(kitId: string, fileId: string, path: string): Promise<Blob> {
    const filePath = `.semio/files/${kitId}/${fileId}/${path}`;
    const buffer = await fs.readFile(filePath);
    return new Blob([buffer]);
  }
  
  getUrl(kitId: string, fileId: string, path: string): string {
    return `.semio/files/${kitId}/${fileId}/${path}`;
  }
}
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
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "kit.json",
  "type": "object",
  "properties": {
    "guid": { "type": "string", "format": "uuid" },
    "name": { "type": "string" },
    "types": {
      "type": "array",
      "items": { "$ref": "#/$defs/Type" }
    },
    "designs": {
      "type": "array",
      "items": { "$ref": "#/$defs/Design" }
    }
  },
  "required": ["guid", "name"]
}
```

**GraphQL API** (graphql/semio/schema.graphql):

```graphql
type Query {
  kit(guid: ID!): Kit
  type(guid: ID!): Type
  design(guid: ID!): Design
  piece(guid: ID!): Piece
}

type Mutation {
  createType(input: CreateTypeInput!): Type!
  updateType(guid: ID!, input: UpdateTypeInput!): Type!
  deleteType(guid: ID!): Boolean!
  
  createDesign(input: CreateDesignInput!): Design!
  placePiece(designGuid: ID!, input: PlacePieceInput!): Piece!
}

type Kit {
  guid: ID!
  name: String!
  types: [Type!]!
  designs: [Design!]!
}

type Type {
  guid: ID!
  name: String!
  connectors: [Connector!]!
  models: [Model!]!
}
```

**MCP tool protocol** (go/mcp/main.go):

```json
{
  "name": "analyze",
  "description": "Analyze codebase for policy violations",
  "inputSchema": {
    "type": "object",
    "properties": {
      "scope": {
        "type": "string",
        "description": "Scope to analyze (file, folder, or bundle)"
      }
    }
  }
}
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
# .github/workflows/gh-pages.yml
name: Deploy Documentation

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: '22'
      
      - name: Install dependencies
        run: npm ci
      
      - name: Run preflight checks
        run: npm run preflight
      
      - name: Build documentation
        run: npm run build -- --projects=@semio/docs
      
      - name: Deploy to GitHub Pages
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./js/docs/dist
```

**Build orchestration** (nx.json):

```json
{
  "targetDefaults": {
    "build": {
      "dependsOn": ["^build"],
      "cache": true
    },
    "test": {
      "dependsOn": ["build"],
      "cache": true
    }
  },
  "plugins": [
    "@nx/js"
  ]
}
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
// js/semio/semio.ts
import { z } from 'zod';

export const pieceSchema = z.object({
  guid: z.string().uuid(),
  name: z.string().optional(),
  typeGuid: z.string().uuid(),
  plane: planeSchema.optional(),
  center: pointSchema.optional()
});

function validatePiece(input: unknown): Piece {
  // Validates and throws on invalid input
  return pieceSchema.parse(input);
}
```

**Kit validation**:

```typescript
// js/semio/semio.ts
export function validateKit(kit: Kit): ValidationResult {
  const problems: Problem[] = [];
  
  // GUID uniqueness
  const guids = new Set<string>();
  for (const type of kit.types) {
    if (guids.has(type.guid)) {
      problems.push({
        constraintId: 'guid-unique',
        severity: 'error',
        message: `Duplicate GUID: ${type.guid}`,
        location: { entityKind: 'Type', entityGuid: type.guid }
      });
    }
    guids.add(type.guid);
  }
  
  return { problems };
}
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
# py/engine/generate-schemas.ts
npx tsx py/engine/generate-schemas.ts

# Generates:
# - jsonschema/kit.json (for validation)
# - graphql/semio/schema.graphql (for API)
# - sql/sqlite/schema.sql (for storage)
```

**Documentation validation** (hooks/code.ts):

```typescript
// Validates that all documented mechanisms exist in code
function validateDocumentation(): ValidationReport {
  const agents = parseMarkdown('AGENTS.md');
  const readme = parseMarkdown('README.md');
  
  const problems: Problem[] = [];
  
  // Check that documented files exist
  for (const ref of agents.fileReferences) {
    if (!fileExists(ref.path)) {
      problems.push({
        id: 'doc:missing-file',
        message: `Documented file does not exist: ${ref.path}`
      });
    }
  }
  
  return { problems };
}
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
# Sketchpad Refactor Plan

## Current State
- Mixed XState and Zustand state management
- Inconsistent hook patterns
- Legacy uiMachine still referenced

## Target State
- Unified XState for all UI state
- Triadic hook pattern [value, setter, canSet]
- Single sketchpadMachine

## Migration Steps
1. [ ] Consolidate to single machine
2. [ ] Remove uiMachine references
3. [ ] Update all hooks to triadic pattern
4. [ ] Remove Zustand dependencies
```

**Gradual migration pattern**:

```typescript
// Step 1: Create new pattern alongside old
export function useDesignAppSelection(): HookResult<DesignSelection> {
  // NEW: XState-based
  const actor = useSketchpadActor();
  const selection = useSelector(actor, selectDesignSelection);
  
  // OLD: Legacy zustand (kept for gradual migration)
  // const selection = useDesignStore(s => s.selection);
  
  return [selection, setSelection, canSet];
}

// Step 2: Update consumers one by one
// Step 3: Remove old implementation
```

**Ticket system for debt**:

```yaml
# tickets/2025/06/15/HOOK-MIGRATION/ticket.md
---
slug: HOOK-MIGRATION
status: open
summary: Migrate legacy hooks to triadic pattern
---

## Tasks
- [ ] Update useDesignAppSelection
- [ ] Update useTypeAppSelection  
- [ ] Update useKitAppSelection
- [ ] Remove deprecated hook exports
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
// Single source of truth for UI state
const sketchpadMachine = createMachine({
  id: 'sketchpad',
  type: 'parallel',
  context: {
    theme: 'system',
    language: 'en',
    expertise: 'normal',
    // App-specific state slices
    homeApp: { ... },
    kitApp: { ... },
    designApp: { ... },
    typeApp: { ... },
  },
  states: {
    navigation: {
      states: {
        home: { ... },
        kit: { ... },
        design: { ... },
        type: { ... },
      }
    }
  }
});

// Components read state via selectors
const selection = useSelector(actor, (s) => s.context.designApp.selection);

// Components modify state via events
actor.send({ type: 'DESIGN.SELECT_PIECE', pieceGuid: '...' });
```

**Real-time collaboration with Y.js**:

```typescript
// Kit data stored in Y.js documents (CRDT)
const yDoc = new Y.Doc();
const yTypes = yDoc.getArray('types');  // Synchronized across users

// Changes automatically sync to all connected users
yTypes.push([newType]);  // Other users see this immediately

// Conflict resolution is automatic
// User A adds type at index 0
// User B adds type at index 0
// Result: both types added, order deterministic
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
# FastAPI provides the web framework
from fastapi import FastAPI
from pydantic import BaseModel

app = FastAPI()

# Models generated from JSON schemas
class Kit(BaseModel):
    name: str
    version: str
    types: List[Type]
    designs: List[Design]
    # ... validated automatically by Pydantic

@app.post("/kits/validate")
async def validate_kit(kit: Kit) -> ValidationResult:
    """Validate a kit and return problems with fixes."""
    return validate_semio_kit(kit)

@app.get("/kits/{kit_id}")
async def get_kit(kit_id: str) -> Kit:
    """Retrieve a kit from storage."""
    return storage.load_kit(kit_id)
```

**Storage format** (SQLite in .semio ZIP files):

```sql
-- Each kit is a ZIP containing kit.db (SQLite)
-- Schema from sql/sqlite/schema.sql

CREATE TABLE types (
    guid TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    variant TEXT,
    is_virtual INTEGER DEFAULT 0,
    -- ... other fields
);

CREATE TABLE connectors (
    guid TEXT PRIMARY KEY,
    type_guid TEXT REFERENCES types(guid),
    name TEXT,
    point_x REAL,
    point_y REAL,
    point_z REAL,
    -- ... other fields
);
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
interface Kit {
  // Identity
  guid: Guid;              // Unique identifier
  name: string;            // Human-readable name
  version: string;         // Semantic version

  // Components
  types: Type[];           // Reusable component definitions
  designs: Design[];       // Assemblies using types
  
  // Standards
  qualities: Quality[];    // Measurement definitions
  interfaces: Interface[]; // Connector compatibility rules
  
  // Organization
  files: File[];           // Associated documents
  folders: Folder[];       // Organizational structure
  tags: Tag[];             // Categorization
  concepts: Concept[];     // Semantic grouping
  authors: Author[];       // Attribution
  
  // Metadata
  description?: string;
  icon?: string;
  image?: string;
  attributes: Attribute[];
}
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
interface Type {
  guid: Guid;
  name: string;
  variant?: string;           // Variations: "Wall" → "Wall-Corner"
  
  // 3D representations
  models: Model[];            // GLB, OBJ, etc.
  
  // Connection points
  connectors: Connector[];    // Where this type attaches to others
  
  // Properties
  props: Prop[];              // Measured characteristics
  
  // Flags
  isVirtual?: boolean;        // Abstract type, not directly usable
  canScale?: boolean;         // Can be resized
  canMirror?: boolean;        // Can be mirrored
  
  // Metadata
  unit?: string;              // Measurement unit
  availableCount?: number;    // Stock quantity
  location?: Location;        // Physical location
  authors: Author[];
  attributes: Attribute[];
}
```

**Connectors define attachment points**:

```typescript
interface Connector {
  id: string;                 // Unique within type
  name?: string;              // "Left", "Top", "Front"
  
  // Position and orientation
  point: Point;               // Location on type
  direction: Vector;          // Outward direction
  
  // Compatibility
  interface?: Interface;      // What can connect here
  mandatory?: boolean;        // Must be connected
  
  // Diagram positioning
  t?: number;                 // Position on ring (0-1)
}
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
interface Design {
  guid: Guid;
  name: string;
  variant?: string;
  
  // Components
  pieces: Piece[];              // Instances of types
  connections: Connection[];    // How pieces attach
  
  // Organization
  layers: Layer[];              // Visual/logical grouping
  groups: Group[];              // Semantic grouping
  
  // Analytics
  stats: Stat[];                // Computed metrics
  
  // Settings
  canScale?: boolean;
  canMirror?: boolean;
  
  // Metadata
  view?: Camera;                // Default view
  authors: Author[];
  attributes: Attribute[];
}
```

**Pieces and Connections**:

```typescript
interface Piece {
  id: string;                   // Unique within design
  name?: string;                // "Living Room Module"
  
  // What it is
  type?: TypeReference;         // Reference to a type
  design?: DesignReference;     // Or a sub-design
  
  // Placement (either explicit or computed)
  plane?: Plane;                // Fixed position/orientation
  center?: Point;               // Diagram position
  
  // Modifications
  scale?: number;               // Size multiplier
  mirrorPlane?: Plane;          // Mirror transformation
  
  // State
  isHidden?: boolean;
  isLocked?: boolean;
  color?: string;
}

interface Connection {
  connected: Side;              // One piece + connector
  connecting: Side;             // Other piece + connector
  
  // Adjustments
  gap?: number;                 // Y offset
  shift?: number;               // X offset
  rise?: number;                // Z offset
  rotation?: number;            // Around Y
  turn?: number;                // Around Z
  tilt?: number;                // Around X
}
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
// Y.js uses Conflict-free Replicated Data Types
// Changes merge automatically without central coordination

// User A's action
yPieces.get('piece-123').set('name', 'Living Room');

// User B's action (same time, different property)
yPieces.get('piece-123').set('color', '#ff0000');

// Result: Both changes apply
// { name: 'Living Room', color: '#ff0000' }

// User A and B both change name simultaneously
yPieces.get('piece-123').set('name', 'Kitchen');    // User A
yPieces.get('piece-123').set('name', 'Bedroom');    // User B

// Result: Deterministic winner (last-writer-wins with vector clocks)
// Both users see the same final value
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
// Show who's working on what
const awareness = new Awareness(yDoc);

awareness.setLocalStateField('user', {
  name: 'Alice',
  color: '#ff6b6b',
  cursor: { x: 100, y: 200 },
  selection: ['piece-123', 'piece-456']
});

// Subscribe to other users
awareness.on('change', () => {
  const users = Array.from(awareness.getStates().values());
  // Render cursors and selections for all users
});
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
// Pure domain validation - no JSON dependencies
interface Problem {
  constraintId: string;           // "type-name-unique"
  severity: 'error' | 'warning';
  message: string;
  location: {
    entityKind: 'Type' | 'Design' | 'Piece' | 'Connection' | ...;
    entityGuid?: Guid;
    field?: string;
  };
  fixes: Fix[];                   // Suggested corrections
}

interface Fix {
  title: string;                  // "Rename to 'Wall 2'"
  diff: KitDiff;                  // Change to apply
}

// Validation function
function validateSemioKit(kit: Kit): ValidationResult {
  const problems: Problem[] = [];
  const ctx = buildValidationContext(kit);
  
  for (const constraint of constraints) {
    problems.push(...constraint(ctx));
  }
  
  return { problems };
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
// Problems include automated fixes
{
  constraintId: 'type-name-unique',
  severity: 'error',
  message: 'Duplicate type name "Wall" among siblings',
  location: { entityKind: 'Type', entityGuid: 'abc-123' },
  fixes: [
    {
      title: 'Rename to "Wall 2"',
      diff: {
        types: {
          updated: [{ guid: 'abc-123', name: 'Wall 2' }]
        }
      }
    }
  ]
}

// Applying a fix
const fixedKit = applyKitDiff(kit, problem.fixes[0].diff);
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
semio/                          # Single Git repository
├── package.json                # Root: Nx workspace config
├── nx.json                     # Nx build orchestration
├── tsconfig.json               # TypeScript config
│
├── js/                         # JavaScript/TypeScript packages
│   ├── semio/                  # @semio/js - core domain
│   ├── sketchpad/              # @semio/sketchpad - UI app
│   ├── vscode/                 # @semio/vscode - editor extension
│   └── docs/                   # @semio/docs - documentation
│
├── py/                         # Python packages
│   └── engine/                 # @semio/engine - backend
│
├── net/                        # C#/.NET packages
│   ├── Semio/                  # Core library
│   └── Semio.Grasshopper/      # Grasshopper plugin
│
├── go/                         # Go packages
│   ├── repo/                   # Repository CLI
│   └── mcp/                    # MCP server
│
├── jsonschema/                 # Generated from TypeScript
├── graphql/                    # API schemas
└── sql/                        # Database schemas
```

**Nx build orchestration**:

```bash
# Single commands build/test everything
npm run build          # Build all packages, respecting dependencies
npm run test           # Run all tests
npm run dev            # Start all dev servers

# Affected-only commands (only what changed)
nx affected:build      # Only build what changed
nx affected:test       # Only test what changed

# Dependency graph
nx graph               # Visual dependency explorer
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
# Single commit changes all affected languages
git add js/semio/semio.ts    # Add field to Type
git add jsonschema/          # Updated JSON schema
git add py/engine/           # Updated Python models
git add net/Semio/           # Updated C# models
git commit -m "Add 'location' field to Type"
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
// Each app registers as a plugin
interface AppPlugin {
  id: string;                     // "design", "type", "quality"
  namespace: string;              // "DESIGN", "TYPE", "QUALITY"
  
  machine: {
    // XState contributions
    actions: Record<string, ActionFunction>;
    guards: Record<string, GuardFunction>;
    eventHandlers: Record<string, EventHandler>;
    selectors: Record<string, SelectorFunction>;
  };
  
  createDefaultState: () => AppState;
  registerStores?: () => void;
}

// Registration (side effect on module load)
const designAppPlugin: AppPlugin = {
  id: 'design',
  namespace: 'DESIGN',
  machine: {
    eventHandlers: {
      'DESIGN.SELECT_PIECE': {
        action: (ctx, event) => ({
          designApp: {
            ...ctx.designApp,
            selection: { pieces: [...event.guids] }
          }
        })
      }
    }
  },
  createDefaultState: () => ({ ... }),
};

registerAppPlugin(designAppPlugin);
```

**Adding a new app** (no core changes):

```typescript
// 1. Create app file: js/semio/sketchpad/MyApp.tsx

// 2. Define plugin
const myAppPlugin: AppPlugin = {
  id: 'myapp',
  namespace: 'MYAPP',
  machine: {
    eventHandlers: {
      'MYAPP.DO_SOMETHING': {
        action: (ctx, event) => ({ ... })
      }
    }
  },
  createDefaultState: () => ({ ... }),
};

// 3. Register
if (typeof window !== 'undefined') {
  registerAppPlugin(myAppPlugin);
}

// 4. Export component
export default function MyApp() {
  return <div>My custom app</div>;
}

// Done! No changes to Sketchpad.tsx required
```

**Panel section registration**:

```typescript
// Apps dynamically add panel sections
useEffect(() => {
  addSection('details', {
    id: 'my-section',
    label: t('mySection'),
    content: () => <MyComponent />,
    order: 1,
  });
  
  return () => removeSection('details', 'my-section');
}, []);
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
// State machine defines valid states and transitions
const sketchpadMachine = createMachine({
  context: {
    theme: 'system',
    designApp: { selection: { pieces: [] }, hover: null },
    // ...
  },
  on: {
    'SET_THEME': { actions: assign({ theme: (_, e) => e.theme }) },
    'DESIGN.SELECT_PIECE': { 
      actions: assign({
        designApp: (ctx, e) => ({
          ...ctx.designApp,
          selection: { pieces: e.guids }
        })
      })
    },
  }
});

// Components read with selectors
const selection = useSelector(actor, s => s.context.designApp.selection);

// Components write with events
const selectPiece = (guid) => actor.send({ 
  type: 'DESIGN.SELECT_PIECE', 
  guids: [guid] 
});
```

**Triadic hook pattern** (`[value, setValue, canSet]`):

```typescript
// All hooks follow consistent pattern
function useDesignAppSelection(): HookResult<Selection> {
  const actor = useSketchpadActor();
  
  const selection = useSelector(actor, 
    s => s.context.designApp?.selection ?? EMPTY_SELECTION
  );
  
  const canSet = useSelector(actor, 
    s => s.can({ type: 'DESIGN.SELECT_PIECE', guids: [] })
  );
  
  const setSelection = canSet
    ? (sel) => actor.send({ type: 'DESIGN.SELECT_PIECE', guids: sel.pieces })
    : undefined;
  
  return [selection, setSelection, canSet];
}

// Usage in components
const [selection, setSelection, canSetSelection] = useDesignAppSelection();
if (canSetSelection) {
  setSelection({ pieces: ['abc'] });
}
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
