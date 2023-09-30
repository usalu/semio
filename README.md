<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="resources/logo/logo-horizontal-dark.svg">
        <source media="(prefers-color-scheme: light)" srcset="resources/logo/logo-horizontal.svg">
        <img alt="semio" href="https://github.com/usalu/semio/" src="resources/logo/logo-horizontal.svg">
    </picture>
    <br/>
    <a href="https://github.com/usalu/semio/"><img src="https://img.shields.io/github/v/release/usalu/semio?style=flat-square&color=ff344f" alt="LGPLv3 License"></a>
    <a href="https://choosealicense.com/licenses/lgpl-3.0/"><img src="docs\badges\license-LGPL_v3.svg" alt="LGPLv3 License"></a>
    <br/>
    <a href="https://www.grasshopper3d.com/"><img src="docs\badges\platform-Grasshopper.svg" alt="Grasshopper"></a>
    <a href="https://www.python.org/"><img src="docs\badges\platform-comming_soon(Python).svg" alt="Python"></a>
    <a href="https://nortikin.github.io/sverchok/"><img src="docs\badges\platform-comming_soon(Sverchok).svg" alt="Sverchok"></a>
    <br/>
    <a href="https://www.python.org/"><img src="docs\badges\powered_by-Python.svg" alt="Python"></a>
    <a href="<http://www.w3.org/2001/sw/wiki/RDF>"><img src="docs\badges\powered_by-Semantic_Web.svg" alt="Semantic Web"></a>
    <a href="https://speckle.systems/"><img src="docs\badges\powered_by-Speckle.svg" alt="Speckle Systems"></a>
    <br/>
    <br/>
    <i>Collaborative, scalable and cross-platform designing.</i>
</p>
<br/>

You want to <emoji>✏️</emoji> the next <emoji>🏛️🏘️🏢🏭🏫🏨⛪🕌</emoji>? But <emoji>📐🔢🗣️</emoji> takes all your <emoji>⌚</emoji>? Then try to capture the <emoji>🧬</emoji> and <emoji>💉</emoji>  it into a new design and let semio <emoji>🖥️✒️🖨️</emoji>.

<p align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="docs/examples/capsuletower/artifacts-dark.svg">
        <source media="(prefers-color-scheme: light)" srcset="docs/examples/capsuletower/artifacts.svg">
        <img alt="Capsule Tower Artifacts" src="docs/examples/capsuletower/artifacts.svg">
    </picture>
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="docs/examples/capsuletower/computation-dark.svg">
        <source media="(prefers-color-scheme: light)" srcset="docs/examples/capsuletower/computation.svg">
        <img alt="Capsule Tower Computation" src="docs/examples/capsuletower/computation.svg">
    </picture>
</p>

## What is semio?
<p align="center">
    <img src="docs/conceptual/simplifiedcontext.svg" alt="drawing" width="400"/>
</p>

semio

# Getting started

## Installation

Requirements:
- Windows 11
- Rhino 7 with Grasshopper 1
- Hops from the Rhino Package Manger

The installation of semio is just one click inside of the default Rhino Package Manger:


> NOTE: Hops (in reality Rhino.Compute) needs certain runtimes which are not by default installed. The developers probably didn't notice because every one had Visual Studio already installed. You can fix this by installing [Visual Studio](https://visualstudio.microsoft.com/) and install .NET for desktop development.
![Alt text](docs\installation\visual-studio-dotnet.png)
To check if Hops is running correctly, you can uncheck the checkbox "Hide 
Compute.Compute Console Window". Make sure that "Launch Local Rhino.Compute at Start" checkbox is checked.
![Alt text](docs\installation\show-compute.png)
When you restart Rhino and start Grasshopper a window like this should appear for Rhino.Compute:
![Alt text](docs\installation\compute.png)
If Rhino.Compute still doesn't run, then try cloning the official [Rhino.Compute Git Repository](https://github.com/mcneel/compute.rhino3d) and open `src\compute.sln` in Visual Studio and agree on downloading the missing packages.
## 


# How does semio work?

![Components](docs/softwarequality/systemarchitecture/componentsdiagram.svg)

![Example](docs/softwarequality/softwarearchitecture/metamodel.svg)

semio is an ecosystem of several components. There is a backend that can be extended on different platforms and different frontends for viewing and/or authoring designs.

All services can either run locally, with docker-compose, inside kubernetes or be simply called over an active server.

## UIs

Currently there is one UI for Grasshopper.
![Menu ribbon](docs/adapters/grasshopper/ribbon.png)

## Extensions

Currently there is one extension for Grasshopper that makes it possible to turn Grasshopper scripts into semio scripts.

## Contribution

![Components](docs/softwarequality/softwarearchitecture/frameworkrelationships.svg)


If you want to contribute to the project, there are lot's of opportunities! Do you want to write an apapter for a platform or contribute to the core?
If you are not sure what your contribution could exactly be, feel free to take a look under the [project site](https://github.com/users/usalu/projects/2) and see if find something.