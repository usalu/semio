---
title: Model Brick Set
description: Define the types (brick molds) and their variants.
---

## **📁 Working Smarter: Loading Geometry from Files in semio ( The Placeholder )**

In this chapter, we explore why referencing geometry from **external files**—rather than live links from within a single Grasshopper script—can be a powerful and more efficient way to work. This approach transforms how teams collaborate, how data is stored, and how modular systems are designed in semio.

---

### **💡 Why Use File-Based Geometry?**

semio introduces a powerful shift in computational design by encouraging **loading geometry from baked, static files (whether a baked Grasshopper solution or a typical 3D modelled Geometry)**

This method that may feel counterintuitive at first, but unlocks surprising benefits.

Let’s unpack what this approach offers.

---

### **🔀 1\. Modular Collaboration Through Chunks**

In traditional Grasshopper workflows, it's common to build massive, tangled scripts that try to manage an entire project in one go. This quickly becomes hard to debug, scale, or share.

semio changes this by letting you **split your design into smaller, reusable “chunks.”** Each chunk can be built in its own Grasshopper file, then exported and referenced through semio using file-based geometry.

#### **✨ Key Benefits:**

- **True team collaboration:** Multiple people can work on different parts of the same design simultaneously.

- **Reusability:** Each chunk becomes a standalone module, reusable across projects.

- **Faster iteration:** You can isolate issues or update a part without breaking the whole system.

🧱 _semio makes this possible through its concept of placeholders, connections, and logic—not raw geometry—so different teams can focus on their pieces independently._

---

### **🌍 2\. Platform-Independent Design Sharing**

One of the biggest headaches in computational design is managing plugin versions, Grasshopper dependencies, or opening files across systems.

By **exporting geometry to files and referencing it in semio**, you unlock a platform-independent workflow.

#### **✨ Key Benefits:**

- **Non-Grasshopper users can collaborate:** Rhino-only users can now access and place designs built by Grasshopper users.

- **No plugin chaos:** You don’t need to ensure everyone has the same Grasshopper or plugin versions installed.

- **Cross-software compatibility:** You can bring work from different sources—Grasshopper, Rhino, semio, or even other platforms—into a unified **design Blueprint**.

🧠 _semio acts as the glue between systems, allowing modular contributions from different tools to merge into one coherent structure._

---

### **⚠️ A Note on Dynamic vs. Static Geometry**

It’s important to acknowledge the tradeoff:  
 When you **bake and export** your geometry to a file, you lose some of the dynamic flexibility Grasshopper offers.

But in a modular design system, this is often a **feature, not a flaw**.

Instead of endlessly tweaking everything in real time, you work with **predefined, controlled parts** that are meant to connect, not endlessly morph.

This leads to:

- More stable design logic

- Predictable behavior

- Cleaner collaboration

📌 _We highly recommend baking the necessary geometry and saving it to a folder before bringing it into semio._

---

### **🔄 Alternative: Direct Dynamic Linking**

That said, semio **does still support direct connections** to Grasshopper outputs without saving to a file.

This can be useful when:

- You need live updates from a flexible system

- You're prototyping with lots of parameter changes

However, this limits collaboration and breaks modularity—because you’re back to working in a **single, shared script**.

A useful middle ground:  
 ✅ Do both in one script—use Semio’s import/export logic to store interim baked files, while keeping your live script flexible.

---

🛠️ **Platforms & Workflow in semio**

Designing in semio is split into two main stages, each supported by a dedicated platform but flexible enough to be done entirely in **Grasshopper**.

---

🧱 **1\. Generating the Kit**

This is where you define your modular parts. Think of it like designing the Lego set:

You model **Types** (the brick templates), place **Ports** (connectors), and attach **Metadata** (semantic and design-relevant information).

✅ **Tool:**

🦗 **semio Plugin for Grasshopper**  
The plugin gives you precise control over your workflow and lets you take full advantage of Grasshopper’s parametric logic

---

🧩 **2\. Creating the Design**

This is where you snap those parts together.

You place **Pieces** (instances of Types) and define **Connections** (rules between Ports). like assembling bricks into a model.

✅ **Tools:**

🌐 **semio Studio (Online):**  
 A visual, intuitive interface for assembling Designs collaboratively. Ideal for layout, teamwork, and quick iteration.

🦗 **Grasshopper Plugin:**  
 You can also use the semio plugin for the entire process, especially when you want to leverage parametric workflows or integrate with plugins like **Wasp** or **Ladybug**, whether to generate connection logic, analyze a Design, or enrich Metadata.

---

### **🔄 Combine Both Platforms**

semio is **platform-flexible**: you can **generate Kits in Grasshopper** and **assemble Designs in Studio**, or do it all in one place.

Everything stays **linked and synced**. Kits and Designs can be reused, shared, and edited across both tools seamlessly.

---

# **🧩 Building Your First semio Design**

**From Sketch to Structure**

In this tutorial, you’ll walk through the process of turning a sketch into a working modular design using semio. Think of this as assembling your first LEGO model from scratch — except now, you’re the one defining the bricks.

We’ll structure each step around Semio’s core components: **Type**, **Piece**, **Port**, and **Connection**, all of which you already encountered in The Anatomy of a Modular Design System ( Think in semio )

---

### **👀 Step 0: Understand Your Sketch**

Your sketch is split into two parts — just like a Lego instruction sheet:

#### **🔲 Top: Designing the Building Blocks**

This is the starting point of any modular design — **spotting patterns**.  
 You can spot five shapes that look almost the same, just with different sizes.

🧱 Think of them like Lego bricks: a 2-stud brick, a 4-stud brick, and a 5-stud brick.  
 Similar shape, same logic — just stretched.

That means:  
**We will have to model them as _variants_ of the same Type** (the same mold with different settings).  
 If there were different shapes — like a window or roof piece — those would count as _different_ Types.

The shape you see is a rectangle, cut at an angle — that’s your custom building block.  
 On the right, the yellow box lists the three repeated **variants** (2, 4, and 5 units long).

---

#### **🔳 Bottom: Building the Design**

This is where you see how those bricks snap together to form your structure.

- Each number (2, 4, 5) shows **which variant** is used where.

- 🔴 **Circles with arrows** = _Ports_ — the snap points where pieces connect

- 🖋️ **Black text** = _Assembly instructions_ — telling you which Port connects to which, and how

We’ll explain all these parts as we move step-by-step through building the design.

---

## **Kit Creation >**

### **🧱 Step 1: Modeling the Types (Brick Molds)**

As mentioned earlier in **Thinking in semio**, a **Type** is like a **brick mold**.  
 You're not just drawing geometry — you're designing a **template** that combines shape with meaningful design information.

#### **🧩 What are the “Molds” in this example?**

Each of the five elements in your sketch is a **variant** of the **same Type** —  
 meaning they all share the same shape logic but vary in size.

So while you see five Pieces in the final Design, they’re made up of **three unique shapes**,  
 with two of them repeated once — making **three Variants of one Type**.

#### **🔧 What do we need to create these Molds?**

To define a Type in semio, you’ll model both its **form** and its **behavior**.  
 Let’s go over the essential ingredients:

- **🏷️ Name** → Every Type needs a unique name that helps you identify and reuse it later.

- **🏷️🧱 Representation** → This is the geometry linked to the Type. Think of it as the visual shape of your brick.

- **🧲 Ports** → These are the snap points. You’ll define exactly where the brick connects to others.

In the next steps, we’ll walk through how to set up each of these elements to build a complete, reusable mold — just like designing a custom Lego piece.

### **🏷️ Naming the Type**

#### **📚 General: Why Naming Matters**

Giving clear and consistent names to elements in semio **pays off later** — especially during the design and connection phases.  
 When your project grows and you’re working with multiple Types, Variants, and Pieces, meaningful names help you:

- stay organized

- avoid confusion

- quickly identify components

- enable smoother teamwork and collaboration

##

### **🧱 Referencing Geometry to the Type (Modeling Representation)**

In semio, geometry is more than just shapes — it’s tied to **meaning** 💡  
 This is different from regular Grasshopper workflows, where geometry is just raw data.

In semio, you're building your own **design language** — like sketching with intent ✏️  
 You don’t say “connect point (x, y, z) to Brep edge 23.”  
 You say “connect the door to the living room” —  
 or in Lego terms: “snap the window brick to the top of the wall” 🪟🧱

🔗 The geometry is still there — but now it’s **semantic**.  
 You’re connecting **components with purpose**, not just lines or meshes.

We call this step **Modeling a Representation** 🧩 —  
 it’s where you attach a visual form to your brick mold (Type),  
 so that every **beam**, **panel**, or **profile** knows what it is and where it fits ✅

---

### **🧊 How to Attach Geometry**

To model a Representation, you first need to create the **geometry** — the body of your building block.

semio lets you attach geometry to a Type in **two main ways**, each with its own strengths 🚀

{/_ TODO: Insert Table _/}

#### **1️⃣ Direct Reference**

Model or script the geometry **directly** in Rhino or Grasshopper 🦗🦏  
 ✅ Fast and flexible for prototyping  
 ⚠️ But tightly coupled to your current file — harder to reuse or collaborate on

#### **2️⃣ File-Based Reference (🔁 Recommended)**

Reference geometry from an **external file** 📁  
 ✅ Modular and clean  
 ✅ Easy to update without breaking connections  
 ✅ Perfect for large teams and scalable design workflows 🤝

---

💡 **Tip:** Choose the method that fits your workflow best.  
 Want to dive deeper into the pros and cons? Learn more here 🔍

We will now discover a hands on of doing both on our example :

Lets try to model our shapes as you see is a rectangle, cut at an angle

As you can see the different design Parameters in the sketch :

- `W` = a single Unit Dimensions  
  `n` = Number of segments (length of each element

In Grasshopper :

For now we will develop This Shape a logic has been created to generate it based on how many Units it has in its length.

We have created a Grashopper logic that first generate this Rectangular shape cut and then Create a wood profile out of this curve

In Rhino

We could model the Geometry we want to attach at the end we reference a layer

File Linking files :

Her link to a Seperate page > Switching between different representations. Placeholder logic

Two Examples > Three different snapshots of iterative process. First when its still a box and then add Capsules windows and other detailing. Showcase how easy is it possible to iteratively Design and come back to the whole system switch between

Two main things should come out

1.Iterative process

2.Switching between versions

Just like in LEGO, you're making a mold that you’ll reuse — not the bricks themselves.

---

### **⚓ Adding Ports to the Type (Snapping Points)**

Once you've modeled the visual look of your brick mold (Representation), it’s time to decide **where** these bricks connect to each other.  
 That’s where **Ports** come in — the snap points that define where a Brick can attach 🧲

### **🧩 What Do You Need to Define a Port?**

In the semio Grasshopper plugin, each Port is defined with 3 main inputs:

- **A Point** 📍  
   This is the location where the connection happens.  
   Think of it as the exact stud or socket on your Lego brick. You choose this point carefully on the surface of your geometry.

- **A Vector** ➡️  
   This defines the direction the Port faces.  
   It’s like saying: "This side is meant to connect outward."  
   The vector is especially important later when aligning pieces — because semio uses this direction to orient other parts when snapping.

- **A Port ID** 🏷️  
   This is the name of the Port — a label you'll use when connecting components.  
   For example: `"n"` (north), `"bottom"`, or `"hingePoint"`.

### **🏷️ About Port IDs**

A **Port ID** is the label you give to each Port.  
 This naming is important — because later, when you make connections, you’ll say things like:

“Connect Piece A’s `top` Port to Piece B’s `bottom` Port.”

You can choose any naming system — as long as it makes sense in your design logic.  
 In this example, we use:

- `n`, `s`, `e`, `w` — standing for **north, south, east, and west** 🧭

But in your own projects, you might use:

- `top`, `bottom`, `hinge`, `plug`, `outlet`, `windowDock` — or anything else 🔧

💡 **Tip**: Choose a consistent and meaningful naming system. It will make connecting components easier — and later on, it helps when working with AI or team members.

### **🔎 Why Are Ports in the Middle in our example?**

You might wonder:

“Why aren’t the Ports placed exactly where the pieces touch?”

That’s because semio lets you **adjust** the position and rotation of a Piece **after** it snaps.  
 So you can define Ports in logical, consistent places — like the center of a face — and still have full control over placement later.

This is especially helpful when modeling **Variants of the same Type**:  
 Keeping Port positions uniform means you can reuse logic more easily — even if the pieces are longer, shorter, or slightly different in shape.

### **🧠 How Many Ports Should I Add?**

That’s up to you. Just like Lego bricks have multiple studs — even if not all of them are used — you can place multiple Ports to offer flexibility 🧱

But in semio, you decide **how much flexibility** you need:

- You can define only one Port per side 🔒

- Or add many, for more options 🔓

There’s no “right” answer — it depends on your design logic.

Here more Grasshopper

### **🧱 Putting It All Together: Modeling the Type**

After modeling all the parts of your brick mold —  
 **Name**, **Variant**, **Ports**, and **Representation** —  
 you plug them into the `Typ` component.

- 🏷️ **Name** → e.g. “Profile”

- 🔢 **Variant** → e.g. 2

- 🧲 **Ports** → e.g. n, s, e, w

- 🧊 **Representation** → the linked geometry

🎯 The result is a complete **Type** —  
 a smart, reusable building block ready to be placed and connected in your Design.

Think of it as snapping all the mold info together — now you have your custom LEGO brick 🔧🧱

---

## **🧪 Step 2: Create Variants of the Same Type**

Now that you’ve created your base **Type**, it’s time to generate **Variants** — based on the **unit count `n`** shown in the sketch.

Each Variant uses the **same mold**, just stretched or scaled.  
 Think of it like longer or shorter Lego bricks of the same kind 🧱🧱🧱  
 Since they follow the same design logic, we treat them as **Variants** of one **Type**, not separate Types.

### **🔢 From the Sketch:**

- **Variant 2** → 2 units long

- **Variant 4** → 4 units long

- **Variant 5** → 5 units long

---

### **🔁 How to Model a Variant in semio**

To model a Variant, you follow **the same steps** as creating a Type —  
 but with **two key differences**:

1. **Reuse** the original **Type name**

2. **Assign** a **unique Variant name** (e.g. the unit count `n`)

This tells semio:  
 ➡️ “Same mold, different size.”

✅ Just like with Types, you have **three ways** to reference geometry:

- Direct from Grasshopper or Rhino Layer

- External file (recommended)

---

### **💡 Advanced Tip: Use Clusters to Generate Variants**

In Grasshopper, a smart way to manage **Variants** is by using a **Cluster** —  
 essentially a reusable mold-maker that builds each version of your LEGO brick 🧱

Since Variants are mostly identical and only differ by one or two inputs,  
 the Cluster helps you model everything once and repeat only what’s necessary.

**Inside the Cluster:**  
 You define the full Type logic —

- Geometry generation
- Representation attachment (Model Representation)
- Port placement (Model Port)
- Type creation (Model Type)

**Outside the Cluster:**  
 You feed in only what changes:

- `n` → the unit count (e.g. 2, 4, 5), which controls the shape size 📏

- Variant name → In this Case it is also `n` 🏷️

This way, you can generate multiple Variants from the same Type mold —  
 without duplicating anything or cluttering your canvas.

✅ **Bonus:** Keeps your file organized and scalable — perfect for growing your Kit later on.

Showcase the three different methods :

Cluster without referencing

Cluster with referencing Files

Note > If the Shape would be different you would create a new type instead of Variants of the same type

Example ?

---
