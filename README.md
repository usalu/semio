> 👀 Do you want to develop the next design AI with us? Then join our Weekly-Evening-Session every Tuesday starting at 6pm (UTC+1) on [Discord](https://discord.gg/m6nnf6pQRc) 🍻

<p align="center">
    <a href="https://docs.semio-tech.com">
      <picture>
          <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/usalu/semio/main-tag/logo/semio-horizontal-dark.svg">
          <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/usalu/semio/main-tag/logo/semio-horizontal.svg">
          <img alt="semio" href="https://github.com/usalu/semio/" src="https://raw.githubusercontent.com/usalu/semio/main-tag/logo/semio-horizontal.svg">
      </picture>
    </a>
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
    <i>🧩 Design-Information-Modeling for Kit-of-Parts Architecture 🏘️ </i>
</p>
<br/>

You want to 🧩 the next 🏛️🏘️🏢🏭🏫🏨⛪🕌 with 🤖? But 📐🔢🗣️👥🖱️⌨️ takes all your ⌚? Then try to 🧠 the 🧬 and let semio 💉🖥️✒️🖨️🪄🚀.

# 👋 Hello dev

> [!NOTE]
> Are you a user of semio? Then you probably want to visit our [docs](https://docs.semio-tech.com) 👀

Glad to see you!

Let me walk you throw 🚶

# ⚖️ Principles

Let's start with the rule of thumbs that this codebase was built with 🫰

## 💾 If something can be written in a single file, then it probably should ✅

I know, the urge to **tidy** up or **separate** things is big 🗃️

But try to withstand it 🫥

Out of my experience, it makes development slower, not faster 🐌

A single file is easier for humans and computers to understand 💡

You will be supprised

- by the awesome fill-in-the-middle suggestions of your copilot 🤖
- by the hassle-free context selection for your ai agent 🖱️
- by the smooth refactor experience by just going top-to-bottom ⬇️
- by the beautiful diff for your next code review 🔍
- by the clean git-history when you try to find a certain change 🔁

## 📁 If a folder doesn't make your life dramatically easier, don't create it ❌

We all know this `./src/**` folder that has made it into a lot of starters 🚀

Other than feeling cool about using hacky abbreviations, does it really help you to understand the project faster and work more efficient on it?

If your project contains hundreds of config file and other project folders at the root, maybe 🤔

But most likely not ❌

## 📑 If multiple people work longterm on the same part, then one file for each part should be created ⚙️

Trust me, it will make collaboration much easier 🔀

## 📦 If you don't need an interface because something is not likely to be extended in the future, don't create it ❌

The main question is the interface productive or not?

The pay-off of abstraction happens in the future 🛣️

Every extension profits from a clean interface 🚀

Most things are not extended 🪨

If you change your architecture, just design proper interfaces for something concrete not something potential and reactor it ✍️

## 🤏 Repeating code is ok if it probably doesn't happen more than twice and the repeated code is close in the source code ✅

We are past the time where we copy code for no reason 📃

Actually repeated code can improve the quality of your copilots suggestion 🤯

The main question is how can your application grow?

If a change requires exponentially more duplication then you'll probably have to fix it 🛠️

If not, then you are probably good 👌

## 🤨 Wait, no high-level advice and only plain numbers, files, folders or close line of codes?

In my understanding, rule-of-thumbs are most useful when they are concrete 🔨

Besides that I am sure you know about **KISS** (Keep-It-Simple-Stupid), **DRY** (Dont-Repeat-Yourself), **YAGNI** (You-Aren't-Gonna-Need-It), **SoC** (Seperation-of-Concerns), **Avoid Premature Optimization**, **Law of Demeter**, **LCHC** (Low-Coupling-High-Cohesion), **SOLID** (Single Responsibility (**SR**), Open/Closed (**OC**), Liskov’s Substitution (**LS**), Interface Segregation (**IS**), Dependency Inversion (**DI**)), …

But as always, the devil is in the details 😈

Even if 95% of the codebase follows those principles, there are good reasons for the other 5% ⚖️

## 🚩 Don't worry, you'll figure out the possiblites and make the right choice for the specific problems ✅

# 🦑 [Monorepo](https://github.com/usalu/semio.git)

This git repo has **everything** that exists in the open semio ecosystem 🤯

# 📦 Components

> Do you wonder how the same looking ui or functionality is avalailable on multiple components? The secret is that they have shared cores in their [ecosystem](#-ecosystems) 🥜

A component is a piece of software which runs independently 🏝️

## ✏️ [sketchpad](https://github.com/usalu/semio/tree/main/js/sketchpad)

An electron-based desktop app primarly working for with local kits 💾

## 🦗 [Grasshopper](https://github.com/usalu/semio/tree/main/dotnet/Semio.Grasshopper)

A full-blown [Grasshopper Plugin](https://developer.rhino3d.com/en/guides/grasshopper/) that has (almost) everything 💯

## ⚙️ [engine](https://github.com/usalu/semio/tree/main/python/engine)

A hidden fat-client which exposes shared functionality to other desktop uis 🤝

It takes care of:

- CRUDs (Create-Read-Update-Delete) for local kits 💾
- Client-Server communication ↔️

It offers two APIs to other clients:

- A simple REST OpenAPI 🥇
- A complex GraphQL Relay API 🥈

### {} [REST OpenAPI](https://github.com/usalu/semio/tree/main/python/engine/engine.py#L5529)

If you go to `http://127.0.0.1:2412/api/docs/` you find the Swagger UI:

![GraphQL Query](https://raw.githubusercontent.com/usalu/semio/main-tag/openapi/docs/swagger.png)

### ⭕ [GraphQL Relay](https://github.com/usalu/semio/tree/main/python/engine/engine.py#L5095)

> Still a prototype ✏️

If you go to `http://127.0.0.1:2412/graphql/` you find the GraphiQL UI:

![GraphQL Query](https://raw.githubusercontent.com/usalu/semio/main-tag/graphql/docs/graphiql.png)

## 📚 [docs](https://github.com/usalu/semio/tree/main/js/docs)

## 🛍️ [assets](https://github.com/usalu/semio/tree/main/assets)

# 🏘️ [Examples](https://github.com/usalu/semio/tree/main/examples)

## 🚀 [Starter](https://github.com/usalu/semio/tree/main/examples/starter)

## 👋 [Hello semio](https://github.com/usalu/hello-semio)

## 🌈 [Geometry](https://github.com/usalu/geometry)

## 🫀 [Metabolism](https://github.com/usalu/metabolism)

# 💿 Ecosystems

You might have noticed that the individual components can be closely related such as [sketchpad](#️-sketchpad), [Grasshopper](#-grasshopper) and [engine](#️-engine) but they are in totaly different folders 📂

The reason for this is that the monorepo is not disected according content but according technology stack ✂️

This is less intuitive but more tool-friendly and everything that is easier for our tools is less pain to develop 🧑‍💻

## 🐍 [Python]()

Currently only [engine](#️-engine) but in the future it might grow and then the `.venv` will be centralized, …

## 💻 Building from source

You need the following tools:

- Windows
- [Visual Studio 2022 Community](https://visualstudio.microsoft.com/de/thank-you-downloading-visual-studio/?sku=Community&channel=Release&version=VS2022)
- [Rhino 8](https://www.rhino3d.com/download/rhino-for-windows/8/latest/)
- Python 3.12
- Poetry
- [Node](https://nodejs.org/dist/v22.14.0/node-v22.14.0-x64.msi)

If you do not have Python installed, I recommend to install it over the [Microsoft Store](<(https://www.microsoft.com/store/productId/9NCVDN91XZQP?ocid=pdpshare)>) 🏪

Afterwards you can install poetry with this Powershell command:

```powershell
(Invoke-WebRequest -Uri https://install.python-poetry.org -UseBasicParsing).Content | python -
```

In the console you will see a warning that the `poetry.exe` is not installed in the requested location 📁
![Actual Location](https://raw.githubusercontent.com/usalu/semio/main-tag/poetry/python_ms-store_location.png)
Then copy the actual path `...\AppData\Local\Packages\PythonSoftwareFoundation...\Roaming\pypoetry\venv\Scripts` and add it to your environmental path variable ➕

Then you can `build.ps1` in the Powershell and add your full path `LOCAL_PATH\dotnet\Semio.Grasshopper\Debug\net48` to your GrasshopperDeveloperSettings ⚙️

If you have never executed local Powershell before then you have to first [Set-ExecutionPolicy](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.security/set-executionpolicy) ⚠️
If you don't care just run:

```powershell
Set-ExecutionPolicy -ExecutionPolicy Unrestricted -Scope LocalMachine
```

# 💯 Brand

## ✍️ Concept

### ✅ Do

- Visual is better than text 👀
- Compact ➡️ Less space ➡️ More information ➡️ Faster to understand 🚀

### ❌ Dont

- Rounded corners ⬜
- Shadows 🌤️
- Multiple unicode directly after each other 🥇🥈🥉

## 🌈 Colors

![Palette](/assets/lists/palette.png)

### 🥇 Primary

### 🥈 Secondary

### 🥉 Tertiary

### ⚫ Dark

### ⚪ Light

### 🩶 Grey

Are you curious how a 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 and 11 colored gradient can come together for an invertible theme in a semantically meaningfull way? Well, here is how you achieve it with 33 colors 🤯

![Grayscale](/assets/lists/grayscale.png)

## 📄 Typography

- One symbol after every sentance 💯
- One symbol at a time 🥇
- Pick emoji if possible otherwise unicode ⚖️
- 📝 One symbol to summarize a title
- 💡 One symbol to summatize a title description and one to think about in the end 🤔
- `.` are forbidden ⛔
- All components in `semio` (`sketchpad`,`studio`, …) start with a small letter 🔡
- Did you know that `…` is just one character?

### 🔡 [Fonts](/assets/fonts/README.md)

- Sans serif: [Anta](https://fonts.google.com/specimen/Anta) 🖨️
- Serif: [Kelly Slab](https://fonts.google.com/specimen/Kelly+Slab) ✍️
- Monospaced: [Shart Tech Mono](https://fonts.google.com/specimen/Share+Tech+Mono) 🖥️
- Emoji: [Noto Emoji](https://fonts.google.com/noto/specimen/Noto+Emoji) ⚫

## 👀 Visual elements

- Sharp corners 📐
- Borders □
- Basic geometric shapes ⚪

# 📊 Stats

<p align="center">
    <a href="https://github.com/usalu/semio"><img src="https://hits.seeyoufarm.com/api/count/incr/badge.svg?url=https%3A%2F%2Fgithub.com%2Fusalu%2Fsemio&count_bg=%23FF344F&title_bg=%23555555&icon=&icon_color=%23E7E7E7&title=visits&edge_flat=true"/></a>
</p>
