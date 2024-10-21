<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/usalu/semio/r24.10-3/logo/logo-horizontal-dark.svg">
        <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/usalu/semio/r24.10-3/logo/logo-horizontal.svg">
        <img alt="semio" href="https://github.com/usalu/semio/" src="https://raw.githubusercontent.com/usalu/semio/r24.10-3/logo/logo-horizontal.svg">
    </picture>
    <br/>
    <a href="https://doi.org/10.5281/zenodo.8419156"><img src="https://raw.githubusercontent.com/usalu/semio/r24.10-3/badges/doizenodo.svg" alt="Cite"></a>
    <a href="https://github.com/usalu/semio/"><img src="https://raw.githubusercontent.com/usalu/semio/r24.10-3/badges/latestrelease.svg" alt="Latest Release"></a>
    <a href="https://choosealicense.com/licenses/agpl-3.0/"><img src="https://raw.githubusercontent.com/usalu/semio/r24.10-3/badges/licenseagplv3.svg" alt="AGPLv3 License"></a>
    <br/>
    <a href="https://www.grasshopper3d.com/"><img src="https://raw.githubusercontent.com/usalu/semio/r24.10-3/badges/uigrasshopper.svg" alt="Grasshopper"></a>
    <a href="https://dynamobim.org/"><img src="https://raw.githubusercontent.com/usalu/semio/r24.10-3/badges/uiplanneddynamo.svg" alt="Dynamo"></a>
    <a href="https://nortikin.github.io/sverchok/"><img src="https://raw.githubusercontent.com/usalu/semio/r24.10-3/badges/uiplannedsverchok.svg" alt="Sverchok"></a>
    <br/>
    <a href="https://www.python.org/"><img src="https://raw.githubusercontent.com/usalu/semio/r24.10-3/badges/poweredbypython.svg" alt="Python"></a>
    <a href="https://learn.microsoft.com/en-us/dotnet/csharp/"><img src="https://raw.githubusercontent.com/usalu/semio/r24.10-3/badges/poweredbycsharp.svg" alt="C#"></a>
    <a href="https://www.typescriptlang.org/"><img src="https://raw.githubusercontent.com/usalu/semio/r24.10-3/badges/poweredbytypescript.svg" alt="Typescript"></a>
    <br/>
    <i>✏️Design kit-of-parts architecture.</i>
</p>
<br/>

You want to ✏️ the next 🏛️🏘️🏢🏭🏫🏨⛪🕌 with 🤖? But 📐🔢🗣️👥🖱️⌨️ takes all your ⌚? Then try to 🧠 the 🧬 and let semio 💉🖥️✒️🖨️🪄🚀.

# 🤖 What is semio?

<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/usalu/semio/r24.10-3/conceptual/analogies-dark.svg">
        <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/usalu/semio/r24.10-3/conceptual/analogies.svg">
        <img alt="Analogies" src="https://raw.githubusercontent.com/usalu/semio/r24.10-3/conceptual/analogies.svg">
    </picture>
</p>

Let me start by what semio is **not**:

- Rhino, SketchUp, 3ds Max, Blender, ...

  A _3d-modeling_ tool 🐚

  > In semio you **link** 3d-models 🎋

- Revit, ArchiCAD, Vectorworks, BricksCAD, BlenderBIM, …

  A BIM-authoring tool 🧱

  > In semio you **link** instances of your own types 🗿

- Grasshopper, Dynamo, GenerativeComponents, Sverchok, …

  A _node-based programming_ tool 🦗

  > In semio you **link** the input with the output ⛓️‍💥

- IFC, Speckle, BHoM, …

  A data tool 📄

  > In semio you **link** design knowledge 📚

- Hypar, Viktor, Fusion, …

  A parametric tool 🎛️

  > In semio you **link** static designs 🩻

- Wasp, Assembler, Monocerous, ComputerGeneratedArchitecture …

  A combinatorical tool 🎰

  > In semio you **link** precise pieces 🪡

- OpenSCAD, CadQuery, Fornjot, …

  A scripting tool 📜

  > In semio you **link** ports visually 🖱️

- Illustrator, AffinityDesigner, InkScape, …

  A _vector graphics_ tool 📐

  > In semio you **link** drawings 🖼️

- EnergyPlus, TRYNSIS, Modellica, …

  A simulation tool ☀️

  > In semio you **link** models and results 🔢

- Autodesk Construction Cloud, Bimcloud, Bentley Infrastructure Cloud, Git, Nextcloud, …

  A CDE tool ☁️

  > In semio you **link** urls 🔗

So, what **is** semio?

- An _open-source_ ecosystem for designing kit-of-parts architecture 🧩
- A _designer-affine_ pencil for a placeholder-based, non-linear and iterative design space exploration 🔀
- A _collaboration-friendly_ platform to design effectively in large teams 🤝
- A _local-first_ storage to link, reuse and develop design systems 🔗
- An _ai-boosted_ design assistant, ready for LLM super powers 🤖
- A _ready-to-use_ API for other developers 👩‍💻

## 🤔 When should I design with semio?

If you design storeys, walls, windows, slabs, roofs
by moving them around, changing individual dimensions of those,
then you should stick to your BIM tool ❌

If you design unique atmospheres
by hand drawn sketches, eager to find the right shape based on surfaces with one-of-each-kind morphology, then you should stick to your 3d tool ❌

If you design high-resolution blobs
by sophisticated rules, statistical noise, intersection-based entropy,
then you should stick to your solver tool ❌

If you design custom typologies
by iteratively detailing types and variants based on metrics,
then you should use semio ✅

# 🚀 Getting started

> The code of semio is compatible with Windows, Mac and Linux but currently compiled for Windows only.

semio has two user-interfaces:

- Grasshopper 🦗
- sketchpad ✏️

> The heart of semio is the `semio-engine` executable which exposes an GraphQL-API. If you are developer, then you should visit `http://localhost:24103/graphql`.

## ⬇️ Installation

> The binaries are not signed (yet). Therefor Windows might give a warning.

### 🦗 Grasshopper

The Grasshopper plugin can be installed over the Rhino Package Manager 🟢

1. Open Rhino Package Manger ⌨️
1. Search for semio 🔎
1. Install the latest version ⬇️
1. Restart Rhino 🔄

![Rhino Package](https://raw.githubusercontent.com/usalu/semio/r24.10-3/dotnet/Semio.Grasshopper/docs/rhinopackage.gif)

That's it!

### ✏️ sketchpad

> This is a prototype. It doesn't use the latest versions of the engine and Grasshopper.

[sketchpad](https://github.com/usalu/semio/releases/download/r24.07-1/sketchpad.exe) is a portable program and needs no installation. To use sketchpad with Grasshopper you need to download semio Grasshopper `2.1.2`. If you want to use sketchpad standalone you need to download, extract and start the [engine](https://github.com/usalu/semio/releases/download/r24.07-1/engine.zip). After this you can open the [examples](https://github.com/usalu/semio/releases/download/r24.07-1/examples.zip)🏎️

## 🏘️ Examples

You can download the latest [examples](https://github.com/usalu/semio/releases/download/r24.10-3/examples.zip) or take a look [here](https://github.com/usalu/semio/blob/r24.10-3/examples/metabolism/README.md) 👀

# ❤️Contribution

New platforms wait to be reached, bugs wait to be found, examples wait to be created, documentation waits to be written, ...
Feel free to open a [discussion](https://github.com/usalu/semio/discussions), an [issue](https://github.com/usalu/semio/issues), take a look under the [project site](https://github.com/users/usalu/projects/2) or just write me an [e-mail](semio-community@posteo.org) 👋
