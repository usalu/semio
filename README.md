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
    <i>🧩 Design-Information-Modeling for Kit-of-Parts Architecture.</i>
</p>
<br/>

You want to 🧩 the next 🏛️🏘️🏢🏭🏫🏨⛪🕌 with 🤖? But 📐🔢🗣️👥🖱️⌨️ takes all your ⌚? Then try to 🧠 the 🧬 and let semio 💉🖥️✒️🖨️🪄🚀.

# 👋 Hello dev

> [!NOTE]
> Are you a user of semio? Then you probably want to visit our [docs](https://docs.semio-tech.com) 👀

Glad to see you!

Let me walk you throw 🚶

# 🦑 [Monorepo](https://github.com/usalu/semio.git)

This git repo has **everything** that exists in the open semio ecosystem 🤯

# 📦 Components

## ✏️ [sketchpad](https://github.com/usalu/semio/tree/main/js/sketchpad)

## 🦗 [Grasshopper](https://github.com/usalu/semio/tree/main/dotnet/Semio.Grasshopper)

## ⚙️ [engine](https://github.com/usalu/semio/tree/main/python/engine)

A hidden fat-client which exposes shared functionality to other desktop uis 🤝

It takes care of

- CRUDs (Create-Read-Update-Delete) with local kits (and handeles all SQLite interaction, ...)
-

It offers two a simple REST OpenAPI and a complex GraphQL Relay API.

### {} [REST OpenAPI](https://github.com/usalu/semio/tree/main/python/engine/engine.py#L5529)

If you go to `http://127.0.0.1:2412/api/docs/` you find the Swagger UI:

![GraphQL Query](https://raw.githubusercontent.com/usalu/semio/main-tag/openapi/docs/swagger.png)

### ⭕ [GraphQL Relay](https://github.com/usalu/semio/tree/main/python/engine/engine.py#L5095)

> Still a prototype ✏️

If you go to `http://127.0.0.1:2412/graphql/` you find the GraphiQL UI:

![GraphQL Query](https://raw.githubusercontent.com/usalu/semio/main-tag/graphql/docs/graphiql.png)

## 📚 [docs](https://github.com/usalu/semio/tree/main/js/docs)

# 🏘️ [Examples](https://github.com/usalu/semio/tree/main/examples)

## 🚀 [Starter](https://github.com/usalu/semio/tree/main/examples/starter)

## 👋 [Hello semio](https://github.com/usalu/hello-semio)

## 🌈 [Geometry](https://github.com/usalu/geometry)

## 🫀 [Metabolism](https://github.com/usalu/metabolism)

# 💿 Ecosystems

## 🐍 [Python]

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
If you don't care just run

```powershell
Set-ExecutionPolicy -ExecutionPolicy Unrestricted -Scope LocalMachine
```

# 📊 Stats

<p align="center">
    <a href="https://github.com/usalu/semio"><img src="https://hits.seeyoufarm.com/api/count/incr/badge.svg?url=https%3A%2F%2Fgithub.com%2Fusalu%2Fsemio&count_bg=%23FF344F&title_bg=%23555555&icon=&icon_color=%23E7E7E7&title=visits&edge_flat=true"/></a>
</p>
