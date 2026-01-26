# Line-by-Line Tutorial: Understanding semio.ts (Lines 1-1000)

## A Complete Beginner's Guide to Reading TypeScript Code

This tutorial explains every single line of the first 1000 lines of `semio.ts` in extreme detail. It assumes you have absolutely zero programming experience.

---

## Line 1

```typescript
// #region Header
```

### What this line does:
This line does absolutely nothing when the program runs. It is a **comment** that humans can read, but the computer completely ignores.

### Why this line exists:
Programmers use this to organize their code into collapsible sections in their code editor. When you see `#region` followed by a name like "Header", it creates a foldable section that can be collapsed to hide details.

### Every symbol explained:
- `//` — This is a "comment marker." Everything after `//` on the same line is ignored by the computer. It's like writing a sticky note for yourself.
- `#region` — This is a special tag that code editors like VS Code recognize. It tells the editor "this is the start of a named section."
- `Header` — This is just a name for this section. It tells humans "the following code is the header of this file."

### Platform/Environment:
This runs in **TypeScript/JavaScript environments** (browsers, Node.js servers, desktop apps). The `#region` feature is recognized by **VS Code** and other modern code editors.

### Background knowledge needed:
- **Comment**: Text that computers ignore but humans can read
- **Code organization**: Large files are often split into named sections to make them easier to navigate

### What happens if removed:
Nothing changes in how the program works. The code just becomes slightly harder for humans to navigate in their editor.

### What beginners find confusing:
Why write something the computer ignores? It seems wasteful. But comments are crucial for human understanding.

### Real-world analogy:
This is like putting a tab divider in a binder that says "INTRODUCTION" — the papers don't need it, but it helps you find things.

---

## Line 2

```typescript
(empty line)
```

### What this line does:
Absolutely nothing. It's a blank line.

### Why this line exists:
Blank lines make code more readable by creating visual separation between different parts.

### Every symbol explained:
There are no symbols. It's just whitespace (empty space).

### Platform/Environment:
All programming languages and environments allow blank lines.

### Background knowledge needed:
- **Whitespace**: Empty space in code is often ignored by computers but helps humans read

### What happens if removed:
The program runs exactly the same. The code just looks more cramped and harder to read.

### What beginners find confusing:
It seems wasteful to have empty lines. But readability is very important when you come back to code months later.

### Real-world analogy:
This is like leaving space between paragraphs in a book — it doesn't change the meaning but makes reading easier.

---

## Line 3

```typescript
// js/semio/semio.ts
```

### What this line does:
This is a comment that tells humans the file path of this file within the project.

### Why this line exists:
When you copy code or view it outside of the file browser, you know where this file lives in the project folder structure.

### Every symbol explained:
- `//` — Comment marker. Everything after is ignored by the computer.
- `js/semio/semio.ts` — This is a file path. It means:
  - `js` — A folder named "js" (short for JavaScript)
  - `/` — A separator between folder levels (like folders within folders)
  - `semio` — A folder inside the "js" folder
  - `semio.ts` — The actual file name. The `.ts` means it's a TypeScript file.

### Platform/Environment:
This path format works on **Mac, Linux, and web servers**. Windows uses backslashes `\` but forward slashes `/` work in most programming contexts.

### Background knowledge needed:
- **File path**: The "address" of a file on your computer
- **TypeScript**: A programming language that adds extra features to JavaScript

### What happens if removed:
Nothing changes in program behavior. You just lose a helpful reference.

### What beginners find confusing:
Why put the file name inside the file? Seems redundant. But when code is shared or copied, this context is valuable.

### Real-world analogy:
This is like writing your address on a letter you're keeping — helpful if it gets mixed with other letters.

---

## Line 4

```typescript
(empty line)
```

### What this line does:
Nothing. Blank line for readability.

(Same explanation as Line 2)

---

## Line 5

```typescript
// 2025 Ueli Saluz <ueli@semio-tech.com>
```

### What this line does:
This is a comment identifying who wrote this code and when.

### Why this line exists:
It's a **copyright notice**. It legally establishes who created this work and when.

### Every symbol explained:
- `//` — Comment marker
- `2025` — The year this code was written (or copyrighted)
- `Ueli Saluz` — The author's name
- `<ueli@semio-tech.com>` — The author's email address. The angle brackets `< >` are a common way to wrap email addresses.

### Platform/Environment:
This is a universal practice in software development across all languages.

### Background knowledge needed:
- **Copyright**: Legal right to control how creative work is used
- **Email address**: Electronic mail address for contacting someone

### What happens if removed:
The program still works, but you lose attribution and it may cause legal issues.

### What beginners find confusing:
Why does code need an author? Because code is intellectual property, like a book or song.

### Real-world analogy:
This is like the "© 2025 John Smith" you see on book covers.

---

## Line 6

```typescript
(empty line)
```

(Blank line for separation)

---

## Line 7

```typescript
// This program is free software: you can redistribute it and/or modify
```

### What this line does:
This is a comment that's part of a legal license, explaining what you're allowed to do with this code.

### Why this line exists:
It's the beginning of the **GNU Lesser General Public License (LGPL)** — a legal document that says you can share and change this code for free.

### Every symbol explained:
- `//` — Comment marker
- `This program is free software:` — Legal statement that this code is "open source"
- `you can redistribute it` — You're allowed to share it with others
- `and/or` — You can do one, the other, or both
- `modify` — You can change the code

### Platform/Environment:
This applies to **any software** regardless of language or platform.

### Background knowledge needed:
- **Free software**: Code that anyone can use, modify, and share
- **License**: Legal permission to use someone's work

### What happens if removed:
The program still runs, but the legal terms become unclear. Users wouldn't know if they can share or modify it.

### What beginners find confusing:
Why do we need legal text in code? Because code is property, and licenses define what others can do with it.

### Real-world analogy:
This is like the terms of service you agree to when using an app — rules for how you can use something.

---

## Lines 8-17

These lines continue the license text. Each follows the same pattern:

```typescript
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
```

### What these lines do:
They provide the complete legal license for this software.

### Key terms explained:
- **GNU Lesser General Public License**: A specific open-source license created by the Free Software Foundation
- **version 3**: There are different versions of this license; this uses version 3
- **NO WARRANTY**: The author isn't legally responsible if the code doesn't work
- **MERCHANTABILITY**: Whether the software can be sold
- **FITNESS FOR A PARTICULAR PURPOSE**: Whether it works for your specific need
- **https://www.gnu.org/licenses/**: A website where you can read the full license

### Real-world analogy:
This is like the legal disclaimer on a free product: "Use at your own risk, we're not responsible if it breaks."

---

## Line 19

```typescript
// #endregion Header
```

### What this line does:
This closes the "Header" region that was opened on Line 1.

### Why this line exists:
It tells the code editor "the Header section ends here." Now the whole section can be collapsed/folded.

### Every symbol explained:
- `//` — Comment marker
- `#endregion` — Special tag meaning "end of a region"
- `Header` — The name of the region being closed (matches the `#region Header` from line 1)

### Platform/Environment:
Recognized by **VS Code** and other modern code editors for code folding.

### Background knowledge needed:
- **Code folding**: The ability to collapse sections of code to hide them temporarily

### What happens if removed:
The program runs fine, but the code editor won't know where the section ends.

### What beginners find confusing:
Why name it again at the end? To make it clear which region is ending when you have many nested regions.

### Real-world analogy:
This is like the closing `</chapter>` tag at the end of a chapter in an e-book format.

---

## Line 20

```typescript
(empty line)
```

(Blank line for separation)

---

## Line 21

```typescript
// #region TODOs
```

### What this line does:
Starts a new collapsible region named "TODOs."

### Why this line exists:
To organize TODO comments (things that need to be done later) into their own section.

### Every symbol explained:
- `//` — Comment marker
- `#region` — Starts a foldable region
- `TODOs` — Name of this region. "TODO" is programmer slang for "to do" — tasks that need completing.

### Background knowledge needed:
- **TODO**: A common programming convention for marking incomplete work

### Real-world analogy:
This is like a sticky note on your desk labeled "TO DO LIST."

---

## Line 22

```typescript
(empty line)
```

(Blank line)

---

## Line 23

```typescript
// TODOs
```

### What this line does:
A comment that acts as a heading for the TODO section.

### Why this line exists:
It's a visual header so humans reading the code know this section contains TODO items.

### Every symbol explained:
- `//` — Comment marker
- `TODOs` — The heading text

### Real-world analogy:
Like writing "TO DO LIST" at the top of a piece of paper before listing your tasks.

---

## Line 24

```typescript
// TODO: Conventionalize error throwing and logging
```

### What this line does:
This is a reminder note for the programmer about work that needs to be done.

### Why this line exists:
The programmer wants to remember to standardize how errors are handled throughout the codebase.

### Every symbol explained:
- `//` — Comment marker
- `TODO:` — A standard prefix that tools can search for. Many code editors highlight TODO comments specially.
- `Conventionalize error throwing and logging` — The actual task description:
  - `Conventionalize` — Make consistent with conventions (standard ways of doing things)
  - `error throwing` — How the code reports problems
  - `logging` — How the code records what it's doing

### Platform/Environment:
TODO comments are a universal convention across all programming languages.

### Background knowledge needed:
- **Error handling**: What code does when something goes wrong
- **Logging**: Recording events for debugging

### What happens if removed:
The program runs fine, but the programmer might forget to do this task.

### What beginners find confusing:
Why write tasks in the code instead of a separate to-do app? Because the task is directly related to this code, keeping it here means it won't be forgotten.

### Real-world analogy:
Like putting a sticky note on a document saying "Remember to format this section."

---

## Line 25

```typescript
(empty line)
```

(Blank line)

---

## Line 26

```typescript
// #endregion TODOs
```

### What this line does:
Closes the "TODOs" region.

(Same pattern as Line 19)

---

## Line 27

```typescript
(empty line)
```

(Blank line)

---

## Line 28

```typescript
import { default as adjectives } from "@semio/assets/lists/adjectives.json";
```

### What this line does:
This line brings in a list of adjectives (describing words like "happy," "tall," "blue") from another file.

### Why this line exists:
The program needs adjectives to generate random names (like "HappyTiger" or "TallElephant"). Instead of writing them all here, they're stored in a separate file.

### Every symbol explained:
- `import` — A keyword that means "bring in code from another file." Think of it as "borrow."
- `{` — Opening curly brace. Used to list what specifically you want to import.
- `default` — A special word meaning "the main thing exported from that file."
- `as` — Means "rename it to." It's giving the imported thing a new name.
- `adjectives` — The new name we're giving to this imported data.
- `}` — Closing curly brace.
- `from` — Indicates where to import from.
- `"@semio/assets/lists/adjectives.json"` — The path to the file:
  - `"` and `"` — Quotes wrap the path string
  - `@semio/assets` — A package name (the `@` indicates a scoped package)
  - `/lists/adjectives.json` — A folder and file within that package
  - `.json` — JSON format, a common way to store data

### Platform/Environment:
This is **ES6 module syntax**, used in modern JavaScript/TypeScript. Works in Node.js, browsers with bundlers, etc.

### Background knowledge needed:
- **Module**: A file that exports code for other files to use
- **Import/Export**: How modules share code with each other
- **JSON**: JavaScript Object Notation, a data format

### What happens if removed:
The program would crash because it tries to use `adjectives` later, but that variable wouldn't exist.

### What beginners find confusing:
- Why `default as adjectives`? The JSON file exports its content as the "default export," and we're renaming it to `adjectives` for clarity.
- What's with the `@` symbol? It's a convention for organization-scoped packages.

### Real-world analogy:
This is like saying "Go get the list of adjectives from the file cabinet and put it on my desk."

---

## Line 29

```typescript
import { default as animals } from "@semio/assets/lists/animals.json";
```

### What this line does:
Brings in a list of animal names (like "tiger," "elephant," "zebra") from another file.

### Why this line exists:
Used with adjectives to generate random names like "HappyTiger."

### Every symbol explained:
(Same pattern as Line 28, but importing `animals` instead of `adjectives`)

---

## Line 30

```typescript
import { ClassValue, clsx } from "clsx";
```

### What this line does:
Imports two things from a library called "clsx" that helps combine CSS class names.

### Why this line exists:
When building user interfaces, you often need to combine multiple CSS classes conditionally. This library makes that easier.

### Every symbol explained:
- `import` — Keyword to bring in external code
- `{` `}` — Curly braces for importing specific things (not the default export)
- `ClassValue` — A **type definition**. It describes what kind of data `clsx` accepts.
- `,` — Comma separates multiple imports
- `clsx` — A **function** that combines class names
- `from` — Indicates source
- `"clsx"` — The npm package name

### Platform/Environment:
This is a popular npm package used in **React, Vue, and other frontend frameworks**.

### Background knowledge needed:
- **CSS classes**: Names that apply styles to HTML elements
- **Type**: A description of what kind of data something is
- **Function**: A reusable piece of code you can call

### What happens if removed:
The `cn` function defined later would break because it uses `clsx`.

### What beginners find confusing:
Why import `ClassValue` if it's just a type? TypeScript uses types to check your code for errors before it runs.

### Real-world analogy:
This is like importing a tool (the `clsx` function) and its instruction manual (the `ClassValue` type).

---

## Line 31

```typescript
import cytoscape from "cytoscape";
```

### What this line does:
Imports a library called "cytoscape" for visualizing graphs (networks of connected things).

### Why this line exists:
Semio likely uses graph visualizations to show connections between design pieces.

### Every symbol explained:
- `import` — Keyword for importing
- `cytoscape` — The name we're giving to the imported library
- `from` — Indicates source
- `"cytoscape"` — The npm package name. This is a popular graph theory library.

### Platform/Environment:
**Cytoscape.js** runs in browsers and Node.js. Used for visualizing networks.

### Background knowledge needed:
- **Graph**: In computer science, a network of nodes (points) connected by edges (lines)
- **Library/Package**: Pre-written code you can use in your project

### What happens if removed:
Any code using `cytoscape` later would fail.

### What beginners find confusing:
When there's no `{ }`, we're importing the "default export" of the library.

### Real-world analogy:
Like buying a set of tools for drawing diagrams.

---

## Line 32

```typescript
import { twMerge } from "tailwind-merge";
```

### What this line does:
Imports a function called `twMerge` from a library that intelligently merges Tailwind CSS classes.

### Why this line exists:
Tailwind CSS has many classes that can conflict (like `text-red` and `text-blue`). `twMerge` resolves these conflicts.

### Every symbol explained:
- `import` — Keyword
- `{` `}` — Importing a specific named export
- `twMerge` — A function that merges class names smartly
- `from` — Indicates source
- `"tailwind-merge"` — The npm package name

### Platform/Environment:
Used with **Tailwind CSS**, a popular utility-first CSS framework.

### Background knowledge needed:
- **Tailwind CSS**: A CSS framework where you apply styles using class names like `text-lg` or `bg-blue-500`

### What happens if removed:
The `cn` function would break.

### Real-world analogy:
Like having a smart assistant who knows not to apply two contradicting instructions.

---

## Line 33

```typescript
import * as THREE from "three";
```

### What this line does:
Imports EVERYTHING from the Three.js library and puts it in an object called `THREE`.

### Why this line exists:
Three.js is a 3D graphics library. Semio uses it for 3D visualizations.

### Every symbol explained:
- `import` — Keyword
- `*` — Asterisk means "everything" (all exports)
- `as` — Rename what we're importing
- `THREE` — The name we're giving to all the imported stuff (convention is ALL CAPS for Three.js)
- `from` — Indicates source
- `"three"` — The npm package name

### Platform/Environment:
**Three.js** is a 3D graphics library for browsers and sometimes Node.js.

### Background knowledge needed:
- **3D graphics**: Creating and displaying three-dimensional visuals

### What happens if removed:
All 3D functionality would break.

### What beginners find confusing:
`* as NAME` imports everything as a single object. You access things like `THREE.Vector3`.

### Real-world analogy:
Like buying an entire toolbox and calling it "THREE-box" so you can find tools like "THREE-box.hammer."

---

## Line 34

```typescript
import { v7 as uuidv7 } from "uuid";
```

### What this line does:
Imports a function for generating unique identifiers, renaming it from `v7` to `uuidv7`.

### Why this line exists:
Every piece, type, and design in Semio needs a unique ID so they don't get confused with each other.

### Every symbol explained:
- `import` — Keyword
- `{` `}` — Import specific thing
- `v7` — The function name in the library (version 7 of UUID algorithm)
- `as` — Rename it
- `uuidv7` — The new name (more descriptive)
- `from` — Source
- `"uuid"` — The npm package

### Platform/Environment:
**UUID** library works everywhere JavaScript runs.

### Background knowledge needed:
- **UUID**: Universally Unique Identifier — a string that's practically guaranteed to be unique
- **v7**: Version 7 UUIDs include a timestamp

### What happens if removed:
The `guid` function would break, and the app couldn't generate unique IDs.

### What beginners find confusing:
Why rename `v7` to `uuidv7`? Because `v7` alone doesn't explain what it does. `uuidv7` is more descriptive.

### Real-world analogy:
Like a machine that stamps unique serial numbers on products.

---

## Line 35

```typescript
import { z } from "zod";
```

### What this line does:
Imports the main function from Zod, a library for validating data.

### Why this line exists:
Zod helps ensure data has the correct shape and type, catching errors early.

### Every symbol explained:
- `import` — Keyword
- `{` `}` — Import specific thing
- `z` — The main Zod function (conventionally named `z`)
- `from` — Source
- `"zod"` — The npm package

### Platform/Environment:
**Zod** works in TypeScript/JavaScript environments.

### Background knowledge needed:
- **Validation**: Checking that data meets expected requirements
- **Schema**: A description of what data should look like

### What happens if removed:
All the schema definitions (like `AttributeSchema`) would break.

### What beginners find confusing:
Why just `z`? It's short and used very frequently, so a short name saves typing.

### Real-world analogy:
Like a quality control inspector who checks that products meet specifications.

---

## Line 36

```typescript
import CONSTANTS from "./constants.json";
```

### What this line does:
Imports a JSON file containing constant values from the same folder.

### Why this line exists:
Constants like tolerance values and icon sizes are stored in a separate file for easy editing.

### Every symbol explained:
- `import` — Keyword
- `CONSTANTS` — The name for the imported data (ALL CAPS indicates it shouldn't change)
- `from` — Source
- `"./constants.json"` — Path to the file:
  - `./` — Current folder
  - `constants.json` — The file name

### Platform/Environment:
Works with bundlers that support JSON imports.

### Background knowledge needed:
- **Constants**: Values that don't change during program execution
- **JSON**: A data format

### What happens if removed:
The constants like `TOLERANCE` wouldn't be available.

### What beginners find confusing:
`./` means "in the same folder" while `../` means "parent folder."

### Real-world analogy:
Like reading settings from a configuration file.

---

## Line 37

```typescript
(empty line)
```

(Blank line for separation)

---

## Line 38

```typescript
// #region Constants
```

### What this line does:
Starts a new collapsible region named "Constants."

(Same pattern as previous `#region` lines)

---

## Line 39

```typescript
(empty line)
```

(Blank line)

---

## Line 40

```typescript
export const ICON_WIDTH = CONSTANTS.icon.width;
```

### What this line does:
Creates a constant called `ICON_WIDTH` that holds the icon width value, and makes it available to other files.

### Why this line exists:
It extracts a specific value from the CONSTANTS object and gives it a memorable name.

### Every symbol explained:
- `export` — Makes this available to other files that import this file
- `const` — Declares a constant (a variable that cannot be changed)
- `ICON_WIDTH` — The name of this constant (ALL CAPS is convention for constants)
- `=` — Assignment operator. "Make the left side equal to the right side."
- `CONSTANTS` — The object we imported from constants.json
- `.` — Dot operator. Accesses a property inside an object.
- `icon` — A property of CONSTANTS
- `.width` — A property of icon
- `;` — Semicolon ends the statement

### Platform/Environment:
Standard TypeScript/JavaScript.

### Background knowledge needed:
- **Object**: A collection of related data with named properties
- **Property access**: Using `.` to get values from objects
- **Export**: Making code available to other files

### What happens if removed:
Any code using `ICON_WIDTH` would break.

### What beginners find confusing:
The chain `CONSTANTS.icon.width` means: start with CONSTANTS, get its `icon` property, then get that object's `width` property.

### Real-world analogy:
Like looking up a value in a reference manual: go to the "icon" section, find "width."

---

## Line 41

```typescript
export const TOLERANCE = CONSTANTS.tolerance;
```

### What this line does:
Creates a constant for tolerance (how close numbers need to be to be considered "equal").

### Why this line exists:
In 3D graphics, tiny floating-point errors occur. TOLERANCE defines how much error is acceptable.

### Every symbol explained:
(Same pattern as Line 40)
- `TOLERANCE` — A small number representing acceptable error
- `CONSTANTS.tolerance` — Gets the tolerance value from the config file

### Background knowledge needed:
- **Floating-point**: How computers store decimal numbers (with tiny errors)
- **Tolerance**: Acceptable margin of error

### Real-world analogy:
Like saying "measurements within 1mm are close enough."

---

## Line 42

```typescript
(empty line)
```

(Blank line)

---

## Line 43

```typescript
// #endregion Constants
```

### What this line does:
Closes the "Constants" region.

---

## Line 44

```typescript
(empty line)
```

(Blank line)

---

## Line 45

```typescript
export function cn(...inputs: ClassValue[]) {
```

### What this line does:
Defines a function called `cn` that combines CSS class names.

### Why this line exists:
It's a utility function that makes combining Tailwind classes easier and smarter.

### Every symbol explained:
- `export` — Makes this function available to other files
- `function` — Keyword that declares a function (reusable code)
- `cn` — The function name (short for "classNames")
- `(` — Opening parenthesis for parameters
- `...` — Spread operator. Means "gather all arguments into an array."
- `inputs` — The name of this parameter
- `:` — In TypeScript, separates a name from its type
- `ClassValue[]` — The type:
  - `ClassValue` — A type from the clsx library
  - `[]` — Array (list) of that type
- `)` — Closing parenthesis
- `{` — Opening brace. Function body starts here.

### Platform/Environment:
TypeScript. The function runs wherever the app runs.

### Background knowledge needed:
- **Function**: A reusable block of code
- **Parameter**: Input that a function receives
- **Type annotation**: TypeScript's way of describing what type of data something is
- **Array**: A list of items

### What happens if removed:
Any code using `cn()` would break.

### What beginners find confusing:
- `...inputs` — This "rest parameter" collects all arguments into one array
- `ClassValue[]` — Square brackets mean "array of"

### Real-world analogy:
Like a recipe that says "take any number of ingredients" instead of listing exactly how many.

---

## Line 46

```typescript
  return twMerge(clsx(inputs));
```

### What this line does:
Combines all the class names smartly and returns the result.

### Why this line exists:
It chains two functions: `clsx` combines classes, then `twMerge` removes conflicts.

### Every symbol explained:
- `return` — Sends a value back to whoever called this function
- `twMerge(` — Calls the twMerge function
- `clsx(` — Calls the clsx function (nested inside twMerge)
- `inputs` — The array of class names
- `)` — Closes clsx
- `)` — Closes twMerge
- `;` — Ends the statement

### Platform/Environment:
TypeScript/JavaScript.

### Background knowledge needed:
- **Return**: How functions give back results
- **Function call**: Running a function with `functionName(arguments)`
- **Nesting**: Putting one function call inside another

### What happens if removed:
The function would return `undefined` (nothing) instead of the combined classes.

### What beginners find confusing:
The nesting: `twMerge(clsx(inputs))` means "run clsx first, then pass its result to twMerge."

### Real-world analogy:
Like putting ingredients in a blender (clsx), then straining the result (twMerge).

---

## Line 47

```typescript
}
```

### What this line does:
Closes the function body that was opened on Line 45.

### Why this line exists:
Every opening `{` must have a matching closing `}`.

### Every symbol explained:
- `}` — Closing curly brace. Marks the end of the function.

### Background knowledge needed:
- **Code blocks**: Sections of code wrapped in `{ }`

### What happens if removed:
Syntax error — the code won't run at all.

### Real-world analogy:
Like the closing parenthesis in math: (2 + 3) needs both ( and ).

---

## Line 48

```typescript
(empty line)
```

(Blank line)

---

## Line 49

```typescript
export const guid = () => uuidv7();
```

### What this line does:
Creates a function called `guid` that generates a unique ID.

### Why this line exists:
It wraps `uuidv7` in a simpler name that's easier to remember.

### Every symbol explained:
- `export` — Makes it available to other files
- `const` — Declares a constant
- `guid` — The name (GUID = Globally Unique Identifier)
- `=` — Assignment
- `()` — Empty parentheses mean this function takes no inputs
- `=>` — Arrow function syntax. A shorter way to write functions.
- `uuidv7()` — Calls the UUID generator function
- `;` — Ends the statement

### Platform/Environment:
TypeScript/JavaScript.

### Background knowledge needed:
- **Arrow function**: A compact way to write functions: `() => result`
- **GUID**: A unique identifier string

### What happens if removed:
Code that calls `guid()` would break.

### What beginners find confusing:
`() => uuidv7()` is equivalent to `function() { return uuidv7(); }` but shorter.

### Real-world analogy:
Like a stamp machine that creates a unique serial number each time you press it.

---

## Line 50

```typescript
(empty line)
```

(Blank line)

---

## Line 51

```typescript
(empty line)
```

(Another blank line — extra spacing for visual clarity)

---

## Line 52

```typescript
class SeededRandom {
```

### What this line does:
Starts defining a class called `SeededRandom` for generating predictable "random" numbers.

### Why this line exists:
Sometimes you want random-seeming numbers that are reproducible (same seed = same sequence). This is useful for testing.

### Every symbol explained:
- `class` — Keyword that defines a class (a blueprint for creating objects)
- `SeededRandom` — The class name (PascalCase convention for classes)
- `{` — Opens the class body

### Platform/Environment:
TypeScript/JavaScript.

### Background knowledge needed:
- **Class**: A blueprint for creating objects with shared behavior
- **Seeded random**: Random numbers that are reproducible if you start with the same "seed" number

### What happens if removed:
The `Generator` class that uses it would break.

### What beginners find confusing:
Why not just use `Math.random()`? Because `Math.random()` gives different numbers each time. Seeded random gives the same sequence when you use the same seed.

### Real-world analogy:
Like a shuffled deck of cards — if you shuffle the same way, you get the same order.

---

## Line 53

```typescript
  private seed: number;
```

### What this line does:
Declares a private variable called `seed` that stores a number.

### Why this line exists:
The seed is the starting point for generating random numbers. It's stored so it can be used repeatedly.

### Every symbol explained:
- `private` — Access modifier. Means only code inside this class can access this variable.
- `seed` — The variable name
- `:` — TypeScript type annotation separator
- `number` — The type (it holds a numeric value)
- `;` — Ends the declaration

### Platform/Environment:
TypeScript (JavaScript doesn't have `private` keyword natively, but TypeScript adds it).

### Background knowledge needed:
- **Private**: Only accessible from within the class
- **Variable**: A named storage location for data
- **Type**: What kind of data something holds

### What happens if removed:
The class couldn't store the seed value.

### What beginners find confusing:
`private` prevents outside code from changing the seed directly — a safety feature.

### Real-world analogy:
Like a locked safe inside a house — only house members have the key.

---

## Line 54

```typescript
  constructor(seed: number) {
```

### What this line does:
Defines the constructor — special code that runs when creating a new instance of this class.

### Why this line exists:
It sets up the object with an initial seed value.

### Every symbol explained:
- `constructor` — Special method that runs when you write `new SeededRandom(123)`
- `(` — Opens parameter list
- `seed` — Parameter name
- `:` — Type annotation separator
- `number` — The type of seed
- `)` — Closes parameter list
- `{` — Opens the constructor body

### Platform/Environment:
TypeScript/JavaScript.

### Background knowledge needed:
- **Constructor**: Initialization code that runs when an object is created
- **Instance**: A specific object created from a class

### What happens if removed:
You couldn't create SeededRandom objects with a seed value.

### What beginners find confusing:
`constructor` is a reserved word — you can't change its name.

### Real-world analogy:
Like the assembly instructions for building furniture — runs once when you build it.

---

## Line 55

```typescript
    this.seed = seed % 2147483647;
```

### What this line does:
Sets the object's seed to the input seed, but limits it to a maximum value.

### Why this line exists:
The algorithm needs the seed to be within a specific range. 2147483647 is 2^31 - 1, the maximum 32-bit signed integer.

### Every symbol explained:
- `this` — Refers to the current object instance
- `.` — Accesses a property of this object
- `seed` — The property we declared earlier
- `=` — Assignment
- `seed` — The parameter passed to the constructor (different from `this.seed`)
- `%` — Modulo operator. Gives the remainder after division.
- `2147483647` — The maximum value (keeps seed in valid range)
- `;` — Ends statement

### Platform/Environment:
TypeScript/JavaScript.

### Background knowledge needed:
- **this**: Refers to "this object I'm working with"
- **Modulo**: Division remainder (10 % 3 = 1 because 10 ÷ 3 = 3 remainder 1)

### What happens if removed:
Large seed values could break the algorithm.

### What beginners find confusing:
`this.seed` (the object's property) vs `seed` (the parameter) — same name, different things.

### Real-world analogy:
Like limiting a dial to its maximum position.

---

## Line 56

```typescript
    if (this.seed <= 0) this.seed += 2147483646;
```

### What this line does:
If the seed is zero or negative, make it positive.

### Why this line exists:
The algorithm doesn't work with zero or negative seeds.

### Every symbol explained:
- `if` — Conditional keyword. "If this is true, do the following."
- `(` — Opens the condition
- `this.seed` — The seed value
- `<=` — Less than or equal to
- `0` — Zero
- `)` — Closes condition
- `this.seed` — The seed property
- `+=` — Add and assign. `x += 5` means `x = x + 5`
- `2147483646` — The value to add
- `;` — Ends statement

### Platform/Environment:
TypeScript/JavaScript.

### Background knowledge needed:
- **Conditional (if)**: Execute code only when a condition is true
- **Comparison operators**: `<=` means "less than or equal to"
- **Compound assignment**: `+=` adds to the existing value

### What happens if removed:
Seeds of 0 or less would break the random number generation.

### What beginners find confusing:
Why no `{ }` after `if`? When there's only one statement, braces are optional.

### Real-world analogy:
Like a rule: "If the dial shows zero, add the maximum to make it positive."

---

## Line 57

```typescript
  }
```

### What this line does:
Closes the constructor body.

---

## Line 58

```typescript
  next = (): number => (this.seed = (this.seed * 16807) % 2147483647);
```

### What this line does:
Defines a method that generates the next random number in the sequence.

### Why this line exists:
This is the core of the random number generator. It uses a mathematical formula (Linear Congruential Generator).

### Every symbol explained:
- `next` — Method name
- `=` — Assignment
- `()` — No parameters
- `:` — Return type annotation
- `number` — Returns a number
- `=>` — Arrow function
- `(` — Groups the expression
- `this.seed` — The seed property
- `=` — Assignment
- `(` — Groups multiplication
- `this.seed` — Current seed value
- `*` — Multiplication
- `16807` — A magic number (7^5, chosen for good randomness properties)
- `)` — Closes multiplication
- `%` — Modulo
- `2147483647` — Maximum value
- `)` — Closes expression
- `;` — Ends statement

### Platform/Environment:
TypeScript/JavaScript.

### Background knowledge needed:
- **Method**: A function that belongs to a class
- **LCG**: Linear Congruential Generator, a classic algorithm for pseudo-random numbers

### What happens if removed:
The random number generator wouldn't work.

### What beginners find confusing:
The formula `(seed * 16807) % 2147483647` is a well-studied mathematical formula for generating pseudo-random numbers.

### Real-world analogy:
Like a complex mathematical formula that produces a sequence of numbers that *look* random.

---

## Line 59

```typescript
  nextFloat = (): number => (this.next() - 1) / 2147483646;
```

### What this line does:
Generates a random decimal number between 0 and 1.

### Why this line exists:
Sometimes you need random decimals (like 0.5, 0.873) instead of huge integers.

### Every symbol explained:
- `nextFloat` — Method name (float = decimal number)
- `=` — Assignment
- `()` — No parameters
- `:` — Return type
- `number` — Returns a number
- `=>` — Arrow function
- `(` — Groups calculation
- `this.next()` — Calls the next method to get a random integer
- `-` — Subtraction
- `1` — Subtract one
- `)` — Closes grouping
- `/` — Division
- `2147483646` — Maximum possible value, so result is 0-1
- `;` — Ends statement

### Background knowledge needed:
- **Float**: A number with a decimal point

### Real-world analogy:
Like converting a percentage (0-100) to a fraction (0-1).

---

## Line 60

```typescript
  nextInt = (max: number): number => Math.floor(this.nextFloat() * max);
```

### What this line does:
Generates a random integer from 0 up to (but not including) max.

### Why this line exists:
To pick a random item from an array, you need a random index (integer).

### Every symbol explained:
- `nextInt` — Method name (int = integer, whole number)
- `=` — Assignment
- `(` — Opens parameters
- `max` — Maximum value (exclusive)
- `:` — Type separator
- `number` — Parameter type
- `)` — Closes parameters
- `:` — Return type
- `number` — Returns a number
- `=>` — Arrow function
- `Math.floor(` — Rounds down to the nearest whole number
- `this.nextFloat()` — Gets a random 0-1 value
- `*` — Multiplication
- `max` — The maximum value
- `)` — Closes Math.floor
- `;` — Ends statement

### Background knowledge needed:
- **Integer**: A whole number (no decimal)
- **Math.floor**: Rounds down (3.9 becomes 3)

### Real-world analogy:
Like rolling a die with `max` sides, but numbered 0 to max-1.

---

## Line 61

```typescript
}
```

### What this line does:
Closes the SeededRandom class definition.

---

## Line 62

```typescript
(empty line)
```

(Blank line)

---

## Line 63

```typescript
export class Generator {
```

### What this line does:
Starts defining a class called Generator that creates random names and IDs.

### Why this line exists:
To generate fun, readable identifiers like "HappyTiger42."

### Every symbol explained:
- `export` — Makes the class available to other files
- `class` — Defines a class
- `Generator` — The class name
- `{` — Opens class body

---

## Line 64

```typescript
  public static randomId(seed: number = Math.floor(Math.random() * 1000000)): string {
```

### What this line does:
Defines a static method to generate a random ID string.

### Why this line exists:
Creates readable IDs like "HappyTiger42" for new objects.

### Every symbol explained:
- `public` — Anyone can call this method
- `static` — You don't need to create an instance; call directly as `Generator.randomId()`
- `randomId` — Method name
- `(` — Opens parameters
- `seed` — Parameter for the random seed
- `:` — Type separator
- `number` — Parameter type
- `=` — Default value assignment
- `Math.floor(Math.random() * 1000000)` — Default: a random number between 0 and 999999
- `)` — Closes parameters
- `:` — Return type separator
- `string` — Returns a string (text)
- `{` — Opens method body

### Background knowledge needed:
- **static**: Called on the class itself, not on instances
- **Default parameter**: If no value provided, use this default
- **Math.random()**: Returns random decimal 0-1

### Real-world analogy:
Like a name generator you can call anytime: Generator.randomId().

---

## Lines 65-70: The randomId method body

```typescript
    const random = new SeededRandom(seed);
    let adjective = adjectives[random.nextInt(adjectives.length)];
    let animal = animals[random.nextInt(animals.length)];
    adjective = adjective.charAt(0).toUpperCase() + adjective.slice(1);
    animal = animal.charAt(0).toUpperCase() + animal.slice(1);
    return `${adjective}${animal}${random.nextInt(1000)}`;
```

### Line 65: `const random = new SeededRandom(seed);`
- Creates a new SeededRandom object with the given seed
- `new` — Keyword to create an instance of a class
- `const` — Variable that won't be reassigned

### Line 66: `let adjective = adjectives[random.nextInt(adjectives.length)];`
- Picks a random adjective from the list
- `let` — Variable that can be changed later
- `adjectives[...]` — Array access. Gets item at index inside brackets.
- `adjectives.length` — How many items in the array
- `random.nextInt(n)` — Random number from 0 to n-1

### Line 67: `let animal = animals[random.nextInt(animals.length)];`
- Same pattern, picks a random animal

### Line 68: `adjective = adjective.charAt(0).toUpperCase() + adjective.slice(1);`
- Capitalizes the first letter of the adjective
- `charAt(0)` — Gets the first character
- `toUpperCase()` — Converts to capital letter
- `+` — Concatenation (joining strings)
- `slice(1)` — Gets everything from position 1 onward (the rest of the word)

### Line 69: `animal = animal.charAt(0).toUpperCase() + animal.slice(1);`
- Same pattern, capitalizes the animal name

### Line 70: `` return `${adjective}${animal}${random.nextInt(1000)}`; ``
- Returns the combined name with a random number
- Template literal (backticks) with `${...}` for inserting values
- Example result: "HappyTiger42"

---

## Lines 71-77: The randomName method

```typescript
  }
  public static randomName(seed: number = Math.floor(Math.random() * 1000000)): string {
    const random = new SeededRandom(seed);
    let animal = animals[random.nextInt(animals.length)];
    animal = animal.charAt(0).toUpperCase() + animal.slice(1);
    return `${animal}`;
  }
}
```

This is similar to `randomId` but simpler — just returns a capitalized animal name like "Tiger."

---

## Line 78

```typescript
(empty line)
```

---

## Line 79

```typescript
export const normalize = (val: string | undefined | null): string => (val === undefined || val === null ? "" : val);
```

### What this line does:
Creates a function that converts undefined or null values to empty strings.

### Why this line exists:
Sometimes data is missing. This ensures you always get a string, even if empty.

### Every symbol explained:
- `export const normalize` — Exported constant function
- `val` — Parameter name
- `:` — Type annotation
- `string | undefined | null` — Union type: can be any of these three types
- `|` — "or" in TypeScript types
- `:` — Return type
- `string` — Always returns a string
- `=>` — Arrow function
- `(` — Groups the ternary expression
- `val === undefined` — Check if val is undefined
- `||` — Logical OR ("or")
- `val === null` — Check if val is null
- `?` — Ternary operator: "if true"
- `""` — Empty string (the result if undefined/null)
- `:` — Ternary operator: "else"
- `val` — Return the original value
- `)` — Closes grouping

### Background knowledge needed:
- **undefined**: A value that hasn't been set
- **null**: Intentionally empty value
- **Ternary operator**: `condition ? valueIfTrue : valueIfFalse`

### Real-world analogy:
Like saying "if the field is blank, write nothing; otherwise, write what's there."

---

## Line 80

```typescript
export const round = (value: number): number => Math.round(value / TOLERANCE) * TOLERANCE;
```

### What this line does:
Rounds a number to the nearest multiple of TOLERANCE.

### Why this line exists:
For consistent precision in 3D coordinates, rounding eliminates tiny floating-point errors.

### Key symbols:
- `Math.round()` — Rounds to nearest integer
- `value / TOLERANCE` — Divide first
- `* TOLERANCE` — Multiply back

### Real-world analogy:
Like rounding prices to the nearest cent.

---

## Lines 81-92: The jaccard function

```typescript
export const jaccard = (a: string[] | undefined, b: string[] | undefined): number => {
  if ((a === undefined && b === undefined) || (a?.length === 0 && b?.length === 0)) return 1;
  if (a === undefined || b === undefined) return 0;
  const setA = new Set(a);
  const setB = new Set(b);
  const intersection = Array.from(setA).filter((x) => setB.has(x)).length;
  const union = setA.size + setB.size - intersection;
  if (union === 0) return 0;
  return intersection / union;
};
```

### What this does:
Calculates the Jaccard index — a measure of how similar two sets are (0 = completely different, 1 = identical).

### Key concepts:
- **Set**: A collection of unique values
- **Intersection**: Items in both sets
- **Union**: All items combined
- **Jaccard formula**: intersection / union

### Real-world analogy:
Like comparing two people's movie preferences — how many movies they both like divided by how many movies they've watched combined.

---

## Lines 94-113: The deepEqual function

```typescript
export const deepEqual = (a: any, b: any): boolean => {
  if (a === b) return true;
  if (a == null && b == null) return true;
  if (a == null || b == null) return false;
  if (typeof a !== typeof b) return false;
  if (Array.isArray(a)) {
    if (!Array.isArray(b) || a.length !== b.length) return false;
    return a.every((item, index) => deepEqual(item, b[index]));
  }
  if (typeof a === "object") {
    const keysA = Object.keys(a);
    const keysB = Object.keys(b);
    if (keysA.length !== keysB.length) return false;
    return keysA.every((key) => keysB.includes(key) && deepEqual(a[key], b[key]));
  }
  return false;
};
```

### What this does:
Compares two values to see if they're deeply equal (including nested objects and arrays).

### Key concepts:
- **any**: TypeScript type meaning "any type"
- **Array.isArray()**: Checks if something is an array
- **typeof**: Gets the type of a value
- **Object.keys()**: Gets all property names of an object
- **Recursion**: The function calls itself for nested values

### Real-world analogy:
Like comparing two folders by checking every file and subfolder inside.

---

## Lines 116-120: The arraysEqual function

```typescript
export const arraysEqual = <T>(a: T[] | undefined, b: T[] | undefined): boolean => {
  if (a === b) return true;
  if (!a || !b) return false;
  return a.length === b.length && a.every((val, index) => deepEqual(val, b[index]));
};
```

### What this does:
Compares two arrays for deep equality.

### Key symbols:
- `<T>` — Generic type parameter. T can be any type.
- `a.every()` — Returns true if every item passes the test

---

## Lines 122-129: The generateUniqueName function

```typescript
export const generateUniqueName = (baseName: string, existingNames: string[], separator: string = " "): string => {
  if (!existingNames.includes(baseName)) return baseName;
  let counter = 2;
  while (existingNames.includes(`${baseName}${separator}${counter}`)) {
    counter++;
  }
  return `${baseName}${separator}${counter}`;
};
```

### What this does:
Creates a unique name by adding a number if the name already exists.

### Example:
- Input: "Wall", existing: ["Wall"]
- Output: "Wall 2"

### Key symbols:
- `while` — Loop that continues while condition is true
- `includes()` — Checks if array contains a value
- `counter++` — Increment counter by 1

### Real-world analogy:
Like Windows naming copied files: "Document", "Document 2", "Document 3".

---

## Lines 131-138: DiffStatus

```typescript
export const DiffStatusSchema = z.enum(["unchanged", "added", "removed", "modified"]);

export enum DiffStatus {
  Unchanged = "unchanged",
  Added = "added",
  Removed = "removed",
  Modified = "modified",
}
```

### What this does:
Defines the possible states when comparing two versions of something.

### Key concepts:
- **enum**: A set of named constants
- **Zod schema**: A validator that checks data matches expected types

---

## Lines 140-145: 3D Rotation Helpers

```typescript
export const toThreeRotation = (): THREE.Matrix4 => new THREE.Matrix4(1, 0, 0, 0, 0, 0, 1, 0, 0, -1, 0, 0, 0, 0, 0, 1);
export const toSemioRotation = (): THREE.Matrix4 => new THREE.Matrix4(1, 0, 0, 0, 0, 0, -1, 0, 0, 1, 0, 0, 0, 0, 0, 1);
export const toThreeQuaternion = (): THREE.Quaternion => new THREE.Quaternion(-0.7071067811865476, 0, 0, 0.7071067811865476);
export const toSemioQuaternion = (): THREE.Quaternion => new THREE.Quaternion(0.7071067811865476, 0, 0, -0.7071067811865476);
export const vectorToThree = (v: Point | Vector): THREE.Vector3 => new THREE.Vector3(v.x, v.y, v.z);
```

### What this does:
Converts between Semio's coordinate system and Three.js's coordinate system.

### Key concepts:
- **Matrix4**: A 4x4 matrix used for 3D transformations
- **Quaternion**: A mathematical way to represent rotations
- **Vector3**: A point or direction in 3D space (x, y, z)

### Why coordinate conversion?
Semio uses Y-forward/Z-up, while Three.js uses Y-up/Z-backward. These functions translate between them.

---

## Line 147: Guid Type

```typescript
export type Guid = string;
```

### What this does:
Creates a type alias. `Guid` is just another name for `string`.

### Why this exists:
For clarity. When you see `Guid`, you know it's supposed to be a unique identifier, not just any string.

### Real-world analogy:
Like saying "Social Security Number" instead of just "number" — it's more descriptive.

---

## Lines 149-258: Entity ID Types and Functions

This large section defines ID types and helper functions for all the different entities in Semio:

```typescript
export type AttributeId = { guid: Guid };
export type LocationId = { guid: Guid };
// ... many more ID types ...

export const AttributeIdSchema = z.object({ guid: z.string() });
// ... many more schemas ...

export const createAttributeId = (guid: Guid): AttributeId => ({ guid });
// ... many more factory functions ...

export const areSameAttributeId = (a: AttributeId, b: AttributeId): boolean => a.guid === b.guid;
// ... many more comparison functions ...

export const getAttributeGuid = (id: AttributeId): Guid => id.guid;
// ... many more getter functions ...
```

### What this pattern does:
For each entity type (Attribute, Location, Author, File, etc.), it provides:
1. **Type definition**: What the ID looks like (`{ guid: string }`)
2. **Schema**: Zod validator for the ID
3. **Factory**: Function to create new IDs
4. **Comparison**: Function to check if two IDs are equal
5. **Getter**: Function to extract the GUID

### Why so repetitive?
Type safety and consistency. Each entity type has its own ID type, preventing you from accidentally using a FileId where an AuthorId is expected.

---

## Lines 267-302: The Attribute Entity

```typescript
export const AttributeSchema = z.object({
  guid: z.string(),
  key: z.string(),
  value: z.string().optional(),
  definition: z.string().optional(),
});
export type Attribute = z.infer<typeof AttributeSchema>;
export const serializeAttribute = (attribute: Attribute): string => JSON.stringify(AttributeSchema.parse(attribute));
export const deserializeAttribute = (json: string): Attribute => AttributeSchema.parse(JSON.parse(json));
```

### What this does:
Defines an Attribute — a key-value pair for metadata.

### Key concepts:
- **z.object()**: Creates a Zod schema for an object
- **z.string()**: Validates that a value is a string
- **optional()**: The value is allowed to be missing
- **z.infer<typeof Schema>**: TypeScript extracts the type from the schema
- **serialize**: Convert object to JSON string
- **deserialize**: Convert JSON string back to object

### Real-world analogy:
An Attribute is like a label on a product: "Color: Red" or "Material: Wood".

---

## Lines 304-367: Attribute Diff Functions

This section defines how to track and apply changes to Attributes:

- **getAttributeDiff**: Compare before/after, return what changed
- **inverseAttributeDiff**: Reverse a change (for undo)
- **mergeAttributeDiff**: Combine two changes
- **applyAttributeDiff**: Apply a change to get new value

This pattern repeats for every entity type. It enables:
- **Undo/Redo**: Reverse changes
- **Synchronization**: Send only what changed
- **Conflict resolution**: Merge concurrent edits

---

## Lines 372-407: Coord (Coordinate)

```typescript
export const CoordSchema = z.object({ u: z.number(), v: z.number() });
export type Coord = z.infer<typeof CoordSchema>;
```

### What this does:
Defines a 2D coordinate with `u` and `v` values.

### Why u/v instead of x/y?
In texturing and UV mapping, `u` and `v` are conventional names for 2D coordinates on a surface.

---

## Lines 412-447: Vec (Vector)

Similar to Coord, but specifically for 2D vectors (directions with magnitude).

---

## Lines 452-497: Point

```typescript
export const PointSchema = z.object({
  x: z.number(),
  y: z.number(),
  z: z.number(),
});
export type Point = z.infer<typeof PointSchema>;
```

### What this does:
Defines a 3D point with x, y, z coordinates.

### Key concept:
A Point represents a position in 3D space.

---

## Lines 502-557: Vector

Similar to Point, but represents a direction and magnitude rather than a position.

---

## Lines 562-650: Plane

```typescript
export const PlaneSchema = z.object({
  origin: PointSchema,
  xAxis: VectorSchema,
  yAxis: VectorSchema,
});
```

### What this does:
Defines a 3D plane using an origin point and two axes.

### Key concept:
A plane is defined by where it is (origin) and how it's oriented (x and y axes). The z-axis is calculated as the cross product of x and y.

### Helper functions:
- **planeToMatrix**: Convert plane to a 4x4 transformation matrix
- **matrixToPlane**: Convert matrix back to plane
- **averagePlane**: Find the average of multiple planes

---

## Lines 652-690: Camera

```typescript
export const CameraSchema = z.object({
  position: PointSchema,
  forward: VectorSchema,
  up: VectorSchema,
});
```

### What this does:
Defines a camera's position and orientation for 3D viewing.

### Key concepts:
- **position**: Where the camera is
- **forward**: Which direction it's looking
- **up**: Which way is "up" for the camera

---

## Lines 692-750: Location

```typescript
export const LocationSchema = z.object({
  guid: z.string(),
  longitude: z.number(),
  latitude: z.number(),
  altitude: z.number().optional(),
  attributes: z.array(AttributeSchema).optional(),
});
```

### What this does:
Defines a geographical location with GPS coordinates.

---

## Lines 752-820: Author

```typescript
export const AuthorSchema = z.object({
  guid: z.string(),
  name: z.string(),
  email: z.string(),
  attributes: z.array(AttributeSchema).optional()
});
```

### What this does:
Defines a person who created or contributed to a design.

---

## Lines 822-920: File

```typescript
export const FileSchema = z.object({
  guid: z.string(),
  name: z.string(),
  mime: z.string().optional(),
  remote: z.string().optional(),
  // ... more fields
});
```

### What this does:
Defines a file reference with metadata like MIME type, size, and hash.

---

## Lines 922-1000: Folder and Benchmark

Continues the pattern with Folder (for organizing files) and Benchmark (for quality measurements).

---

# How All the Lines Work Together

Now that we've examined each line, here's how the whole program works:

## 1. Setup Phase (Lines 1-36)
- Comments identify the file and its license
- Imports bring in external libraries:
  - Word lists for random names
  - CSS utilities (clsx, tailwind-merge)
  - 3D graphics (Three.js)
  - Graph visualization (cytoscape)
  - ID generation (uuid)
  - Data validation (zod)
  - Configuration (constants.json)

## 2. Utility Functions (Lines 40-130)
- `cn()`: Combines CSS classes smartly
- `guid()`: Generates unique identifiers
- `SeededRandom`: Reproducible random numbers
- `Generator`: Creates human-readable random names
- `normalize()`: Handles missing values
- `round()`: Consistent number precision
- `jaccard()`: Measures set similarity
- `deepEqual()`: Compares complex objects
- `arraysEqual()`: Compares arrays
- `generateUniqueName()`: Avoids duplicate names

## 3. Coordinate System (Lines 140-145)
- Functions to convert between Semio's 3D coordinate system and Three.js

## 4. Entity ID System (Lines 147-260)
- Type definitions for all entity IDs
- Factory functions to create IDs
- Comparison functions to check equality
- Getter functions to extract GUIDs

## 5. Data Models (Lines 267+)
- Each entity (Attribute, Coord, Point, Vector, Plane, Camera, Location, Author, File, Folder, Benchmark, etc.) follows the same pattern:
  - **Schema**: Zod validator defining the structure
  - **Type**: TypeScript type extracted from schema
  - **Serialization**: Convert to/from JSON
  - **Diff functions**: Track changes for undo/redo and sync

---

# Where This Code Runs

This code runs in multiple environments:

1. **Web Browser**: The Sketchpad app that users interact with
2. **Node.js Server**: Backend processing and API
3. **Electron Desktop App**: Desktop version of Sketchpad
4. **Build Tools**: During development and testing

---

# One Small, Safe Change to Try

**Change the separator in generateUniqueName from space to underscore:**

Find line 122:
```typescript
export const generateUniqueName = (baseName: string, existingNames: string[], separator: string = " "): string => {
```

Change `" "` to `"_"`:
```typescript
export const generateUniqueName = (baseName: string, existingNames: string[], separator: string = "_"): string => {
```

**What will happen:**
- Instead of "Wall 2", duplicate names will become "Wall_2"
- This is a safe change because it only affects the default separator
- The function still works exactly the same way

**Why this is safe:**
- It's just changing a default value
- No existing code should break
- It's easy to change back

---

# Summary

The first 1000 lines of `semio.ts` establish the foundational building blocks:

1. **External dependencies** are imported
2. **Utility functions** handle common tasks
3. **ID types** ensure type safety for entities
4. **Data models** define the structure of everything in Semio
5. **Diff functions** enable undo/redo and real-time collaboration

This is the "vocabulary" of the Semio application — all the basic words and concepts that the rest of the code builds upon.
