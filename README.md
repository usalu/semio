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

<picture>
    <source media="(prefers-color-scheme: dark)" srcset="docs/conceptual/analogies-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="docs/conceptual/analogies.svg">
    <img alt="Analogies" src="docs/conceptual/analogies.svg">
</picture>

semio is intended to become an open ecosystem for component-based design of architecture. The decomposition of a design by a graph enables a collaborative, scalable and cross-platform design process. Instead of 3D blobs, proprietary or documentary file formats from which it is difficult to extract individual parts, the heart of semio consists of a linked data model. All components can be easily shared, searched and reused. Unlike existing data models, not every design is squeezed into a handful of standard types such as floors, walls or windows, but you can define more individual, complex and flexible types, which can be freely combined via a generalized port mechanism.

## Getting started

### Requirements
- Windows
- Rhino 7
- Hops package from the Rhino Package Manger

> NOTE: Hops (in reality Rhino.Compute) needs certain runtimes which are not by default installed. The developers probably didn't notice because every one had Visual Studio already installed. You can fix this by installing [Visual Studio](https://visualstudio.microsoft.com/) and install .NET for desktop development.
![Visual Studio .NET](docs/installation/visual-studio-dotnet.png)
To check if Hops is running correctly, you can uncheck the checkbox "Hide 
Compute.Compute Console Window". Make sure that "Launch Local Rhino.Compute at Start" checkbox is checked.
![Show Compute Option](docs/installation/show-compute.png)
When you restart Rhino and start Grasshopper a window like this should appear for Rhino.Compute:
![Compute](docs/installation/compute.png)
If Rhino.Compute still doesn't run, then try cloning the official [Rhino.Compute Git Repository](https://github.com/mcneel/compute.rhino3d) and open `src\compute.sln` in Visual Studio and agree on downloading the missing packages.

### Installation
1. Open Rhino Package Manger
1. Search for semio
1. Install the latest version
1. Restart Rhino

That's it!

> NOTE: The first time you will use semio, the Windows Firewall will ask you to allow internet access for
`restproxy.exe` and `semio.exe`. This is because the semio backend is a microservice architecture which communicates over several ports. semio will by default **<ins>not</ins>** access the internet because a local server will be started. If you want to host the backend somewhere else you **<ins>can</ins>** but that requires a Rhino.Compute license, an active server and configuration in the client.

### Hello World
Now you can lay out your first design!
![Hello World](docs/examples/helloworld/grasshopper.gif)

### Nakagin Capsule Tower
At least in semio, the iconic capsule tower(s) will keep on existing!
![Nakagin Capsule Tower](docs/examples/capsuletower/grasshopper.gif)

Ever wanted to change the capsules with one click?
![Nakagin Capsule Tower](docs/examples/capsuletower/grasshopper-variant.gif)

## How does semio work?

>NOTE: This only applies (and not to full extent) to v1. We are working on a complete rewrite with a totally different metamodel, system architecture and software architecture. Short: Almost everything will change... but many more exciting features will come! Such as a persistance layer for easy sharing design components. A shared type no longer needs a runtime but parameters and geometry are persisted.

<picture>
    <source media="(prefers-color-scheme: dark)" srcset="docs/softwarequality/systemarchitecture/componentsdiagram-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="docs/softwarequality/systemarchitecture/componentsdiagram.svg">
    <img alt="Components" src="docs/softwarequality/systemarchitecture/componentsdiagram.svg">
</picture>

semio is an ecosystem of several components. There is a backend that can be extended on different platforms and different frontends for viewing and/or authoring designs. All services including their depenencies are vendored into the platform extensions to run as a binary. Consider the rest as magic.

<picture>
    <source media="(prefers-color-scheme: dark)" srcset="docs/softwarequality/softwarearchitecture/metamodel-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="docs/softwarequality/softwarearchitecture/metamodel.svg">
    <img alt="Metamodel" src="docs/softwarequality/softwarearchitecture/metamodel.svg">
</picture>

Here is a brief overview over the process:

<picture>
    <source media="(prefers-color-scheme: dark)" srcset="docs/softwarequality/softwarearchitecture/designprocessmodel-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="docs/softwarequality/softwarearchitecture/designprocessmodel.svg">
    <img alt="Process Part 1" src="docs/softwarequality/softwarearchitecture/designprocessmodel.svg">
</picture>

<picture>
    <source media="(prefers-color-scheme: dark)" srcset="docs/softwarequality/softwarearchitecture/designprocessmodel2-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="docs/softwarequality/softwarearchitecture/designprocessmodel2.svg">
    <img alt="Process Part 2" src="docs/softwarequality/softwarearchitecture/designprocessmodel2.svg">
</picture>

## UIs

Currently there is one UI for Grasshopper.
![Menu ribbon](docs/adapters/grasshopper/ribbon.png)

## Extensions

Currently there is one extension for Grasshopper that makes it possible to turn Grasshopper scripts into semio scripts.

## Contribution

![]()

<picture>
    <source media="(prefers-color-scheme: dark)" srcset="docs/softwarequality/softwarearchitecture/frameworkrelationships-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="docs/softwarequality/softwarearchitecture/frameworkrelationships.svg">
    <img alt="Components" src="docs/softwarequality/softwarearchitecture/frameworkrelationships.svg">
</picture>


If you want to contribute to the project, there are lot's of opportunities! Do you want to write an apapter for a platform or contribute to the core?
If you are not sure what your contribution could exactly be, feel free to take a look under the [project site](https://github.com/users/usalu/projects/2) and see if find something.