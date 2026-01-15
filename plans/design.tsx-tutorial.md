Understanding Design.tsx

## How To Use This Document

This is a **complete programming course** disguised as a code explanation. If you've never programmed before, start at Chapter 0 and work through every section in order. If you have some experience, use the table of contents to jump to what you need.

**What You'll Learn**:

1. How programming works (from zero)
2. JavaScript and TypeScript fundamentals
3. React and component-based UI development
4. How professional-grade applications are structured
5. How to read and understand any React codebase

---

# 📖 CHAPTER 0 — Before You Begin: Programming Fundamentals

> **This chapter is for absolute beginners.** If you know what variables, functions, and loops are, skip to Stage 1.

## What Is Programming?

Programming is giving instructions to a computer. That's it. The computer is extremely fast but extremely literal — it does exactly what you tell it, nothing more, nothing less.

Think of it like writing a recipe for someone who has never cooked before and takes everything 100% literally:

**Bad recipe**: "Make pasta"
**Good recipe**:

1. Fill a pot with 4 liters of water
2. Place the pot on the stove
3. Turn the burner to HIGH
4. Wait until you see bubbles rising continuously (this is "boiling")
5. Add 500 grams of pasta
6. Set a timer for 10 minutes
7. When the timer rings, turn off the burner
8. Pour the contents through a strainer over the sink
9. The pasta is ready

Programming is writing those detailed instructions, but for computers instead of humans.

## What Is Code?

**Code** is the text that contains your instructions. It's written in a **programming language** — a special way of writing that both humans and computers can understand.

Here's a piece of code in JavaScript (the language we'll be learning):

```javascript
let message = "Hello, world!";
console.log(message);
```

This code does two things:

1. Creates a container called `message` and puts the text "Hello, world!" inside it
2. Displays that message on the screen

## The Three Things Every Program Does

Every program, from a simple calculator to a complex video game, does three things:

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   INPUT     │────▶│  PROCESSING │────▶│   OUTPUT    │
│             │     │             │     │             │
│ What goes   │     │ What the    │     │ What comes  │
│ into the    │     │ computer    │     │ out of the  │
│ program     │     │ does with   │     │ program     │
│             │     │ the input   │     │             │
└─────────────┘     └─────────────┘     └─────────────┘

Examples:
• Calculator: numbers → math → result
• Word processor: keystrokes → formatting → document
• Video game: controller inputs → physics/AI → graphics/sound
• Design.tsx: mouse clicks → update design → 2D/3D views
```

## Fundamental Concept 1: Variables

### What Is a Variable?

A **variable** is a labeled box that holds information. You give the box a name, and you can put things in it, look at what's inside, or change its contents.

```
┌──────────────────────────────────────────────────────┐
│                      VARIABLES                        │
├──────────────────────────────────────────────────────┤
│                                                       │
│   ┌───────────┐    ┌───────────┐    ┌───────────┐    │
│   │   name    │    │    age    │    │  isHappy  │    │
│   ├───────────┤    ├───────────┤    ├───────────┤    │
│   │  "Alice"  │    │    25     │    │   true    │    │
│   └───────────┘    └───────────┘    └───────────┘    │
│                                                       │
│   Box label        Box label        Box label         │
│   (the name)       (the name)       (the name)        │
│                                                       │
│   Box contents     Box contents     Box contents      │
│   (the value)      (the value)      (the value)       │
│                                                       │
└──────────────────────────────────────────────────────┘
```

### How To Create a Variable

In JavaScript/TypeScript, you create variables using these keywords:

```javascript
// 'let' creates a variable that can change
let score = 0;
score = 10; // OK: score is now 10
score = 25; // OK: score is now 25

// 'const' creates a variable that can NOT change
const pi = 3.14159;
// pi = 3;  // ERROR: you can't change a const

// 'var' is the old way (avoid using this)
var oldStyle = "don't use this";
```

### Variable Naming Rules

```javascript
// ✅ Good variable names
let userName = "alice";
let totalPrice = 99.99;
let isLoggedIn = true;
let numberOfItems = 5;

// ❌ Bad variable names (but technically valid)
let x = "alice"; // Too vague
let tp = 99.99; // Abbreviations are confusing
let thing = true; // Meaningless name

// ❌ Invalid variable names (these will cause errors)
// let 123abc = "nope";    // Can't start with a number
// let my-name = "nope";   // Can't have dashes
// let my name = "nope";   // Can't have spaces
```

### Types of Data Variables Can Hold

```javascript
// 1. STRINGS (text) - wrapped in quotes
let greeting = "Hello, world!";
let name = "Alice"; // Single or double quotes both work
let template = `My name is ${name}`; // Backticks allow inserting variables

// 2. NUMBERS - no quotes needed
let age = 25;
let price = 19.99;
let negative = -10;

// 3. BOOLEANS - true or false (no quotes!)
let isRaining = true;
let hasPermission = false;

// 4. NULL - intentionally empty
let selectedItem = null; // "Nothing is selected"

// 5. UNDEFINED - not yet assigned
let futureValue; // This is undefined

// 6. ARRAYS - lists of things (use square brackets)
let colors = ["red", "green", "blue"];
let numbers = [1, 2, 3, 4, 5];

// 7. OBJECTS - collections of named values (use curly braces)
let person = {
  name: "Alice",
  age: 25,
  isStudent: true,
};
```

### Practice: Variables

Try to predict what each line outputs:

```javascript
let a = 5;
let b = 10;
let c = a + b;
console.log(c); // What does this print?

let name = "Bob";
let greeting = "Hello, " + name + "!";
console.log(greeting); // What does this print?

let x = 10;
x = x + 5;
x = x * 2;
console.log(x); // What does this print?
```

<details>
<summary>Click to see answers</summary>

```
15
Hello, Bob!
30
```

</details>

---

## Fundamental Concept 2: Functions

### What Is a Function?

A **function** is a reusable set of instructions with a name. Instead of writing the same code over and over, you write it once inside a function and then "call" that function whenever you need it.

Think of a function like a recipe card:

```
┌─────────────────────────────────────────────────────────┐
│                    RECIPE: Greet Person                  │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  INGREDIENTS NEEDED (Parameters):                        │
│  • name (text)                                           │
│                                                          │
│  STEPS (Code):                                           │
│  1. Take the name                                        │
│  2. Put "Hello, " in front of it                         │
│  3. Put "!" at the end                                   │
│                                                          │
│  RESULT (Return value):                                  │
│  • The complete greeting                                 │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

### How To Create a Function

```javascript
// Method 1: Function Declaration
function greet(name) {
  return "Hello, " + name + "!";
}

// Method 2: Arrow Function (modern style)
const greet = (name) => {
  return "Hello, " + name + "!";
};

// Method 3: Arrow Function (shorthand for simple functions)
const greet = (name) => "Hello, " + name + "!";
```

### How To Use (Call) a Function

```javascript
// Define the function
function add(a, b) {
  return a + b;
}

// Call the function
let result = add(5, 3);
console.log(result); // Prints: 8

// You can call it many times with different inputs
console.log(add(10, 20)); // Prints: 30
console.log(add(1, 1)); // Prints: 2
```

### Functions That Don't Return Anything

Some functions just DO something instead of calculating a result:

```javascript
// This function doesn't return anything - it just displays text
function sayHello(name) {
  console.log("Hello, " + name + "!");
  // No 'return' statement
}

sayHello("Alice"); // Prints: Hello, Alice!
sayHello("Bob"); // Prints: Hello, Bob!
```

### Functions Inside Functions

Functions can call other functions:

```javascript
function square(x) {
  return x * x;
}

function sumOfSquares(a, b) {
  return square(a) + square(b);
}

let result = sumOfSquares(3, 4);
console.log(result); // Prints: 25 (because 9 + 16 = 25)
```

### Practice: Functions

```javascript
// Exercise 1: What does this print?
function double(n) {
  return n * 2;
}
console.log(double(double(5)));

// Exercise 2: Write a function that takes two numbers
// and returns the larger one

// Exercise 3: Write a function called 'isEven' that
// returns true if a number is even, false if odd
```

<details>
<summary>Click to see answers</summary>

```javascript
// Exercise 1: 20 (double(5) = 10, then double(10) = 20)

// Exercise 2:
function max(a, b) {
  if (a > b) {
    return a;
  } else {
    return b;
  }
}

// Exercise 3:
function isEven(n) {
  return n % 2 === 0;
}
```

</details>

---

## Fundamental Concept 3: Control Flow

### What Is Control Flow?

**Control flow** determines which code runs and when. Without control flow, code just runs top to bottom. With control flow, the code can make decisions and repeat actions.

### If/Else: Making Decisions

```javascript
let temperature = 25;

if (temperature > 30) {
  console.log("It's hot!");
} else if (temperature > 20) {
  console.log("It's nice!");
} else {
  console.log("It's cold!");
}
// Prints: "It's nice!" (because 25 > 20 but not > 30)
```

### Comparison Operators

```javascript
// These all produce true or false

5 === 5; // true  (equal to)
5 !== 3; // true  (not equal to)
5 > 3; // true  (greater than)
5 < 3; // false (less than)
5 >= 5; // true  (greater than or equal)
5 <= 3; // false (less than or equal)

// ⚠️ WARNING: Use === not ==
5 == "5"; // true  (BAD: loose comparison)
5 === "5"; // false (GOOD: strict comparison)
```

### Loops: Repeating Actions

```javascript
// FOR loop: repeat a specific number of times
for (let i = 0; i < 5; i++) {
  console.log("Count: " + i);
}
// Prints: Count: 0, Count: 1, Count: 2, Count: 3, Count: 4

// WHILE loop: repeat while a condition is true
let countdown = 5;
while (countdown > 0) {
  console.log(countdown);
  countdown = countdown - 1;
}
console.log("Blast off!");
// Prints: 5, 4, 3, 2, 1, Blast off!

// FOR...OF loop: go through each item in an array
let fruits = ["apple", "banana", "cherry"];
for (let fruit of fruits) {
  console.log("I like " + fruit);
}
// Prints: I like apple, I like banana, I like cherry
```

---

## Fundamental Concept 4: Objects

### What Is an Object?

An **object** is a collection of related data grouped together. Think of it like a contact card:

```
┌─────────────────────────────────────────┐
│              CONTACT CARD                │
├─────────────────────────────────────────┤
│  Name:      Alice Johnson               │
│  Age:       28                          │
│  Email:     alice@example.com           │
│  Phone:     555-1234                    │
│  Active:    Yes                         │
└─────────────────────────────────────────┘
```

In JavaScript, this becomes:

```javascript
let contact = {
  name: "Alice Johnson",
  age: 28,
  email: "alice@example.com",
  phone: "555-1234",
  active: true,
};
```

### Accessing Object Properties

```javascript
let person = {
  name: "Alice",
  age: 25,
};

// Method 1: Dot notation (most common)
console.log(person.name); // "Alice"
console.log(person.age); // 25

// Method 2: Bracket notation (for dynamic keys)
console.log(person["name"]); // "Alice"

let key = "age";
console.log(person[key]); // 25
```

### Nested Objects

Objects can contain other objects:

```javascript
let company = {
  name: "Tech Corp",
  address: {
    street: "123 Main St",
    city: "New York",
    country: "USA",
  },
  employees: [
    { name: "Alice", role: "Developer" },
    { name: "Bob", role: "Designer" },
  ],
};

// Accessing nested data
console.log(company.address.city); // "New York"
console.log(company.employees[0].name); // "Alice"
```

### Why Objects Matter in Design.tsx

The entire Design App is built on objects. A "Piece" is an object. A "Connection" is an object. The entire "DesignAppState" is a giant object containing the current state of the application:

```typescript
// This is an object that describes the current state of the design app
let designAppState = {
  selection: {
    pieces: ["piece-123", "piece-456"],
    connections: [],
  },
  hover: {
    pieces: ["piece-789"],
  },
  camera: {
    position: { x: 0, y: 0, z: 10 },
    forward: { x: 0, y: 0, z: -1 },
  },
  activeTool: "selection",
};
```

---

## Fundamental Concept 5: Arrays

### What Is an Array?

An **array** is an ordered list of things. Each item has a position number (called an "index"), starting from 0.

```
┌─────────────────────────────────────────────────────────┐
│                        ARRAY                             │
├─────────────────────────────────────────────────────────┤
│                                                          │
│   Index:    0          1          2          3           │
│           ┌────┐     ┌────┐     ┌────┐     ┌────┐       │
│   Value:  │ 🍎 │     │ 🍌 │     │ 🍒 │     │ 🍇 │       │
│           └────┘     └────┘     └────┘     └────┘       │
│                                                          │
│   fruits[0] = 🍎    fruits[2] = 🍒                       │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

### Working With Arrays

```javascript
// Create an array
let fruits = ["apple", "banana", "cherry"];

// Access items (remember: first item is index 0!)
console.log(fruits[0]); // "apple"
console.log(fruits[1]); // "banana"
console.log(fruits[2]); // "cherry"

// How many items?
console.log(fruits.length); // 3

// Add to the end
fruits.push("date"); // ["apple", "banana", "cherry", "date"]

// Remove from the end
let last = fruits.pop(); // last = "date", fruits = ["apple", "banana", "cherry"]

// Find an item
let index = fruits.indexOf("banana"); // 1

// Check if item exists
let hasApple = fruits.includes("apple"); // true
```

### Array Methods (Very Important!)

These methods are used constantly in React:

```javascript
let numbers = [1, 2, 3, 4, 5];

// MAP: Transform each item
let doubled = numbers.map((n) => n * 2);
// doubled = [2, 4, 6, 8, 10]

// FILTER: Keep only items that pass a test
let bigOnes = numbers.filter((n) => n > 3);
// bigOnes = [4, 5]

// FIND: Get the first item that passes a test
let firstBig = numbers.find((n) => n > 3);
// firstBig = 4

// SOME: Check if ANY item passes a test
let hasEven = numbers.some((n) => n % 2 === 0);
// hasEven = true

// EVERY: Check if ALL items pass a test
let allPositive = numbers.every((n) => n > 0);
// allPositive = true

// REDUCE: Combine all items into one value
let sum = numbers.reduce((total, n) => total + n, 0);
// sum = 15
```

### Why Arrays Matter in Design.tsx

The Design App uses arrays everywhere:

- `pieces: Piece[]` — list of all pieces in a design
- `connections: Connection[]` — list of all connections
- `selection.pieces: string[]` — list of selected piece IDs

---

## Now You're Ready

You now understand:

- ✅ Variables (storing data)
- ✅ Functions (reusable code)
- ✅ Control flow (decisions and loops)
- ✅ Objects (structured data)
- ✅ Arrays (lists of data)

These five concepts are the foundation of ALL programming. Everything else builds on top of them.

---

# 🎯 STAGE 1 — Big Picture

## What Is This Program?

Imagine you're building with LEGO blocks, but digitally. This program is a **visual design editor** that lets you:

1. **Place building blocks** (called "pieces") on a canvas
2. **Connect those blocks together** using connection points (like LEGO studs)
3. **View your creation** in both 2D (a diagram) and 3D (a 3D model viewer)
4. **Edit properties** of each piece and connection

## What Problem Does It Solve?

Architects, engineers, and designers need to create modular systems - buildings made of prefabricated parts, furniture systems, or mechanical assemblies. This tool lets them:

- Design how parts fit together
- Visualize the result instantly
- Make changes and see updates in real-time
- Collaborate with others

## What Does It Produce?

- **A "Design"**: A collection of pieces and their connections
- **Visual representations**: 2D diagrams showing how pieces connect, 3D views showing what it looks like
- **Data**: Structured information that can be saved, shared, or used in manufacturing

## How Does Someone Use It?

```
┌─────────────────────────────────────────────────────┐
│                    SKETCHPAD APP                     │
├─────────────────────────────────────────────────────┤
│  [Toolbar: Selection, Add, Connect tools]           │
├──────────────────┬──────────────────────────────────┤
│                  │                                   │
│   WORKBENCH      │         CANVAS                   │
│   (Parts list)   │   ┌─────────────────────────┐    │
│                  │   │   2D Diagram View       │    │
│   □ Wall         │   │   [○]───[○]───[○]       │    │
│   □ Window       │   │         │               │    │
│   □ Door         │   │        [○]              │    │
│                  │   └─────────────────────────┘    │
│                  │   ┌─────────────────────────┐    │
│                  │   │   3D Scene View         │    │
│                  │   │   [3D Model Preview]    │    │
│                  │   └─────────────────────────┘    │
├──────────────────┴──────────────────────────────────┤
│  [Footer: Model options, Tags, Zoom controls]       │
└─────────────────────────────────────────────────────┘
```

---

# 🏗️ STAGE 2 — System Architecture

## The Five Major Parts

```
┌──────────────────────────────────────────────────────────────────┐
│                        DESIGN APP ARCHITECTURE                    │
└──────────────────────────────────────────────────────────────────┘

  ┌─────────────┐     ┌─────────────────┐     ┌─────────────────┐
  │   INPUT     │────▶│   PROCESSING    │────▶│    OUTPUT       │
  │             │     │                 │     │                 │
  │ • Mouse     │     │ • State Machine │     │ • 2D Diagram    │
  │ • Keyboard  │     │ • Commands      │     │ • 3D Scene      │
  │ • Drag/Drop │     │ • Diff System   │     │ • Property Panel│
  └─────────────┘     └─────────────────┘     └─────────────────┘
         │                    │                       │
         │                    ▼                       │
         │           ┌─────────────────┐              │
         │           │      DATA       │              │
         │           │                 │              │
         └──────────▶│ • Design Store  │◀─────────────┘
                     │ • Kit Store     │
                     │ • App State     │
                     └─────────────────┘
                              │
                              ▼
                     ┌─────────────────┐
                     │    EXTERNAL     │
                     │                 │
                     │ • File System   │
                     │ • Collaboration │
                     │ • 3D Models     │
                     └─────────────────┘
```

## Data Flow Story

```
User clicks "Add Piece"
        │
        ▼
┌───────────────────┐
│ Event captured by │
│ React component   │
└───────────────────┘
        │
        ▼
┌───────────────────┐
│ Command executed: │
│ "addPiece"        │
└───────────────────┘
        │
        ▼
┌───────────────────┐
│ Store updated     │
│ (state changes)   │
└───────────────────┘
        │
        ▼
┌───────────────────┐
│ React re-renders  │
│ affected views    │
└───────────────────┘
        │
        ├────────────┬────────────┐
        ▼            ▼            ▼
   ┌────────┐   ┌────────┐   ┌────────┐
   │Diagram │   │ Scene  │   │Details │
   │updates │   │updates │   │ Panel  │
   └────────┘   └────────┘   └────────┘
```

---

# 📖 STAGE 3 — Logic Flow

## The Program as a Story

### Chapter 1: The Application Wakes Up

```
SCENE 1: Initialization
━━━━━━━━━━━━━━━━━━━━━━━
When the app loads:
1. Create an empty "DesignStore" (a container for all design data)
2. Register the Design App as a "plugin" in the larger Sketchpad system
3. Set up default settings (no selection, no hover, normal tool active)
4. Wait for user interaction

SCENE 2: Loading a Design
━━━━━━━━━━━━━━━━━━━━━━━━━
When a design is opened:
1. Fetch the design data from storage
2. Build a list of all pieces (nodes in our diagram)
3. Build a list of all connections (edges in our diagram)
4. Calculate where each piece should appear on screen
5. Render everything
```

### Chapter 2: User Interacts

```
SCENE 3: Selecting a Piece
━━━━━━━━━━━━━━━━━━━━━━━━━━
User clicks a piece:
1. Detect which piece was clicked
2. Clear previous selection
3. Add this piece to selection
4. Update the piece's appearance (highlight it)
5. Show piece details in the side panel

SCENE 4: Creating a Connection
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
User connects two pieces:
1. User clicks a connector (port) on piece A
2. Connector is highlighted, waiting for second selection
3. User clicks a connector on piece B
4. System creates a new Connection object
5. Diagram shows a line between the pieces
6. 3D scene updates to show physical connection
```

### Chapter 3: Making Changes

```
SCENE 5: Moving a Piece
━━━━━━━━━━━━━━━━━━━━━━━
User drags a piece:
1. Start drag: Record starting position
2. During drag: Update piece position in real-time
3. Show alignment guides (helper lines)
4. End drag: Finalize new position
5. Update all connected pieces' connection points

SCENE 6: Undo/Redo
━━━━━━━━━━━━━━━━━━
User presses Ctrl+Z:
1. Look at the "transaction stack" (history of changes)
2. Find the last change (a "diff" - what changed)
3. Calculate the "inverse diff" (how to reverse it)
4. Apply the inverse diff
5. Move the change to the "redo stack"
```

## Flowchart: Main User Actions

```
                         ┌─────────────────┐
                         │   USER ACTION   │
                         └────────┬────────┘
                                  │
           ┌──────────────────────┼──────────────────────┐
           │                      │                      │
           ▼                      ▼                      ▼
    ┌─────────────┐        ┌─────────────┐        ┌─────────────┐
    │   SELECT    │        │   CREATE    │        │   MODIFY    │
    └──────┬──────┘        └──────┬──────┘        └──────┬──────┘
           │                      │                      │
    ┌──────┴──────┐        ┌──────┴──────┐        ┌──────┴──────┐
    │ Click piece │        │ Add piece   │        │ Drag piece  │
    │ Click conn. │        │ Add conn.   │        │ Edit props  │
    │ Lasso select│        │ From library│        │ Delete      │
    └──────┬──────┘        └──────┬──────┘        └──────┬──────┘
           │                      │                      │
           └──────────────────────┼──────────────────────┘
                                  │
                                  ▼
                         ┌─────────────────┐
                         │ EXECUTE COMMAND │
                         └────────┬────────┘
                                  │
                                  ▼
                         ┌─────────────────┐
                         │  UPDATE STORE   │
                         │  (with diff)    │
                         └────────┬────────┘
                                  │
                                  ▼
                         ┌─────────────────┐
                         │  RE-RENDER UI   │
                         └─────────────────┘
```

---

# 📚 STAGE 4 — Core Concepts (Deep Dive)

> **Note**: This section teaches each concept thoroughly. We start with the general concept, explain why it exists, show how it works in simple examples, and THEN show how Design.tsx uses it.

---

## Concept 1: Variables & State (The Memory of Your App)

### The Simple Version

A **variable** is a labeled box that holds data. You learned this in Chapter 0.

**State** is a special kind of variable that React watches. When state changes, React automatically updates the screen.

```
Regular Variable:                    State Variable:
┌──────────────┐                    ┌──────────────┐
│    count     │                    │    count     │  ←── React is watching!
├──────────────┤                    ├──────────────┤
│      5       │                    │      5       │
└──────────────┘                    └──────────────┘
       │                                   │
       ▼                                   ▼
  Changes?                            Changes?
  Nothing happens.                    React re-renders!
```

### Why Is State Different From Regular Variables?

Regular JavaScript variables don't trigger screen updates:

```javascript
// ❌ This won't work in React
let count = 0;

function handleClick() {
  count = count + 1; // Variable changes...
  // ...but the screen doesn't update!
}
```

State variables DO trigger screen updates:

```javascript
// ✅ This works in React
const [count, setCount] = useState(0);

function handleClick() {
  setCount(count + 1); // State changes...
  // ...and React updates the screen!
}
```

### The useState Hook: Line by Line

```javascript
const [count, setCount] = useState(0);
```

Let's break this down completely:

```javascript
const [count, setCount] = useState(0);
│      │      │            │        │
│      │      │            │        └─ Initial value (0)
│      │      │            │
│      │      │            └─ The function to call (from React)
│      │      │
│      │      └─ A function to update the value
│      │         (we name it "set" + variable name)
│      │
│      └─ The current value of the state
│
└─ Can't reassign (we use setCount instead)
```

The `useState(0)` function returns an array with two items:

1. The current value
2. A function to update it

The `[count, setCount] =` part is called "destructuring" — it's a shortcut for:

```javascript
// The long way (don't do this)
const result = useState(0);
const count = result[0];
const setCount = result[1];

// The short way (do this)
const [count, setCount] = useState(0);
```

### State in Design.tsx

The Design App has LOTS of state. Here's what it tracks:

```typescript
// The app tracks what's selected, hovered, etc.
export interface DesignAppState {
  // What tool is currently active? (selection tool, lasso tool, etc.)
  activeTool?: ToolKind;

  // Which pieces and connections are selected?
  selection?: DesignAppSelection;

  // What is the mouse hovering over?
  hover?: DesignAppHover;

  // Where is the 3D camera looking?
  camera?: Camera;

  // Where is the center of the 2D diagram view?
  diagramCenter?: Coord;

  // How zoomed in is the diagram?
  diagramScale?: number;

  // Which panels are visible?
  panelVisibility: PanelVisibility;

  // ...and more
}
```

**Line-by-Line Explanation:**

| Line                             | What It Means                | Real-World Analogy                          |
| -------------------------------- | ---------------------------- | ------------------------------------------- |
| `activeTool?: ToolKind`          | Which tool the user is using | Like which pen you're holding               |
| `selection?: DesignAppSelection` | What's selected              | What you've highlighted with a marker       |
| `hover?: DesignAppHover`         | What the mouse is over       | What your finger is pointing at             |
| `camera?: Camera`                | 3D viewpoint                 | Where you're standing to look at a building |
| `diagramScale?: number`          | Zoom level                   | How close you've moved to the paper         |

The `?` after each property name means it's **optional** — it might not have a value yet.

### Practice Exercise

Create a simple component that counts clicks:

```tsx
// Exercise: Complete this component
function ClickCounter() {
  // 1. Create state for the count
  // Hint: const [count, setCount] = ???

  // 2. Create a function to increment the count
  // Hint: function handleClick() { ??? }

  // 3. Return a button that shows the count and calls handleClick
  return (
    <button onClick={???}>
      Clicked {???} times
    </button>
  );
}
```

<details>
<summary>Click to see solution</summary>

```tsx
function ClickCounter() {
  const [count, setCount] = useState(0);

  function handleClick() {
    setCount(count + 1);
  }

  return <button onClick={handleClick}>Clicked {count} times</button>;
}
```

</details>

---

## Concept 2: Functions (Reusable Instruction Sets)

### The Simple Version

A **function** is a named set of instructions you can run whenever you want. You've already learned the basics in Chapter 0.

### Why Functions Are Essential

Without functions, you'd have to copy-paste code everywhere:

```javascript
// ❌ Without functions: Copy-paste nightmare
console.log("Hello, Alice!");
console.log("Welcome to the app, Alice.");
console.log("Would you like help, Alice?");

console.log("Hello, Bob!");
console.log("Welcome to the app, Bob.");
console.log("Would you like help, Bob?");

// ...repeat for every user...

// ✅ With functions: Write once, use everywhere
function welcomeUser(name) {
  console.log("Hello, " + name + "!");
  console.log("Welcome to the app, " + name + ".");
  console.log("Would you like help, " + name + "?");
}

welcomeUser("Alice");
welcomeUser("Bob");
welcomeUser("Charlie");
```

### The Anatomy of a Function

```javascript
function calculateTotal(price, taxRate) {
    const tax = price * taxRate;
    const total = price + tax;
    return total;
}
│        │              │        │
│        │              │        └── RETURN: What comes out
│        │              │
│        │              └── BODY: The instructions
│        │
│        └── PARAMETERS: What goes in (inputs)
│
└── NAME: What you call it
```

### Arrow Functions: The Modern Way

In modern JavaScript (and TypeScript), you'll often see **arrow functions**:

```javascript
// Traditional function
function add(a, b) {
  return a + b;
}

// Arrow function (exactly the same behavior)
const add = (a, b) => {
  return a + b;
};

// Arrow function shorthand (for simple one-liners)
const add = (a, b) => a + b;
```

**When you see `=>`, think "arrow function."**

### Functions in Design.tsx: Commands

The Design App uses functions everywhere, but the most important ones are **commands**. A command is a function that describes how to change the app state.

```typescript
// This is a command function
"semio.designApp.selectPiece": (context, guid) => {
  return {
    diff: {
      selection: {
        pieces: {
          removed: context.designApp.selection?.pieces || [],
          added: [guid],
        },
      },
    },
  };
}
```

Let's decode this completely:

```typescript
"semio.designApp.selectPiece"    // The command's name (like a function name)
: (context, guid) =>              // Arrow function with two parameters
{                                 // Function body starts
  return {                        // Return an object describing what changed
    diff: {                       // The "diff" is what's different now
      selection: {                // We're changing the selection
        pieces: {                 // Specifically, which pieces are selected
          removed: context.designApp.selection?.pieces || [],
          //       ↑ Remove currently selected pieces (to clear selection)
          added: [guid],
          //      ↑ Add the clicked piece to selection
        },
      },
    },
  };
}
```

**Plain English Translation:**
"When the user clicks a piece, clear whatever was selected before, then select only the clicked piece."

### Understanding the `?.` Operator (Optional Chaining)

You'll see `?.` a lot in this code:

```typescript
context.designApp.selection?.pieces || [];
```

This means: "Try to get `pieces` from `selection`. If `selection` doesn't exist, don't crash — just give me `undefined`."

```javascript
// Without ?.  (dangerous)
context.designApp.selection.pieces; // CRASH if selection is undefined!

// With ?.  (safe)
context.designApp.selection?.pieces; // Returns undefined if selection is missing

// With fallback  (safest)
context.designApp.selection?.pieces || []; // Returns [] if anything is missing
```

### Practice Exercise

```typescript
// Exercise: Write a command function that adds a piece to selection
// (without removing existing selection)

"semio.designApp.addPieceToSelection": (context, guid) => {
  // Your code here
  // Hint: You need to keep existing pieces AND add the new one
}
```

<details>
<summary>Click to see solution</summary>

```typescript
"semio.designApp.addPieceToSelection": (context, guid) => {
  const currentPieces = context.designApp.selection?.pieces || [];

  return {
    diff: {
      selection: {
        pieces: {
          added: [guid],  // Just add, don't remove anything
        },
      },
    },
  };
}
```

</details>

---

## Concept 3: React Components (Building Blocks of UI)

### The Simple Version

A **React component** is a piece of user interface. Like LEGO blocks, you combine small components to build bigger ones.

### What Is React?

React is a JavaScript library for building user interfaces. Instead of writing HTML directly, you write JavaScript that _describes_ what the HTML should look like.

### Why Components?

Before React, web developers mixed HTML, CSS, and JavaScript together. This got messy fast.

React said: **"What if we could build UI the same way we build with functions?"**

```javascript
// ❌ Old way: HTML scattered everywhere
document.getElementById("button").innerHTML = "Click me";
document.getElementById("button").addEventListener("click", handleClick);
// ...100 more lines of connecting things...

// ✅ React way: Self-contained components
function Button() {
  return <button onClick={handleClick}>Click me</button>;
}
```

### JSX: HTML Inside JavaScript

That weird `<button>` syntax inside JavaScript is called **JSX**. It looks like HTML but it's actually JavaScript.

```jsx
// This JSX:
const element = <h1>Hello, world!</h1>;

// Gets converted to this JavaScript:
const element = React.createElement("h1", null, "Hello, world!");
```

You don't need to understand the conversion — just know that JSX lets you write HTML-like code in JavaScript.

### Anatomy of a React Component

```jsx
function Greeting({ name }) {
  // Component name (capitalized) + props
  return (
    // Must return JSX
    <div>
      {" "}
      // JSX starts here
      <h1>Hello, {name}!</h1> // {} to insert JavaScript values
      <p>Welcome to our app.</p>
    </div> // JSX ends here
  );
}

// Using the component:
<Greeting name="Alice" />;
```

**Key Rules:**

1. Component names start with a **C**apital letter
2. Components must return JSX (or `null`)
3. Use `{}` to insert JavaScript expressions in JSX

### Props: Passing Data to Components

**Props** (short for "properties") are how you pass data to a component.

```jsx
// Parent component passes props
<Button text="Click me" color="blue" size="large" />;

// Child component receives props
function Button({ text, color, size }) {
  return <button style={{ backgroundColor: color, fontSize: size }}>{text}</button>;
}
```

**The `{ text, color, size }` syntax is called destructuring** — it unpacks values from an object:

```javascript
// Without destructuring:
function Button(props) {
  console.log(props.text); // "Click me"
  console.log(props.color); // "blue"
}

// With destructuring (cleaner):
function Button({ text, color }) {
  console.log(text); // "Click me"
  console.log(color); // "blue"
}
```

### Children: Components Inside Components

The `children` prop contains whatever you put _between_ a component's tags:

```jsx
function Card({ children }) {
  return <div className="card">{children}</div>;
}

// Usage:
<Card>
  <h1>Title</h1> // These become
  <p>Some content</p> // the "children"
</Card>;
```

### Components in Design.tsx: PieceNodeComponent

Here's a real component from Design.tsx — the `PieceNodeComponent` that displays a single piece in the diagram:

```tsx
const PieceNodeComponent: React.FC<NodeProps<PieceNode>> = ({ id, data }) => {
  const { piece, type } = data;
  const isSelected = useIsPieceSelected(id);
  const isHovered = useIsPieceHovered(id);

  return (
    <div className={cn("piece-node", isSelected && "selected", isHovered && "hovered")}>
      <Avatar name={type.name} icon={type?.icon} />
    </div>
  );
};
```

Let me break this down completely:

| Line                                                       | What It Does                                                  |
| ---------------------------------------------------------- | ------------------------------------------------------------- |
| `const PieceNodeComponent: React.FC<NodeProps<PieceNode>>` | Create a component that accepts NodeProps with PieceNode data |
| `= ({ id, data }) =>`                                      | Arrow function that destructures `id` and `data` from props   |
| `const { piece, type } = data;`                            | Destructure the piece and type from data                      |
| `const isSelected = useIsPieceSelected(id);`               | Call a hook to check if this piece is selected                |
| `const isHovered = useIsPieceHovered(id);`                 | Call a hook to check if mouse is over this piece              |
| `className={cn(...)}`                                      | Apply CSS classes conditionally using cn() helper             |
| `isSelected && "selected"`                                 | Add "selected" class only if piece is selected                |
| `<Avatar name={...} icon={...} />`                         | Render an Avatar component with the type's name and icon      |

### The `cn()` Helper Function

You'll see `cn()` everywhere in this codebase:

```typescript
// cn() combines CSS class names conditionally
cn("base-class", isActive && "active", isDisabled && "disabled");

// If isActive is true and isDisabled is false:
// Result: "base-class active"

// If both are false:
// Result: "base-class"
```

### Practice Exercise

```jsx
// Exercise: Create a ConnectionLine component that displays a line
// between two pieces. It should:
// 1. Take "from" and "to" props (piece IDs)
// 2. Show a "selected" class if either piece is selected

function ConnectionLine({ from, to }) {
  // Your code here
}
```

<details>
<summary>Click to see solution</summary>

```jsx
function ConnectionLine({ from, to }) {
  const isFromSelected = useIsPieceSelected(from);
  const isToSelected = useIsPieceSelected(to);
  const isEitherSelected = isFromSelected || isToSelected;

  return (
    <div className={cn("connection-line", isEitherSelected && "selected")}>
      <span>From: {from}</span>
      <span>To: {to}</span>
    </div>
  );
}
```

</details>

---

## Concept 4: Hooks (React's Superpowers)

### What Is a Hook?

A **hook** is a special function that lets React components do more than just display things. Hooks let components:

- Remember values between renders (**state**)
- Talk to the outside world (**effects**)
- Share logic without copying code (**custom hooks**)

**The Rule:** Hook names always start with `use` — like `useState`, `useEffect`, `useMemo`.

### Why Do Hooks Exist?

Before hooks existed (2018), React components had two forms:

```jsx
// ❌ OLD WAY: Class components (complicated)
class Counter extends React.Component {
  constructor(props) {
    super(props);
    this.state = { count: 0 };
    this.handleClick = this.handleClick.bind(this);
  }

  handleClick() {
    this.setState({ count: this.state.count + 1 });
  }

  render() {
    return <button onClick={this.handleClick}>{this.state.count}</button>;
  }
}

// ✅ NEW WAY: Function components with hooks (simple!)
function Counter() {
  const [count, setCount] = useState(0);
  return <button onClick={() => setCount(count + 1)}>{count}</button>;
}
```

Same functionality, way less code. That's why hooks won.

### The Essential Hooks

#### 1. useState — Remembering Values

`useState` creates a piece of state that survives re-renders.

```tsx
const [value, setValue] = useState(initialValue);
//     │       │                    │
//     │       │                    └── Starting value
//     │       └── Function to update the value
//     └── Current value

// Example
const [count, setCount] = useState(0);
setCount(5); // Now count is 5
setCount((c) => c + 1); // Increment based on current value
```

#### 2. useEffect — Doing Things After Render

`useEffect` runs code after the component renders. Great for:

- Fetching data
- Setting up subscriptions
- Updating the document title

```tsx
useEffect(() => {
  // This code runs after every render
  document.title = `You clicked ${count} times`;
});

// With cleanup
useEffect(() => {
  const timer = setInterval(() => console.log("tick"), 1000);

  return () => clearInterval(timer); // Cleanup when component unmounts
}, []); // Empty array = run only once

// With dependencies
useEffect(() => {
  console.log(`Count changed to ${count}`);
}, [count]); // Run whenever count changes
```

**The dependency array `[]` controls WHEN the effect runs:**

- No array: Run after every render
- Empty `[]`: Run only once (on mount)
- With values `[count, name]`: Run when those values change

#### 3. useMemo — Expensive Calculations

`useMemo` caches the result of an expensive calculation:

```tsx
// ❌ Without useMemo: recalculates on EVERY render
const sortedItems = items.sort((a, b) => a.name.localeCompare(b.name));

// ✅ With useMemo: recalculates only when items change
const sortedItems = useMemo(() => {
  return items.sort((a, b) => a.name.localeCompare(b.name));
}, [items]);
```

#### 4. useCallback — Stable Function References

`useCallback` keeps the same function reference between renders:

```tsx
// ❌ Without useCallback: new function every render
const handleClick = () => {
  console.log(count);
};

// ✅ With useCallback: same function unless count changes
const handleClick = useCallback(() => {
  console.log(count);
}, [count]);
```

This matters when passing functions to child components that check for changes.

#### 5. useContext — Sharing Data Deeply

`useContext` grabs data from a "context" without passing props:

```tsx
// Create a context
const ThemeContext = React.createContext("light");

// Provide it at the top
function App() {
  return (
    <ThemeContext.Provider value="dark">
      <DeepComponent />
    </ThemeContext.Provider>
  );
}

// Use it anywhere below
function DeepComponent() {
  const theme = useContext(ThemeContext); // "dark"
  return <div className={theme}>...</div>;
}
```

### Custom Hooks: Packaging Logic

You can create your own hooks by combining built-in hooks:

```tsx
// Custom hook for mouse position
function useMousePosition() {
  const [position, setPosition] = useState({ x: 0, y: 0 });

  useEffect(() => {
    const handleMove = (e) => setPosition({ x: e.clientX, y: e.clientY });
    window.addEventListener("mousemove", handleMove);
    return () => window.removeEventListener("mousemove", handleMove);
  }, []);

  return position;
}

// Use it in any component
function Cursor() {
  const { x, y } = useMousePosition();
  return <div style={{ left: x, top: y }}>•</div>;
}
```

### Hooks in Design.tsx

Design.tsx has many custom hooks. Here's how one works:

```tsx
export function useDesignAppSelection(): HookResult<DesignAppSelection> {
  return fieldToHookResult(useDesignAppSelectionField());
}
```

This is a **triadic hook** — it returns three values:

```tsx
const [value, setValue, canSet] = useDesignAppSelection();
//     │       │         │
//     │       │         └── Boolean: Is setting allowed right now?
//     │       └── Function to update (or undefined if not allowed)
//     └── Current selection value
```

**Usage in a component:**

```tsx
function SelectionInfo() {
  const [selection, setSelection, canSet] = useDesignAppSelection();

  if (!selection.pieces.length) {
    return <p>Nothing selected</p>;
  }

  return (
    <div>
      <p>Selected: {selection.pieces.length} pieces</p>
      {canSet && <button onClick={() => setSelection({ pieces: [], connections: [] })}>Clear Selection</button>}
    </div>
  );
}
```

### The Rules of Hooks

React has strict rules about hooks:

1. **Only call hooks at the top level** — not inside loops, conditions, or nested functions
2. **Only call hooks from React functions** — components or other hooks

```tsx
// ❌ WRONG: Hook inside a condition
function Component() {
  if (someCondition) {
    const [state, setState] = useState(); // BREAKS!
  }
}

// ✅ RIGHT: Hook at top, condition inside
function Component() {
  const [state, setState] = useState();

  if (someCondition) {
    // use state here
  }
}
```

### Practice Exercise

```tsx
// Exercise: Create a custom hook called useCounter that:
// 1. Tracks a count starting at 0
// 2. Returns the count and functions to increment/decrement

function useCounter() {
  // Your code here
  // Should return: { count, increment, decrement }
}
```

<details>
<summary>Click to see solution</summary>

```tsx
function useCounter(initialValue = 0) {
  const [count, setCount] = useState(initialValue);

  const increment = useCallback(() => {
    setCount((c) => c + 1);
  }, []);

  const decrement = useCallback(() => {
    setCount((c) => c - 1);
  }, []);

  return { count, increment, decrement };
}

// Usage:
function Counter() {
  const { count, increment, decrement } = useCounter(10);

  return (
    <div>
      <button onClick={decrement}>-</button>
      <span>{count}</span>
      <button onClick={increment}>+</button>
    </div>
  );
}
```

</details>

---

## Concept 5: The Store Pattern (Centralized State)

### What Is a Store?

Imagine you're building a house with a team. Instead of everyone keeping their own notes about what materials are available, you have **one central warehouse** that everyone checks and updates.

A **store** is that warehouse for your app's data.

### The Problem It Solves

Without a store, you'd pass data through every component:

```jsx
// ❌ Prop drilling nightmare
function App() {
  const [selection, setSelection] = useState([]);
  return <Layout selection={selection} setSelection={setSelection} />;
}

function Layout({ selection, setSelection }) {
  return <Sidebar selection={selection} setSelection={setSelection} />;
}

function Sidebar({ selection, setSelection }) {
  return <PieceList selection={selection} setSelection={setSelection} />;
}

function PieceList({ selection, setSelection }) {
  // Finally! We can use the data here
}
```

With a store:

```jsx
// ✅ Central store
function PieceList() {
  const [selection, setSelection] = useSelection(); // Get from store directly
}
```

### How Stores Work

```
                    ┌─────────────────────────┐
                    │         STORE           │
                    │  ┌───────────────────┐  │
                    │  │  pieces: [...]    │  │
                    │  │  connections: []  │  │
                    │  │  selection: {...} │  │
                    │  │  hover: {...}     │  │
                    │  └───────────────────┘  │
                    └───────────┬─────────────┘
                                │
          ┌─────────────────────┼─────────────────────┐
          │                     │                     │
          ▼                     ▼                     ▼
    ┌───────────┐         ┌───────────┐         ┌───────────┐
    │  Diagram  │         │  Details  │         │   Scene   │
    │ Component │         │   Panel   │         │ Component │
    └───────────┘         └───────────┘         └───────────┘

    All components read from the same store.
    When the store changes, all components update automatically.
```

### Store Operations

A store has three core operations:

1. **Read**: Get current state
2. **Write**: Update state
3. **Subscribe**: Get notified when state changes

```typescript
// Simplified store interface
interface Store<T> {
  getState(): T; // Read
  setState(newState: T): void; // Write
  subscribe(callback: () => void); // Subscribe
}
```

### The DesignStore in Design.tsx

Here's how the actual DesignStore works:

```typescript
export class DesignStore extends PlainKitDiffAppStore {
  // The Y.js map that holds all data (Y.js enables real-time collaboration)
  yMap: Y.Map<any>;

  // Read: Build current state from Y.js
  buildSnapshot(): DesignAppState {
    return {
      selection: this.getSelection(),
      hover: this.getHover(),
      camera: this.getCamera(),
      activeTool: this.getActiveTool(),
      panelVisibility: this.getPanelVisibility(),
      // ... more state
    };
  }

  // Write: Update Y.js (which triggers re-renders)
  setSelection(selection: DesignAppSelection) {
    this.yMap.set("selection", selection);
  }
}
```

**Line-by-line explanation:**

| Line                           | Purpose                                                     |
| ------------------------------ | ----------------------------------------------------------- |
| `extends PlainKitDiffAppStore` | Inherits base store functionality (transactions, undo/redo) |
| `yMap: Y.Map<any>`             | Uses Y.js for collaborative real-time sync                  |
| `buildSnapshot()`              | Creates a complete picture of current state                 |
| `getSelection()`               | Helper to read selection from yMap                          |
| `setSelection()`               | Writes new selection to yMap                                |

### Connecting Components to the Store

Components connect to the store using hooks:

```tsx
// Inside a component
function DiagramCanvas() {
  // These hooks read from the DesignStore
  const [selection] = useDesignAppSelection();
  const [hover] = useDesignAppHover();
  const pieces = usePieces();

  return (
    <div>
      {pieces.map((piece) => (
        <PieceNode key={piece.id} piece={piece} isSelected={selection.pieces.includes(piece.id)} isHovered={hover?.pieceId === piece.id} />
      ))}
    </div>
  );
}
```

### Why Y.js?

The store uses **Y.js** (a library for real-time collaboration). When you edit a design:

1. Your changes go to Y.js
2. Y.js syncs with other users
3. Their screens update automatically

```
   User A's Computer         Server          User B's Computer
         │                     │                     │
   edit(piece)                 │                     │
         │────────────────────▶│                     │
         │                     │────────────────────▶│
         │                     │              screen updates
         │                     │                     │
```

### Practice Exercise

```typescript
// Exercise: Create a simple counter store

const counterStore = {
  count: 0,
  subscribers: [],

  // Implement these methods:
  getCount() {
    /* return current count */
  },
  increment() {
    /* add 1 and notify subscribers */
  },
  decrement() {
    /* subtract 1 and notify subscribers */
  },
  subscribe(callback) {
    /* add callback to subscribers */
  },
  notifySubscribers() {
    /* call all subscriber callbacks */
  },
};
```

<details>
<summary>Click to see solution</summary>

```typescript
const counterStore = {
  count: 0,
  subscribers: [],

  getCount() {
    return this.count;
  },

  increment() {
    this.count += 1;
    this.notifySubscribers();
  },

  decrement() {
    this.count -= 1;
    this.notifySubscribers();
  },

  subscribe(callback) {
    this.subscribers.push(callback);
    // Return unsubscribe function
    return () => {
      this.subscribers = this.subscribers.filter((cb) => cb !== callback);
    };
  },

  notifySubscribers() {
    this.subscribers.forEach((callback) => callback());
  },
};

// Usage:
counterStore.subscribe(() => console.log("Count:", counterStore.getCount()));
counterStore.increment(); // Logs: "Count: 1"
counterStore.increment(); // Logs: "Count: 2"
```

</details>

---

## Concept 6: The Command Pattern (Actions as Objects)

### What Is the Command Pattern?

Instead of directly changing data, you create a **command object** that describes the change. Then a central system executes the command.

Think of it like ordering food at a restaurant:

- **Direct approach**: You walk into the kitchen and cook
- **Command pattern**: You give your order to a waiter who handles everything

### Why Use Commands?

Commands unlock powerful features:

| Feature        | How It Works                                              |
| -------------- | --------------------------------------------------------- |
| **Undo/Redo**  | Store commands in a list. Undo = reverse the last command |
| **Logging**    | Record every command for debugging or analytics           |
| **Validation** | Check if a command is allowed before executing            |
| **Batching**   | Group multiple commands into one transaction              |
| **Replay**     | Recreate state by replaying all commands                  |

### How Commands Work

```
    User clicks      Command created       Executor runs       State updates
    "Delete"         & dispatched          the command
        │                 │                    │                   │
        ▼                 ▼                    ▼                   ▼
    ┌────────┐      ┌───────────────┐    ┌───────────┐      ┌──────────┐
    │ Button │ ──▶  │ { type:       │ ─▶ │ Execute   │ ──▶  │ pieces:  │
    │ Click  │      │   "delete",   │    │ command   │      │ [A, C]   │
    └────────┘      │   id: "B" }   │    └───────────┘      └──────────┘
                    └───────────────┘
```

### Commands in Design.tsx

Commands are defined in a registry:

```typescript
export const designAppCommands = {
  // Command name → Command function
  "semio.designApp.selectPiece": (context, guid) => {
    return {
      diff: {
        selection: {
          pieces: {
            removed: context.designApp.selection?.pieces || [],
            added: [guid],
          },
        },
      },
    };
  },

  "semio.designApp.deleteSelected": (context) => {
    const selectedPieces = context.designApp.selection?.pieces || [];
    const selectedConnections = context.designApp.selection?.connections || [];

    return {
      // UI state changes
      diff: {
        selection: { pieces: { removed: selectedPieces } },
      },
      // Data changes (affects the actual design)
      kitDiff: {
        designs: {
          updated: [
            {
              guid: context.designGuid,
              diff: {
                pieces: { removed: selectedPieces },
                connections: { removed: selectedConnections },
              },
            },
          ],
        },
      },
    };
  },
};
```

**Key insight:** Commands return TWO kinds of diffs:

- `diff` → UI state changes (selection, hover, camera)
- `kitDiff` → Data changes (pieces, connections, types)

### Executing Commands

When you want to run a command:

```typescript
// From a component
function DeleteButton() {
  const { executeCommand } = useDesignAppCommands();

  return (
    <button
      onClick={() => {
        executeCommand(
          "semio.designApp.deleteSelected", // Command name
          "semio.sketchpad.toolbar.delete" // Origin (for logging/debugging)
        );
      }}
    >
      Delete Selected
    </button>
  );
}
```

The executor:

1. Looks up the command in the registry
2. Builds the context (current state)
3. Calls the command function
4. Applies the returned diffs
5. Stores the edit for undo/redo

### The Origin Parameter

Every command call includes an **origin** — the ID of the UI element that triggered it:

```typescript
executeCommand(
  "semio.designApp.selectPiece", // What to do
  "semio.sketchpad.diagram.pieceNode.123", // Where it came from
  pieceGuid, // Additional arguments
);
```

This helps with:

- **Debugging**: Know which button triggered which action
- **Analytics**: Track which features users use
- **Tutorials**: Highlight the button that was clicked

### Practice Exercise

```typescript
// Exercise: Write a command that adds a piece to the selection
// (without removing existing selection)

"semio.designApp.addToSelection": (context, pieceGuid) => {
  // Your code here
  // Hint: You want to ADD to existing selection, not replace
}
```

<details>
<summary>Click to see solution</summary>

```typescript
"semio.designApp.addToSelection": (context, pieceGuid) => {
  // Just add the new piece, don't remove anything
  return {
    diff: {
      selection: {
        pieces: {
          // removed: [],  // Not needed - default is no removal
          added: [pieceGuid],
        },
      },
    },
  };
}
```

</details>

---

## Concept 7: Diffs (Describing Changes)

### What Is a Diff?

A **diff** (short for "difference") describes exactly what changed between two states. Instead of saying "here's the new state," you say "here's what's different."

Think of it like a recipe change:

- **Full state approach**: "Here's the entire recipe again with one ingredient changed"
- **Diff approach**: "Change the sugar from 1 cup to 2 cups"

### Why Use Diffs?

| Benefit           | Explanation                           |
| ----------------- | ------------------------------------- |
| **Smaller**       | Store only changes, not entire copies |
| **Reversible**    | Swap "added" and "removed" to undo    |
| **Mergeable**     | Combine multiple diffs into one       |
| **Collaborative** | Send tiny diffs instead of full state |

### Diff Structure

```
BEFORE STATE:                    AFTER STATE:
┌─────────────────────┐         ┌─────────────────────┐
│ pieces: [A, B, C]   │   ──▶   │ pieces: [A, C, D]   │
│ selection: [A]      │         │ selection: [C, D]   │
└─────────────────────┘         └─────────────────────┘

DIFF (what changed):
┌─────────────────────────────────────────┐
│ pieces: { removed: [B], added: [D] }    │
│ selection: { removed: [A], added: [C, D] } │
└─────────────────────────────────────────┘
```

### The Diff Structure in Design.tsx

```typescript
export interface DesignAppSelectionDiff {
  pieces?: {
    added?: Guid[]; // New pieces to select
    removed?: Guid[]; // Pieces to unselect
  };
  connections?: {
    added?: Guid[];
    removed?: Guid[];
  };
  connector?: {
    added?: Guid; // Single connector (for connection tool)
    removed?: Guid;
  };
}
```

**Line-by-line explanation:**

| Line               | Purpose                                                       |
| ------------------ | ------------------------------------------------------------- |
| `pieces?:`         | Changes to piece selection (optional)                         |
| `added?: Guid[]`   | List of piece GUIDs to add to selection                       |
| `removed?: Guid[]` | List of piece GUIDs to remove from selection                  |
| `connections?:`    | Changes to connection selection                               |
| `connector?:`      | Special single-connector selection (for creating connections) |

### Applying Diffs

To apply a diff, you:

1. Remove the "removed" items
2. Add the "added" items

```typescript
function applySelectionDiff(current: DesignAppSelection, diff: DesignAppSelectionDiff): DesignAppSelection {
  return {
    pieces: [
      // Keep pieces that weren't removed
      ...current.pieces.filter((p) => !diff.pieces?.removed?.includes(p)),
      // Add new pieces
      ...(diff.pieces?.added || []),
    ],
    connections: [...current.connections.filter((c) => !diff.connections?.removed?.includes(c)), ...(diff.connections?.added || [])],
  };
}
```

### Inverse Diffs (for Undo)

To undo a diff, create its **inverse** by swapping added and removed:

```typescript
// Original diff: { pieces: { added: [D], removed: [B] } }
// Inverse diff:  { pieces: { added: [B], removed: [D] } }

function inverseDiff(diff: DesignAppSelectionDiff): DesignAppSelectionDiff {
  return {
    pieces: {
      added: diff.pieces?.removed, // What was removed, now add back
      removed: diff.pieces?.added, // What was added, now remove
    },
    connections: {
      added: diff.connections?.removed,
      removed: diff.connections?.added,
    },
  };
}
```

### How Undo/Redo Works

```
ACTION FLOW:
    ┌────────┐     ┌────────┐     ┌────────┐     ┌────────┐
    │ State  │ ──▶ │ State  │ ──▶ │ State  │ ──▶ │ State  │
    │   0    │     │   1    │     │   2    │     │   3    │
    └────────┘     └────────┘     └────────┘     └────────┘
         │              │              │
         ▼              ▼              ▼
    ┌────────┐     ┌────────┐     ┌────────┐
    │ Diff A │     │ Diff B │     │ Diff C │
    └────────┘     └────────┘     └────────┘

UNDO (reverse Diff C):
    Apply inverse of Diff C → Back to State 2

REDO (reapply Diff C):
    Apply Diff C again → Back to State 3
```

### Practice Exercise

```typescript
// Exercise: Given this state and diff, what's the result?

const currentSelection = {
  pieces: ["piece-1", "piece-2", "piece-3"],
  connections: ["conn-1"],
};

const diff = {
  pieces: { added: ["piece-4"], removed: ["piece-2"] },
  connections: { removed: ["conn-1"] },
};

// What is the new selection after applying this diff?
```

<details>
<summary>Click to see solution</summary>

```typescript
const newSelection = {
  pieces: ["piece-1", "piece-3", "piece-4"],
  // piece-2 was removed, piece-4 was added
  connections: [],
  // conn-1 was removed
};
```

</details>

---

## Concept 8: React Context (Deep Data Sharing)

### What Is Context?

**Context** is React's way of passing data through many component layers without prop drilling.

Imagine your component tree is a building:

- **Props**: Passing a package by hand from floor to floor
- **Context**: An elevator that delivers packages directly to any floor

### The Problem Context Solves

```jsx
// ❌ WITHOUT CONTEXT: Prop drilling nightmare
function App() {
  const [theme, setTheme] = useState("dark");
  return <Layout theme={theme} />;
}

function Layout({ theme }) {
  return <Sidebar theme={theme} />;
}

function Sidebar({ theme }) {
  return <Button theme={theme} />;
}

function Button({ theme }) {
  return <button className={theme}>Click</button>;
}
```

Every component has to pass `theme` even if it doesn't use it.

```jsx
// ✅ WITH CONTEXT: Direct access anywhere
const ThemeContext = createContext("light");

function App() {
  const [theme, setTheme] = useState("dark");
  return (
    <ThemeContext.Provider value={theme}>
      <Layout />
    </ThemeContext.Provider>
  );
}

function Layout() {
  return <Sidebar />; // No theme prop needed!
}

function Sidebar() {
  return <Button />; // No theme prop needed!
}

function Button() {
  const theme = useContext(ThemeContext); // Gets "dark" directly
  return <button className={theme}>Click</button>;
}
```

### The Three Parts of Context

```jsx
// 1. CREATE: Define the context
const ThemeContext = createContext("light"); // "light" is the default

// 2. PROVIDE: Make it available to children
<ThemeContext.Provider value="dark">
  <ChildComponent />
</ThemeContext.Provider>;

// 3. CONSUME: Use it in any child component
function ChildComponent() {
  const theme = useContext(ThemeContext); // "dark"
}
```

### Context in Design.tsx

Design.tsx uses several contexts. Here's the main one:

```tsx
// Context for design app scope (which design we're editing)
const DesignAppScopeContext = createContext<{
  kitGuid: Guid;
  designGuid: Guid;
} | null>(null);

// Provider wraps the entire design app
export function DesignAppScopeProvider({ kitGuid, designGuid, children }) {
  const value = useMemo(() => ({ kitGuid, designGuid }), [kitGuid, designGuid]);

  return <DesignAppScopeContext.Provider value={value}>{children}</DesignAppScopeContext.Provider>;
}

// Hook to access the scope
export function useDesignAppScope() {
  const scope = useContext(DesignAppScopeContext);
  if (!scope) {
    throw new Error("useDesignAppScope must be used within DesignAppScopeProvider");
  }
  return scope;
}
```

**How it's used:**

```tsx
// At the app level
<DesignAppScopeProvider kitGuid={kitId} designGuid={designId}>
  <DesignAppCanvas />
  <DesignAppToolbar />
  <DesignAppDetails />
</DesignAppScopeProvider>;

// Deep inside any component
function PieceDetails() {
  const { designGuid } = useDesignAppScope();
  const pieces = usePieces(designGuid);
  // ... render piece details
}
```

### Multiple Contexts

Real apps often have many contexts:

```tsx
function App() {
  return (
    <ThemeProvider value="dark">
      <UserProvider value={currentUser}>
        <DesignAppScopeProvider designGuid={id}>
          <TransactionProvider>
            <ActualApp />
          </TransactionProvider>
        </DesignAppScopeProvider>
      </UserProvider>
    </ThemeProvider>
  );
}
```

### Practice Exercise

```tsx
// Exercise: Create a LanguageContext that provides the current language
// 1. Create the context with "en" as default
// 2. Create a provider component
// 3. Create a useLanguage hook

// Your code here
```

<details>
<summary>Click to see solution</summary>

```tsx
// 1. Create context
const LanguageContext = createContext("en");

// 2. Provider component
function LanguageProvider({ language, children }) {
  return <LanguageContext.Provider value={language}>{children}</LanguageContext.Provider>;
}

// 3. Hook
function useLanguage() {
  return useContext(LanguageContext);
}

// Usage:
function App() {
  return (
    <LanguageProvider language="fr">
      <Greeting />
    </LanguageProvider>
  );
}

function Greeting() {
  const lang = useLanguage(); // "fr"
  return <h1>{lang === "fr" ? "Bonjour" : "Hello"}</h1>;
}
```

</details>

---

## Concept 9: TypeScript Types & Interfaces (Contracts for Data)

### What Is TypeScript?

**TypeScript** is JavaScript with types. Types describe what shape data should have.

Think of types as contracts:

- JavaScript: "This function takes something and returns something"
- TypeScript: "This function takes a string and returns a number"

### Why Use Types?

```typescript
// JavaScript: No errors until runtime 💣
function greet(name) {
  return "Hello, " + name.toUpperCase();
}
greet(42); // CRASH! (42 doesn't have toUpperCase)

// TypeScript: Error at compile time 🛡️
function greet(name: string): string {
  return "Hello, " + name.toUpperCase();
}
greet(42); // ERROR: Argument of type 'number' is not assignable
```

### Basic Type Annotations

```typescript
// Variables
let name: string = "Alice";
let age: number = 25;
let isActive: boolean = true;
let items: string[] = ["a", "b", "c"];

// Functions
function add(a: number, b: number): number {
  return a + b;
}

// Optional parameters (?)
function greet(name: string, title?: string): string {
  return title ? `${title} ${name}` : name;
}
```

### Interfaces: Shapes for Objects

An **interface** defines what properties an object must have:

```typescript
interface Person {
  name: string;
  age: number;
  email?: string; // Optional (note the ?)
}

// ✅ Valid
const alice: Person = { name: "Alice", age: 30 };
const bob: Person = { name: "Bob", age: 25, email: "bob@example.com" };

// ❌ Invalid
const charlie: Person = { name: "Charlie" }; // ERROR: missing 'age'
```

### The Guid Type

Design.tsx uses `Guid` for unique identifiers:

```typescript
type Guid = string; // Just a string, but semantically meaningful

interface Piece {
  guid: Guid; // Unique ID
  type: Guid; // References a Type's guid
  name?: string;
  // ...
}
```

### Interfaces in Design.tsx

Here's the main state interface:

```typescript
interface DesignAppState {
  panelVisibility: PanelVisibility;
  activeTool: ToolKind;
  selection: DesignAppSelection;
  hover: DesignAppHover;
  camera: Camera;
  diagramCenter: Point;
  diagramScale: number;
  focusedPieceGuid?: Guid;
  selectedModelTags: Record<Guid, string[]>;
  fullscreenWindow?: WindowKind;
  windowLayout: string;
}
```

**Breaking this down:**

| Property            | Type                     | Purpose                               |
| ------------------- | ------------------------ | ------------------------------------- |
| `panelVisibility`   | `PanelVisibility`        | Which panels are open/closed          |
| `activeTool`        | `ToolKind`               | Current tool (select, connect, etc.)  |
| `selection`         | `DesignAppSelection`     | Currently selected pieces/connections |
| `hover`             | `DesignAppHover`         | Currently hovered element             |
| `camera`            | `Camera`                 | 3D camera position and orientation    |
| `diagramCenter`     | `Point`                  | 2D diagram pan position               |
| `diagramScale`      | `number`                 | 2D diagram zoom level                 |
| `focusedPieceGuid?` | `Guid` (optional)        | Piece being edited in detail          |
| `selectedModelTags` | `Record<Guid, string[]>` | Model variant selections per type     |
| `fullscreenWindow?` | `WindowKind` (optional)  | Which window is fullscreen            |
| `windowLayout`      | `string`                 | Serialized window arrangement         |

### Generic Types

Generics are types that work with any type:

```typescript
// Without generics: separate functions for each type
function firstString(arr: string[]): string {
  return arr[0];
}
function firstNumber(arr: number[]): number {
  return arr[0];
}

// With generics: one function for all types
function first<T>(arr: T[]): T {
  return arr[0];
}

first<string>(["a", "b"]); // Returns "a" (string)
first<number>([1, 2, 3]); // Returns 1 (number)
```

### The HookResult Type

Design.tsx uses a generic `HookResult` for all hooks:

```typescript
type HookResult<T> = readonly [
  T, // The value
  ((value: T) => void) | undefined, // Setter (or undefined if not allowed)
  boolean, // Can set?
];

// Usage
const [selection, setSelection, canSet] = useDesignAppSelection();
//      ^T              ^setter        ^boolean
```

### Practice Exercise

```typescript
// Exercise: Define an interface for a Connection

// A connection should have:
// - guid: a unique identifier (Guid)
// - connectedPiece: the guid of one piece
// - connectingPiece: the guid of the other piece
// - gap: a number (optional)
// - rotation: a number (optional)

interface Connection {
  // Your code here
}
```

<details>
<summary>Click to see solution</summary>

```typescript
interface Connection {
  guid: Guid;
  connectedPiece: Guid;
  connectingPiece: Guid;
  gap?: number;
  rotation?: number;
}
```

</details>

---

```typescript
// Define what a Person looks like
interface Person {
  name: string;
  age: number;
  email?: string; // Optional (note the ?)
}

// TypeScript ensures you use it correctly
const alice: Person = {
  name: "Alice",
  age: 30,
  // email is optional, so we can skip it
};

// This would be an error:
// const bob: Person = { name: "Bob" };
// Error: missing 'age' property
```

### How This Code Uses It

```typescript
// The shape of a design's selection
export interface DesignAppSelection {
  pieces?: Guid[]; // List of selected piece IDs
  connections?: Guid[]; // List of selected connection IDs
  connector?: {
    // A specific connector being selected
    piece: Guid;
    connector: Guid;
  };
}

// The shape of the hover state
export interface DesignAppHover {
  pieces?: Guid[];
  connections?: Guid[];
  types?: Guid[];
  designs?: Guid[];
}
```

---

## Concept 10: React Flow (Node-Based Diagrams)

### What Is React Flow?

**React Flow** is a library for building interactive node-based diagrams — flowcharts, mind maps, state machines, and in our case, **design diagrams**.

Think of it like a digital whiteboard where you can:

- Place boxes (nodes)
- Connect them with lines (edges)
- Drag them around
- Zoom in and out

### Why Use React Flow?

Building a drag-and-drop diagram editor from scratch is _incredibly_ complex. You'd need to handle:

- Mouse position tracking
- Coordinate transformations (screen ↔ canvas)
- Drag gestures
- Selection boxes
- Edge path calculations
- Performance optimization for many nodes

React Flow handles ALL of this for you.

### React Flow Concepts

```
┌─────────────────────────────────────────────────────────────┐
│                     REACT FLOW CANVAS                       │
│                                                             │
│    ┌─────────┐                          ┌─────────┐         │
│    │  NODE   │───────── EDGE ──────────▶│  NODE   │         │
│    │   A     │                          │   B     │         │
│    └─────────┘                          └─────────┘         │
│         │                                    │              │
│         │                                    │              │
│         │                                    │              │
│         ▼                                    ▼              │
│    ┌─────────┐                          ┌─────────┐         │
│    │  NODE   │                          │  NODE   │         │
│    │   C     │◀─────────────────────────│   D     │         │
│    └─────────┘                          └─────────┘         │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

**Node**: A box that can contain anything — text, images, buttons, etc.
**Edge**: A line connecting two nodes
**Handle**: A connection point on a node (where edges attach)

### Basic React Flow Example

```tsx
import ReactFlow, { Node, Edge } from "reactflow";

// Define nodes
const nodes: Node[] = [
  {
    id: "node-1",
    type: "default", // Node type
    position: { x: 0, y: 0 }, // Position on canvas
    data: { label: "Start" }, // Data passed to node component
  },
  {
    id: "node-2",
    type: "default",
    position: { x: 200, y: 100 },
    data: { label: "End" },
  },
];

// Define edges (connections)
const edges: Edge[] = [
  {
    id: "edge-1",
    source: "node-1", // Connect FROM node-1
    target: "node-2", // Connect TO node-2
  },
];

// Render the flow
function MyDiagram() {
  return (
    <ReactFlow
      nodes={nodes}
      edges={edges}
      fitView // Automatically fit all nodes in view
    />
  );
}
```

### Custom Node Types

React Flow lets you create custom node components:

```tsx
// Define a custom node
function PieceNode({ data }) {
  return (
    <div className="piece-node">
      <img src={data.icon} alt={data.name} />
      <span>{data.name}</span>

      {/* Handles are connection points */}
      <Handle type="target" position="top" />
      <Handle type="source" position="bottom" />
    </div>
  );
}

// Register it
const nodeTypes = {
  piece: PieceNode,
};

// Use it
<ReactFlow nodes={nodes} edges={edges} nodeTypes={nodeTypes} />;
```

### React Flow in Design.tsx

In Design.tsx, pieces become nodes and connections become edges:

```typescript
// Convert a semio Piece to a React Flow Node
const pieceToNode = (piece: Piece, type: Type, center: Coord, index: number): PieceNode => ({
  type: "piece", // Custom node type
  id: `piece-${index}-${piece.guid}`, // Unique ID
  position: {
    x: center.u * DIAGRAM_UNIT, // Convert to pixels
    y: -center.v * DIAGRAM_UNIT, // Flip Y (screen coordinates)
  },
  data: {
    piece, // Original piece data
    type, // Type information
  },
});

// Convert a semio Connection to a React Flow Edge
const connectionToEdge = (connection: Connection, sourceNodeId: string, targetNodeId: string, index: number): ConnectionEdge => ({
  type: "SemioConnection", // Custom edge type
  id: `connection-${index}-${connection.guid}`,
  source: sourceNodeId, // Source node ID
  target: targetNodeId, // Target node ID
  sourceHandle: `connector-${connection.connected.connector}`,
  targetHandle: `connector-${connection.connecting.connector}`,
  data: { connection },
});
```

### The Diagram Component

Here's a simplified version of the diagram component:

```tsx
function DesignDiagram() {
  // Get design data
  const pieces = usePieces();
  const connections = useConnections();
  const types = useTypes();

  // Convert to React Flow format
  const nodes = useMemo(() =>
    pieces.map((piece, i) => pieceToNode(piece, types[piece.type], ...)),
    [pieces, types]
  );

  const edges = useMemo(() =>
    connections.map((conn, i) => connectionToEdge(conn, ...)),
    [connections]
  );

  // Handle interactions
  const onNodeClick = useCallback((event, node) => {
    executeCommand("semio.designApp.selectPiece", node.data.piece.guid);
  }, []);

  return (
    <ReactFlow
      nodes={nodes}
      edges={edges}
      nodeTypes={nodeTypes}
      edgeTypes={edgeTypes}
      onNodeClick={onNodeClick}
      onNodeDragStop={onNodeDragStop}
      // ... more handlers
    />
  );
}
```

### Practice Exercise

```tsx
// Exercise: Create a simple flow with 3 nodes in a triangle
// Node A at top, B at bottom-left, C at bottom-right
// Connect A → B, B → C, C → A (a cycle)

const nodes = [
  // Your code here
];

const edges = [
  // Your code here
];
```

<details>
<summary>Click to see solution</summary>

```tsx
const nodes = [
  { id: "A", position: { x: 100, y: 0 }, data: { label: "A" } },
  { id: "B", position: { x: 0, y: 100 }, data: { label: "B" } },
  { id: "C", position: { x: 200, y: 100 }, data: { label: "C" } },
];

const edges = [
  { id: "A-B", source: "A", target: "B" },
  { id: "B-C", source: "B", target: "C" },
  { id: "C-A", source: "C", target: "A" },
];
```

</details>

---

## Concept 11: Three.js & React Three Fiber (3D Graphics)

### What Is Three.js?

**Three.js** is a JavaScript library that makes 3D graphics in the browser possible. It wraps the complex WebGL API into something usable.

Without Three.js, you'd write hundreds of lines of low-level graphics code to draw a single cube.

### What Is React Three Fiber?

**React Three Fiber** (R3F) is a React renderer for Three.js. It lets you build 3D scenes using React components instead of imperative code.

```jsx
// ❌ Traditional Three.js (imperative)
const geometry = new THREE.BoxGeometry(1, 1, 1);
const material = new THREE.MeshStandardMaterial({ color: "orange" });
const cube = new THREE.Mesh(geometry, material);
scene.add(cube);

// ✅ React Three Fiber (declarative)
<mesh>
  <boxGeometry args={[1, 1, 1]} />
  <meshStandardMaterial color="orange" />
</mesh>;
```

### 3D Scene Fundamentals

Every 3D scene needs:

```
┌─────────────────────────────────────────────────────────────┐
│                          SCENE                              │
│                                                             │
│     ☀️ LIGHTS                                               │
│     │  (illuminate objects)                                 │
│     │                                                       │
│     ▼                                                       │
│   ┌────────────────────┐                                    │
│   │     📦 MESH        │                                    │
│   │  ┌──────────────┐  │                                    │
│   │  │  GEOMETRY    │  │  ← The shape                       │
│   │  │  (box, sphere)│  │                                    │
│   │  └──────────────┘  │                                    │
│   │  ┌──────────────┐  │                                    │
│   │  │  MATERIAL    │  │  ← The surface                     │
│   │  │  (color, etc)│  │                                    │
│   │  └──────────────┘  │                                    │
│   └────────────────────┘                                    │
│                                                             │
│   📷 CAMERA  ─────────────────────────────────────────────▶ │
│     (viewpoint)                                             │
└─────────────────────────────────────────────────────────────┘
```

**Scene**: The container for everything
**Camera**: Your viewpoint into the scene
**Light**: Illuminates objects (without light, everything is black)
**Mesh**: A visible object (geometry + material)
**Geometry**: The shape (box, sphere, custom)
**Material**: The surface appearance (color, shininess, texture)

### Basic R3F Example

```tsx
import { Canvas } from "@react-three/fiber";
import { OrbitControls } from "@react-three/drei";

function MyScene() {
  return (
    <Canvas camera={{ position: [3, 3, 3] }}>
      {/* Lighting */}
      <ambientLight intensity={0.5} />
      <directionalLight position={[10, 10, 5]} intensity={1} />

      {/* A rotating cube */}
      <mesh>
        <boxGeometry args={[1, 1, 1]} />
        <meshStandardMaterial color="hotpink" />
      </mesh>

      {/* Camera controls (drag to rotate) */}
      <OrbitControls />
    </Canvas>
  );
}
```

### Coordinate Systems

⚠️ **Important:** Semio and Three.js use different coordinate systems!

```
SEMIO (Left-handed)          THREE.JS (Right-handed)
       Z (up)                       Y (up)
       │                            │
       │                            │
       │                            │
       └───────X (right)            └───────X (right)
      /                            /
     /                            /
    Y (forward)                  Z (toward you)
```

The code converts between them using rotation matrices.

### Loading 3D Models

Semio loads models from files using Three.js loaders:

```tsx
import { useGLTF } from "@react-three/drei";

function Model({ url }) {
  // Load a .gltf or .glb model
  const { scene } = useGLTF(url);

  return <primitive object={scene} />;
}
```

### 3D Components in Design.tsx

Here's how a piece is rendered in 3D:

```tsx
const PieceMesh: FC<{ pieceGuid: Guid }> = ({ pieceGuid }) => {
  // Get piece data
  const piece = usePiece(pieceGuid);
  const type = useType(piece?.type);
  const plane = useFlatPiecePlane(pieceGuid);

  // Get selection/hover state
  const isSelected = useIsPieceSelected(pieceGuid);
  const isHovered = useIsPieceHovered(pieceGuid);

  // Calculate color
  const color = isSelected ? "blue" : isHovered ? "lightblue" : "gray";

  if (!plane) return null;

  return (
    <group position={[plane.origin.x, plane.origin.y, plane.origin.z]} rotation={calculateRotation(plane)}>
      {/* The actual 3D model */}
      <LoadedModel url={type.model.url} highlightColor={color} />

      {/* Connector spheres */}
      {type.connectors.map((connector) => (
        <ConnectorSphere key={connector.id} position={connector.point} isHighlighted={isSelected} />
      ))}
    </group>
  );
};
```

**Line-by-line:**

| Line                            | Purpose                                 |
| ------------------------------- | --------------------------------------- |
| `usePiece(pieceGuid)`           | Get piece data from the store           |
| `useFlatPiecePlane(pieceGuid)`  | Get computed 3D position/orientation    |
| `useIsPieceSelected(pieceGuid)` | Check if this piece is selected         |
| `<group position={...}>`        | Container at the piece's position       |
| `<LoadedModel>`                 | The actual 3D model                     |
| `<ConnectorSphere>`             | Visual indicators for connection points |

### The Full Scene

```tsx
function DesignScene() {
  const pieces = usePieces();

  return (
    <Canvas>
      {/* Lighting */}
      <ambientLight intensity={0.6} />
      <directionalLight position={[5, 10, 7.5]} castShadow />

      {/* All pieces */}
      {pieces.map((piece) => (
        <PieceMesh key={piece.guid} pieceGuid={piece.guid} />
      ))}

      {/* Camera controls */}
      <OrbitControls />

      {/* Grid helper */}
      <gridHelper args={[100, 100]} />
    </Canvas>
  );
}
```

### Practice Exercise

```tsx
// Exercise: Create a scene with:
// 1. A red sphere at position [0, 1, 0]
// 2. A blue box at position [2, 0.5, 0]
// 3. A green floor (flat box) at [0, 0, 0]
// 4. Ambient and directional lighting

function MyScene() {
  return <Canvas>{/* Your code here */}</Canvas>;
}
```

<details>
<summary>Click to see solution</summary>

```tsx
function MyScene() {
  return (
    <Canvas camera={{ position: [5, 5, 5] }}>
      {/* Lighting */}
      <ambientLight intensity={0.5} />
      <directionalLight position={[10, 10, 5]} />

      {/* Red sphere */}
      <mesh position={[0, 1, 0]}>
        <sphereGeometry args={[0.5, 32, 32]} />
        <meshStandardMaterial color="red" />
      </mesh>

      {/* Blue box */}
      <mesh position={[2, 0.5, 0]}>
        <boxGeometry args={[1, 1, 1]} />
        <meshStandardMaterial color="blue" />
      </mesh>

      {/* Green floor */}
      <mesh position={[0, 0, 0]} rotation={[-Math.PI / 2, 0, 0]}>
        <planeGeometry args={[10, 10]} />
        <meshStandardMaterial color="green" />
      </mesh>

      <OrbitControls />
    </Canvas>
  );
}
```

</details>

---

# 🔍 STAGE 5 — Code Walkthrough

Now that you understand all the concepts, let's walk through the actual code structure.

## Section 1: Type Definitions (Lines 216-310)

### What This Section Does

Defines TypeScript interfaces that describe the **shape** of all data used in the Design App.

### Key Types Explained

```typescript
// What is currently selected in the design
export interface DesignAppSelection {
  pieces?: Guid[]; // List of selected piece IDs
  connections?: Guid[]; // List of selected connection IDs
  connector?: {
    // Single connector for creating connections
    pieceGuid: Guid;
    connectorId: string;
  };
}
```

**Plain English Translation:**

> "Selection is like a shopping cart. It can hold:
>
> - Multiple pieces (building blocks you've clicked)
> - Multiple connections (links between blocks)
> - One connector (the port you're connecting from)"

```typescript
// Complete snapshot of everything in the Design App
export interface DesignAppState {
  fullscreenWindow: DesignAppFullscreenWindow; // Which window is fullscreen?
  panelVisibility: PanelVisibility; // Which panels are open?
  activeTool?: ToolKind; // What tool is active?
  selection?: DesignAppSelection; // What's selected?
  hover?: DesignAppHover; // What's hovered?
  camera?: Camera; // 3D camera state
  diagramCenter?: Coord; // 2D diagram pan position
  diagramScale?: number; // 2D diagram zoom level
  focusedPieceGuid?: Guid; // Piece being edited
  selectedModelTags?: Record<Guid, string[]>; // Model variant per type
  windowLayout?: string; // Window arrangement
}
```

**Plain English Translation:**

> "DesignAppState is a complete photo of the app right now. Looking at it, you can answer:
>
> - Is anything fullscreen?
> - Which side panels are visible?
> - What tool is the user using?
> - What's selected and hovered?
> - Where is the camera pointing?
> - How zoomed in is the diagram?"

---

## Section 2: Commands (Lines 311-900)

### What This Section Does

Defines every action a user can take. Each command is a function that returns what should change.

### The Command Pattern

#### Select All Pieces

```typescript
"semio.designApp.selectAll": (context) => {
  // Get all piece IDs from the design
  const allPieceGuids = context.design.pieces?.map(p => p.guid) || [];

  return {
    diff: {
      selection: {
        pieces: { added: allPieceGuids }
      }
    }
  };
}
```

**Plain English**: "Make every piece selected. Take the current design, look at every piece, add all their IDs to the selection."

#### Delete Selected

```typescript
"semio.designApp.deleteSelected": (context) => {
  const selectedPieces = context.designApp.selection?.pieces || [];
  const selectedConnections = context.designApp.selection?.connections || [];

  return {
    // Clear the selection (they're being deleted)
    diff: {
      selection: {
        pieces: { removed: selectedPieces },
        connections: { removed: selectedConnections }
      }
    },
    // Actually remove from the design data
    kitDiff: {
      designs: {
        updated: [{
          design: { guid: context.design.guid },
          diff: {
            pieces: { removed: selectedPieces.map(g => ({ guid: g })) },
            connections: { removed: selectedConnections.map(g => ({ guid: g })) }
          }
        }]
      }
    }
  };
}
```

**Plain English**: "Whatever's selected, delete it. First, un-select everything (because it won't exist anymore). Then, tell the design data to remove those pieces and connections."

---

## Section 3: The Design Store (Lines 940-1123)

### What It Does

The central "brain" that holds all state and processes changes.

### Key Parts

```typescript
export class DesignStore extends PlainKitDiffAppStore<...> {

  // Build a snapshot of current state
  buildSnapshot(): DesignAppState {
    return {
      selection: this.getSelection(),
      hover: this.getHover(),
      camera: this.getCamera(),
      // ... gather all current values
    };
  }

  // Apply a diff to change state
  applyDiff(diff: DesignAppDiff) {
    if (diff.selection) this.applySelectionDiff(diff.selection);
    if (diff.camera) this.setCamera(diff.camera);
    // ... apply each part of the diff
  }
}
```

**Plain English**: The Store is like a bank. It holds all the valuable data. When you want to check your balance, it gives you a `snapshot`. When you want to make a transaction, you give it a `diff` (what to change).

---

## Section 4: Custom Hooks (Lines 1370-2045)

### What They Do

Provide easy access to state for React components.

### The Triadic Pattern

Every hook returns three things: `[value, setter, canSet]`

```typescript
export function useDesignAppSelection(): HookResult<DesignAppSelection> {
  // Returns:
  // [0] The current selection
  // [1] A function to update the selection (or undefined)
  // [2] Whether updating is allowed
}
```

**Plain English**: "When I ask for the selection, tell me:

1. What is currently selected
2. How can I change it (or say I can't)
3. Am I even allowed to change it right now?"

### Action Hooks

```typescript
export function useDesignAppSelectPiece(): ActionHookResult<[pieceGuid: string]> {
  const store = useDesignStore();
  const canAct = store !== null;

  const action = useMemo(() => {
    if (!store) return undefined;
    return (pieceGuid: string) => {
      store.executeCommand("semio.designApp.selectPiece", pieceGuid);
    };
  }, [store]);

  return [action, canAct];
}
```

**Plain English**: "Give me a button to select a piece. If I'm in a design context, the button works. If not, the button is disabled."

---

## Section 5: The Diagram (Lines 4127-5340)

### What It Does

Renders the 2D node-edge diagram using React Flow.

### Data Transformation

```typescript
// Turn design pieces into React Flow nodes
const designToNodesAndEdges = (design, metadata, kit) => {
  const nodes: DiagramNode[] = [];
  const edges: DiagramEdge[] = [];

  // For each piece, create a node
  for (const piece of design.pieces) {
    const type = kit.types.find(t => t.guid === piece.type.guid);
    const center = metadata.get(piece.guid)?.center;

    nodes.push(pieceToNode(piece, type, center, index));
  }

  // For each connection, create an edge
  for (const connection of design.connections) {
    edges.push(connectionToEdge(connection, ...));
  }

  return { nodes, edges };
};
```

**Plain English**: "Translate the design language into diagram language. A piece becomes a node (a box on screen). A connection becomes an edge (a line between boxes)."

### The Diagram Component

```typescript
const DesignDiagram: FC = ({ reactFlowInstanceRef }) => {
  const design = useDesign();
  const kit = useKit();

  // Convert data to nodes/edges
  const { nodes, edges } = useMemo(
    () => designToNodesAndEdges(design, metadata, kit),
    [design, kit]
  );

  // Render
  return (
    <ReactFlow
      nodes={nodes}
      edges={edges}
      nodeTypes={nodeComponents}
      edgeTypes={edgeComponents}
      onNodesChange={handleNodesChange}
      onConnect={handleConnect}
    >
      <MiniMap />
      <Controls />
    </ReactFlow>
  );
};
```

---

## Section 6: The 3D Scene (Lines 6646-7285)

### What It Does

Renders pieces as 3D models in a Three.js scene.

### Key Components

```typescript
// Load and display a 3D model
const LoadedPieceMesh: FC<{
  url: string;
  fileExtension: string;
  highlightColor: string | null;
}> = ({ url, fileExtension, highlightColor }) => {
  // Choose loader based on file type
  if (fileExtension === "gltf" || fileExtension === "glb") {
    return <GLTFMesh url={url} highlightColor={highlightColor} />;
  }
  if (fileExtension === "fbx") {
    return <FBXMesh url={url} highlightColor={highlightColor} />;
  }
  // ... more formats
};
```

**Plain English**: "To show a 3D model, first figure out what format it is. GLTF? FBX? OBJ? Then use the right loader."

```typescript
// Position a piece in 3D space
const ModelPiece: FC = () => {
  const piece = usePiece();
  const plane = useFlatPiecePlane(); // Where should it be?

  return (
    <group
      position={[plane.origin.x, plane.origin.y, plane.origin.z]}
      rotation={calculateRotation(plane)}
    >
      <PieceMesh highlightColor={...} />
    </group>
  );
};
```

---

## Section 7: The Main App Component (Lines 7290-8020)

### What It Does

Orchestrates everything together: the diagram, the scene, the panels, the toolbar.

### Structure

```typescript
const App: FC = () => {
  // Initialize hooks for state access
  const [panelVisibility] = useDesignAppPanelVisibility();
  const commands = useDesignAppCommands();

  // Set up panel sections
  useEffect(() => {
    addSection("details", {
      id: "design-section",
      content: () => <DesignSection />,
    });
    // ... more sections
  }, []);

  // Render the layout
  return (
    <>
      {/* 2D Diagram Window */}
      <DiagramWindow reactFlowInstanceRef={reactFlowRef} />

      {/* 3D Scene Window */}
      <SceneWindow />

      {/* Details Panel Content */}
      <DesignSettingsContent />
    </>
  );
};
```

---

# 🧠 STAGE 6 — Mental Models

## Metaphor 1: The Restaurant

Imagine the Design App is a restaurant:

```
┌─────────────────────────────────────────────────────┐
│                    THE RESTAURANT                    │
├─────────────────────────────────────────────────────┤
│                                                      │
│  CUSTOMER (User)                                    │
│     │                                               │
│     │ "I'd like the salmon, please"                 │
│     ▼                                               │
│  WAITER (React Component)                           │
│     │                                               │
│     │ Writes down order (creates Command)           │
│     ▼                                               │
│  KITCHEN (Store + Commands)                         │
│     │                                               │
│     │ Cooks the food (processes diff)               │
│     ▼                                               │
│  PLATING (React Re-render)                          │
│     │                                               │
│     │ Presents the dish                             │
│     ▼                                               │
│  CUSTOMER sees their salmon                         │
│                                                      │
└─────────────────────────────────────────────────────┘
```

- **Menu** = Available commands
- **Order** = Command object
- **Kitchen** = Store that processes commands
- **Ingredients** = Design data (pieces, connections)
- **Plate** = The rendered UI

---

## Metaphor 2: The Film Set

The app is like a movie production:

```
DIRECTOR (User)          "Move camera to piece A"
     │
     ▼
SCRIPT (Commands)        { type: "setCamera", target: pieceA }
     │
     ▼
CAMERA CREW (State)      Updates camera position in state
     │
     ├─── CAMERAMAN 1 (Diagram) ──▶ Updates 2D view
     │
     └─── CAMERAMAN 2 (Scene)  ──▶ Updates 3D view
     │
     ▼
MONITORS (UI)            Show both views to director
```

---

## Metaphor 3: The Spreadsheet

Think of the Design App as a super-powered spreadsheet:

| Analogy       | Spreadsheet       | Design App                  |
| ------------- | ----------------- | --------------------------- |
| **Cell**      | A1, B2, C3        | A Piece                     |
| **Formula**   | =SUM(A1:A5)       | A Connection (links pieces) |
| **Worksheet** | Sheet1, Sheet2    | A Design                    |
| **Workbook**  | MyFile.xlsx       | A Kit                       |
| **Selection** | Highlighted cells | Selected pieces             |
| **Undo**      | Ctrl+Z            | Transaction stack           |

---

## Metaphor 4: The LEGO Table

```
┌───────────────────────────────────────────────┐
│              THE LEGO TABLE                    │
├───────────────────────────────────────────────┤
│                                                │
│   ┌──────────┐                                │
│   │ PARTS    │  ◄── Kit (all available types) │
│   │ DRAWER   │                                │
│   │ □ □ □    │                                │
│   │ △ △ △    │                                │
│   │ ○ ○ ○    │                                │
│   └──────────┘                                │
│                                                │
│   ┌──────────────────────────────────────┐    │
│   │         BUILDING PLATE               │    │
│   │                                      │    │
│   │    [□]───[□]───[△]                   │    │
│   │           │                          │    │
│   │          [○]                         │    │
│   │                                      │    │
│   │  ◄── Design (your creation)          │    │
│   └──────────────────────────────────────┘    │
│                                                │
│   CONNECTORS = The studs on LEGO bricks       │
│   CONNECTIONS = Where bricks snap together     │
│   SELECTION = The piece you're holding        │
│                                                │
└───────────────────────────────────────────────┘
```

---

# ✏️ STAGE 7 — Simplified Rebuild

## Pseudocode Version

```
DESIGN APP PSEUDOCODE
=====================

WHEN app starts:
    create empty State {
        pieces: []
        connections: []
        selectedPieces: []
    }
    render initial view

WHEN user clicks "Add Piece":
    newPiece = create new Piece with random ID
    add newPiece to State.pieces
    re-render diagram with new piece

WHEN user clicks a piece:
    clear State.selectedPieces
    add clicked piece to State.selectedPieces
    highlight the piece in diagram
    show piece details in panel

WHEN user drags a piece:
    update piece.position continuously
    re-render diagram

WHEN user connects two pieces:
    newConnection = {
        from: firstPiece,
        to: secondPiece
    }
    add newConnection to State.connections
    draw line between pieces

WHEN user presses Ctrl+Z:
    lastChange = pop from history stack
    reverseChange = calculate opposite of lastChange
    apply reverseChange to State
    re-render everything
```

## Minimal Working Example

```tsx
// A simplified Design App in ~100 lines

import React, { useState, useCallback } from "react";

// Types
interface Piece {
  id: string;
  name: string;
  x: number;
  y: number;
}

interface Connection {
  id: string;
  from: string;
  to: string;
}

// Main Component
function SimpleDesignApp() {
  // State
  const [pieces, setPieces] = useState<Piece[]>([]);
  const [connections, setConnections] = useState<Connection[]>([]);
  const [selectedId, setSelectedId] = useState<string | null>(null);

  // Add a new piece
  const addPiece = useCallback(() => {
    const newPiece: Piece = {
      id: `piece-${Date.now()}`,
      name: `Piece ${pieces.length + 1}`,
      x: Math.random() * 400,
      y: Math.random() * 300,
    };
    setPieces([...pieces, newPiece]);
  }, [pieces]);

  // Select a piece
  const selectPiece = useCallback((id: string) => {
    setSelectedId(id);
  }, []);

  // Delete selected piece
  const deleteSelected = useCallback(() => {
    if (!selectedId) return;

    // Remove piece
    setPieces(pieces.filter((p) => p.id !== selectedId));

    // Remove related connections
    setConnections(connections.filter((c) => c.from !== selectedId && c.to !== selectedId));

    // Clear selection
    setSelectedId(null);
  }, [selectedId, pieces, connections]);

  // Render
  return (
    <div style={{ display: "flex", height: "100vh" }}>
      {/* Toolbar */}
      <div style={{ width: 200, padding: 10, background: "#f0f0f0" }}>
        <button onClick={addPiece}>Add Piece</button>
        <button onClick={deleteSelected} disabled={!selectedId}>
          Delete Selected
        </button>

        {/* Selected piece info */}
        {selectedId && (
          <div style={{ marginTop: 20 }}>
            <strong>Selected:</strong>
            <p>{pieces.find((p) => p.id === selectedId)?.name}</p>
          </div>
        )}
      </div>

      {/* Canvas */}
      <svg style={{ flex: 1, background: "#fff" }}>
        {/* Draw pieces as circles */}
        {pieces.map((piece) => (
          <g key={piece.id} onClick={() => selectPiece(piece.id)}>
            <circle cx={piece.x} cy={piece.y} r={30} fill={piece.id === selectedId ? "#4A90D9" : "#ccc"} stroke="#333" strokeWidth={2} style={{ cursor: "pointer" }} />
            <text x={piece.x} y={piece.y} textAnchor="middle" dominantBaseline="middle" fontSize={12}>
              {piece.name}
            </text>
          </g>
        ))}

        {/* Draw connections as lines */}
        {connections.map((conn) => {
          const from = pieces.find((p) => p.id === conn.from);
          const to = pieces.find((p) => p.id === conn.to);
          if (!from || !to) return null;

          return <line key={conn.id} x1={from.x} y1={from.y} x2={to.x} y2={to.y} stroke="#666" strokeWidth={2} />;
        })}
      </svg>
    </div>
  );
}

export default SimpleDesignApp;
```

---

# 📚 STAGE 8 — Mini Lessons

## Lesson 1: Foundations – State and Rendering

### Explanation

Every UI application has **state** (data) and a **view** (what you see). React's job is to keep them in sync.

```
STATE changes → React detects change → VIEW updates
```

### Example

```tsx
function Counter() {
  const [count, setCount] = useState(0);

  return (
    <div>
      <p>Count: {count}</p>
      <button onClick={() => setCount(count + 1)}>+1</button>
    </div>
  );
}
```

### Exercise 1.1

Create a component that shows your name and has a button to change it to "Anonymous".

### Exercise 1.2

Create a component with a list of colors. Clicking a color changes the background.

---

## Lesson 2: Data Flow – Props and Context

### Explanation

Data flows DOWN through props. But sometimes data needs to jump levels – that's when you use Context.

```
      App (has theme)
       │
       ├─ Header (needs theme)
       │    └─ Logo (needs theme)
       │
       └─ Content (doesn't need theme)
            └─ Article
                 └─ Button (needs theme)  ← How does theme get here?
```

**With props**: App → Header → Content → Article → Button (tedious!)
**With context**: App provides theme, Button consumes it directly.

### Exercise 2.1

Create a "dark mode" toggle using Context that affects multiple components.

### Exercise 2.2

Create a shopping cart context that tracks items across different page components.

---

## Lesson 3: Core Logic – Commands and Diffs

### Explanation

Instead of directly changing state, we create "commands" that describe changes. This enables:

- Undo/Redo (reverse the command)
- Logging (record what happened)
- Validation (check before applying)

### Example

```typescript
// Instead of this:
function directChange() {
  selection = [pieceA, pieceB]; // Direct mutation
}

// We do this:
function commandPattern() {
  const command = {
    type: "select",
    added: [pieceA, pieceB],
    removed: [],
  };
  applyCommand(command); // Controlled mutation
  history.push(command); // Can undo later
}
```

### Exercise 3.1

Implement undo/redo for a simple text editor.

### Exercise 3.2

Create a "transaction" system where multiple changes are grouped and can be undone together.

---

## Lesson 4: Architecture – Stores and Separation of Concerns

### Explanation

Large apps separate:

- **UI** (what you see)
- **State** (data)
- **Logic** (how data changes)

```
UI Components      ←→      Hooks      ←→      Store
(presentation)         (connection)        (data + logic)
```

### Exercise 4.1

Refactor a component with mixed concerns into separate layers.

### Exercise 4.2

Create a mini store with `subscribe` and `notify` methods.

---

## Lesson 5: Advanced – Derived State and Performance

### Explanation

Sometimes you need computed data that depends on other data. Instead of recalculating every render, we "memoize" (cache) the result.

```typescript
// Bad: Recalculates every render
function Component() {
  const expensiveResult = calculateExpensiveThing(data);
}

// Good: Only recalculates when data changes
function Component() {
  const expensiveResult = useMemo(() => calculateExpensiveThing(data), [data]);
}
```

### Exercise 5.1

Create a filtered list that only recomputes when the filter or items change.

### Exercise 5.2

Implement a "derived store" that automatically updates when its source data changes.

---

# ✅ STAGE 9 — Mastery Check

## Practice Questions

### Conceptual

1. **What is the difference between "state" and "props" in React?**

2. **Why do we use diffs instead of full state snapshots for undo/redo?**

3. **What problem does the Context API solve?**

4. **In the Design App, what's the relationship between a Piece and a Type?**

5. **Why are commands defined as objects rather than direct function calls?**

### Code Reading

6. **What does this hook return and why?**

```typescript
export function useDesignAppSelection(): HookResult<DesignAppSelection>;
```

7. **What happens when this command is executed?**

```typescript
"semio.designApp.deleteSelected": (context) => {...}
```

8. **Why does `pieceToNode` return an object with `type: "piece"`?**

### Debugging Challenges

9. **Bug**: A piece shows as selected in the panel but not highlighted in the diagram. Where would you look?

10. **Bug**: Undo doesn't work after connecting two pieces. What might be missing?

11. **Bug**: Adding a piece works, but it appears at position (0,0). What's likely wrong?

## Small Projects

### Project 1: Connection Counter

Add a status bar that shows "X pieces, Y connections" and updates in real-time.

### Project 2: Selection History

Show a sidebar with the last 5 things the user selected.

### Project 3: Simple Snapping

When dragging a piece near another, snap it to align horizontally or vertically.

### Project 4: Export to JSON

Add a button that downloads the current design as a JSON file.

### Project 5: Keyboard Shortcuts

Implement Delete key for deleting selected pieces and Escape for clearing selection.

---

## Final Summary

You've now learned:

1. ✅ **Big Picture**: This is a visual editor for designing modular systems
2. ✅ **Architecture**: Data flows from user actions → commands → store → UI
3. ✅ **Logic Flow**: Events trigger commands, commands create diffs, diffs update state
4. ✅ **Core Concepts**: State, hooks, commands, diffs, stores, context
5. ✅ **Code Structure**: Types → Commands → Store → Hooks → Components
6. ✅ **Mental Models**: Restaurant, film set, LEGO table
7. ✅ **Simplified Version**: The essence in ~100 lines
8. ✅ **Learning Path**: 5 progressive lessons
9. ✅ **Practice**: Questions and projects to solidify understanding

The Design.tsx file is a sophisticated piece of software, but at its heart, it follows a simple pattern:

```
User does something
     │
     ▼
Command describes the change
     │
     ▼
Store applies the change
     │
     ▼
React shows the result
```

Everything else is refinement, optimization, and handling edge cases. Master that core loop, and you can understand any React application!
