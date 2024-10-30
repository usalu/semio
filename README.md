<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/usalu/semio/main-tag/logo/semio-horizontal-dark.svg">
        <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/usalu/semio/main-tag/logo/semio-horizontal.svg">
        <img alt="semio" href="https://github.com/usalu/semio/" src="https://raw.githubusercontent.com/usalu/semio/main-tag/logo/semio-horizontal.svg">
    </picture>
    <br/>
    <a href="https://doi.org/10.5281/zenodo.8419156"><img src="https://raw.githubusercontent.com/usalu/semio/main-tag/badges/doi-zenodo.svg" alt="Cite"></a>
    <a href="https://github.com/usalu/semio/"><img src="https://raw.githubusercontent.com/usalu/semio/main-tag/badges/latest-release.svg" alt="Latest Release"></a>
    <a href="https://choosealicense.com/licenses/agpl-3.0/"><img src="https://raw.githubusercontent.com/usalu/semio/main-tag/badges/license-agpl-v3.svg" alt="AGPLv3 License"></a>
    <br/>
    <a href="https://www.grasshopper3d.com/"><img src="https://raw.githubusercontent.com/usalu/semio/main-tag/badges/ui-grasshopper.svg" alt="Grasshopper"></a>
    <a href="https://www.microsoft.com/windows/windows-11"><img src="https://raw.githubusercontent.com/usalu/semio/main-tag/badges/ui-windows.svg" alt="Windows"></a>
    <a href="https://apple.com/macos"><img src="https://raw.githubusercontent.com/usalu/semio/main-tag/badges/ui-macos.svg" alt="macOS"></a>
    <br/>
    <a href="https://www.python.org/"><img src="https://raw.githubusercontent.com/usalu/semio/main-tag/badges/powered-by-python.svg" alt="Python"></a>
    <a href="https://learn.microsoft.com/en-us/dotnet/csharp/"><img src="https://raw.githubusercontent.com/usalu/semio/main-tag/badges/powered-by-csharp.svg" alt="C#"></a>
    <a href="https://www.typescriptlang.org/"><img src="https://raw.githubusercontent.com/usalu/semio/main-tag/badges/powered-by-typescript.svg" alt="Typescript"></a>
    <br/>
    <i>✏️ Design-Information-Modeling for Kit-of-Parts Architecture.</i>
</p>
<br/>

You want to ✏️ the next 🏛️🏘️🏢🏭🏫🏨⛪🕌 with 🤖? But 📐🔢🗣️👥🖱️⌨️ takes all your ⌚? Then try to 🧠 the 🧬 and let semio 💉🖥️✒️🖨️🪄🚀.

# 🤖 What is semio?

Let me start by what semio is **not**:

- Rhino, SketchUp, 3ds Max, Blender, ...

  A _3d_ tool 🐚

  > In semio you **link** 3d-models 🎋

- AutoCAD, DraftSight, LibreCAD, ...

  A _CAD_ tool 📐

  > In semio you **link** dynamic blocks 📦

- Revit, ArchiCAD, Vectorworks, BricksCAD, BlenderBIM, …

  A _BIM_ tool 🧱

  > In semio you **link** instances of your own types 🗿

- Grasshopper, Dynamo, GenerativeComponents, Sverchok, …

  A _node_ tool 🦗

  > In semio you **link** the input with the output ⛓️‍💥

- IFC, Speckle, BHoM, …

  A _data_ tool 📄

  > In semio you **link** design knowledge 📚

- Hypar, Viktor, Fusion, …

  A _parametric_ tool 🎛️

  > In semio you **link** static designs 🩻

- Wasp, Assembler, Monocerous, ComputerGeneratedArchitecture …

  A _solver_ tool 🎰

  > In semio you **link** precise pieces 🪡

- OpenSCAD, CadQuery, Fornjot, …

  A _scripting_ tool 📜

  > In semio you **link** ports visually 🖱️

- Illustrator, AffinityDesigner, InkScape, …

  A _vector_ tool ↗️

  > In semio you **link** drawings 🖼️

- EnergyPlus, TRYNSIS, Modellica, …

  A _simulation_ tool ☀️

  > In semio you **link** models and results 🔢

- Autodesk Construction Cloud, Bimcloud, Bentley Infrastructure Cloud, Git, Nextcloud, …

  A _CDE_ tool ☁️

  > In semio you **link** urls 🔗

So, what **is** semio?

- An _open-source_ ecosystem for designing kit-of-parts architecture 🧩
- A _designer-affine_ pencil for a placeholder-based, non-linear and iterative design space exploration 🔀
- A _collaboration-friendly_ platform to design effectively in large teams 🤝
- A _local-first_ storage to link, reuse and develop design systems 🔗
- An _ai-boosted_ design assistant, ready for LLM super powers 🤖
- A _ready-to-use_ API for other developers 👩‍💻

## 🤔 When should I design with semio?

If you design unique atmospheres
by hand drawn sketches, eager to find the right shape based on surfaces with one-of-each-kind morphology, then you should stick to your 3d tool ❌

If you design floor plans, sections and elevations
by moving lines and hatches around, analyzing contours
then you should stick to your CAD tool ❌

If you design storeys, walls, windows, slabs and roofs
by moving them around, changing individual dimensions of those,
then you should stick to your BIM tool ❌

If you design high-resolution blobs
by sophisticated rules, statistical noise, intersection-based entropy,
then you should stick to your solver tool ❌

If you design modular systems
by iteratively developing typology based on metrics,
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

![Rhino Package](https://raw.githubusercontent.com/usalu/semio/main-tag/dotnet/Semio.Grasshopper/docs/rhinopackage.gif)

That's it!

### ✏️ sketchpad

> This is a prototype. It doesn't use the latest versions of the engine and Grasshopper.

[sketchpad](https://github.com/usalu/semio/releases/download/r24.07-1/sketchpad.exe) is a portable program and needs no installation. To use sketchpad with Grasshopper you need to download semio Grasshopper `2.1.2`. If you want to use sketchpad standalone you need to download, extract and start the [engine](https://github.com/usalu/semio/releases/download/r24.07-1/engine.zip). After this you can open the [examples](https://github.com/usalu/semio/releases/download/r24.07-1/examples.zip)🏎️

## 🏘️ Examples

You can download the latest [examples](https://github.com/usalu/semio/releases/download/r24.10-3/examples.zip) or take a look [here](https://github.com/usalu/semio/blob/main-tag/examples/metabolism/README.md) 👀

# ❤️ Contribution

New platforms wait to be reached, bugs wait to be found, examples wait to be created, documentation waits to be written, ...
Feel free to open a [discussion](https://github.com/usalu/semio/discussions), an [issue](https://github.com/usalu/semio/issues), take a look under the [project site](https://github.com/users/usalu/projects/2) or just write me an [e-mail](semio-community@posteo.org) 👋

# 📊 Stats

<p align="center">
    <a href="https://github.com/usalu/semio"><img src="https://hits.seeyoufarm.com/api/count/incr/badge.svg?url=https%3A%2F%2Fgithub.com%2Fusalu%2Fsemio&count_bg=%23FF344F&title_bg=%23555555&icon=&icon_color=%23E7E7E7&title=visits&edge_flat=true"/></a>
</p>
