# 🎓 Kit App Tutorial: Managing the Building Sets

Welcome to the team! This document is designed to walk you through [js/semio/sketchpad/Kit.tsx](js/semio/sketchpad/Kit.tsx). Think of this as your "User Manual" for the code that manages our digital building sets.

---

## 🌎 Big Picture Overview

### What is this file?
This file is the **Brain** for the **Kit Application**. In our world, a **Kit** is like a "LEGO Set" or a "Toolbox." It contains blueprints (called **Types**) and examples of things built with those blueprints (called **Designs**). 

### Why does it exist?
When a user is looking at their list of building parts, sorting them, or deciding which one to edit, this file is doing all the heavy lifting in the background. It remembers what the user clicked on, how they want the list sorted, and how the "Relationship Map" (the Diagram) should look.

### What problem does it solve?
Imagine trying to build a complex skyscraper with thousands of parts but no way to organize them. This file provides the organization. It ensures that when you select a "Window" type, the system knows exactly which one you mean, and if you change its name, that change is tracked so you can "Undo" it if you make a mistake.

### Where does it sit?
It lives inside the **Sketchpad**, which is our main visual workspace. It acts as one of several "Apps" (like Home, Design, or Type) that a user can switch between.

---

## 🏗️ Architecture & Relationships

### How it connects
This file is like a **middleman** in a three-way conversation:
1.  **The User (UI):** When a user clicks a button, the UI tells this file.
2.  **The Notepad (Y.js):** We use a technology called **Y.js** which acts like a "Shared Google Doc." Multiple people can edit the same Kit at once, and this file makes sure everyone's "Notepad" stays in sync.
3.  **The Rules (XState):** We use a **State Machine** (XState). Think of this as a set of strict rules (e.g., "You can't delete a part while you are still creating it").

### Data Flow
1.  **Input:** A User clicks a row in a table.
2.  **Process:** This file receives that click, updates its "Memory" (State), and calculates if anything else needs to change.
3.  **Output:** It sends the updated data back to the screen so the row turns blue (Selected).

---

## 📋 File Responsibilities

*   **State Management:** Keeping track of "Memory" (Selection, Hover, Filters).
*   **Command Handling:** Executing specific tasks like "Create a new Type" or "Delete this Design."
*   **Undo/Redo Logic:** Recording every change so the user can go back in time.
*   **Relationship Tracking:** Figuring out which parts belong to which "Family" (Design Families).
*   **Visual Organization:** Managing how windows are laid out on the screen.

---

## 📦 Imports & Dependencies Explained

*   **React:** The library we use to build the visual parts of the website.
*   **Y.js (Y):** The "Shared Notepad" technology that allows for live collaboration.
*   **XState:** The "Rulebook" that manages what states the app can be in.
*   **Lucide (Icons):** A library of small pictures (like a Trash Can or a Plus Sign).
*   **semio.ts:** Our "Core Library" which contains the basic definitions of what a "Piece" or "Connection" is.

---

## 🚶 Step-by-Step Code Walkthrough

### 1. Design Family Helpers
*   **What it is:** A set of functions like `getDesignFamilyGuids`.
*   **Why it exists:** Designs can have "Parents" and "Children." This code is like a "Genealogist" that finds all the relatives of a specific design.
*   **In Action:** If you delete a "Parent" design, this code helps find all its "Children" so the user can decide what to do with them.

### 2. Internal State Management
This section defines the "Shapes" of our memory.
*   **`KitAppSelection`**: A box that holds the IDs of everything the user has clicked on.
*   **`KitAppHover`**: A box that remembers what the user's mouse is currently pointing at.
*   **`KitAppWindowKind`**: An "Enum" (a list of choices) defining if the user is looking at a **Table** (list) or a **Diagram** (map).

### 3. The KitStore (The Manager)
*   **What it is:** A "Class" (a blueprint for an object) that acts as the main manager.
*   **Why it exists:** It's the central hub for all Kit data. It handles "Transactions" (batches of changes).
*   **In Action:** When you rename a part, the `KitStore` starts a "Transaction," changes the name, creates an "Undo" version, and then "Commits" (saves) the change.

### 4. The App Plugin (`kitAppPlugin`)
*   **What it is:** A configuration object that tells the main Sketchpad "Hey, I'm here, and here are my rules!"
*   **Why it exists:** It allows us to keep the Kit App's code separate from the main system while still letting them talk to each other.

### 5. Action Hooks (The Pipes)
*   **What it is:** Functions starting with `use...` (like `useKitAppSelectType`).
*   **Why it exists:** In React, "Hooks" are how we connect the visual components to the data. 
*   **In Action:** A "Button" on the screen uses the `useKitAppSelectType` hook. When the button is clicked, it calls the function provided by the hook to tell the "Brain" to select that type.

---

## 🔄 Execution Flow

1.  **Initialization:** The app starts. The `kitAppPlugin` tells the system to create a new "Memory box" (State) with empty selections and default sorting.
2.  **User Interaction:** The user hovers over a list of files. 
3.  **Event Dispatch:** The `useKitAppSetHover` hook sends a message to the "State Machine."
4.  **State Update:** The Machine checks if this is allowed, then updates the "Hovered" box in memory.
5.  **Re-render:** React notices the memory changed and redraws the screen, perhaps highlighting the file in gray.

---

## 📖 Glossary of Terms

*   **API (Application Programming Interface):** A standardized way for different parts of our program to talk to each other.
*   **Artifact:** A general word we use for "anything inside a Kit" (like a Type, a Design, or a File).
*   **GUID (Globally Unique Identifier):** A long string of random letters and numbers (like `abc-123-xyz`) used to give every single thing in our system its own unique handle.
*   **State:** The "Current Memory" of the app (e.g., "The user is currently looking at the Table view").
*   **Hook:** A special React function that lets a visual component "hook into" the app's data or logic.
*   **Transaction:** A way of grouping many changes together so they all happen at once, or not at all (and so they can be undone together).

---

## ✨ Final Summary

In short, **Kit.tsx** is the responsible adult in the room for the Kit App. It manages the data, enforces the rules, keeps everyone in sync during collaboration, and provides the "Pipes" (Hooks) that the visual interface uses to display information to the user. Without this file, the Kit UI would be a pretty picture that doesn't actually **do** anything.
