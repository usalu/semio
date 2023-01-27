const CONSTANTS = {
  "DEFAULT_GATEWAY_PORT": 2000,
  "DEFAULT_GATEWAY_PROXY_PORT": 2999,
  "DEFAULT_ASSEMBLER_PORT": 2001,
  "DEFAULT_MANAGER_PORT": 2002,
  "GENERAL_EXTENSIONS": [
    "txt",
    "json",
    "xml"
  ],
  "PLATFORMS": {
    "SEMIO": {
      "URL": "usalu/semio",
      "DEFAULT_PORT": 65000,
      "FULL_NAME": "semio",
      "NAME": "semio",
      "LICENSE": "AGPL-3.0-or-later",
      "ABBREVIATION": "semio",
      "EXTENSION": ".semio",
      "NORMAL_EXTENSIONS": [
        ".semio",
        ".semio.json"
      ],
      "SOURCE": "https://github.com/usalu/semio",
      "LEARN": "https://github.com/usalu/semio"
    },
    "THREE": {
      "URL": "mrdoob/three",
      "DEFAULT_PORT": 64900,
      "FULL_NAME": "Three.js",
      "NAME": "three",
      "LICENSE": "MIT",
      "ABBREVIATION": "three",
      "EXTENSION": ".three.json",
      "NORMAL_EXTENSIONS": [
        ".json"
      ],
      "SOURCE": "https://github.com/mrdoob/three.js/",
      "LEARN": "https://threejs.org/"
    },
    "SVERCHOK": {
      "URL": "nortikin/sverchok",
      "DEFAULT_PORT": 64800,
      "FULL_NAME": "Sverchok",
      "NAME": "sverchok",
      "LICENSE": "GPL-3.0-only",
      "ABBREVIATION": "sk",
      "EXTENSION": ".sk.json",
      "NORMAL_EXTENSIONS": [
        ".json",
        ".sk.json"
      ],
      "SOURCE": "https://github.com/nortikin/sverchok",
      "LEARN": "https://nortikin.github.io/sverchok/"
    },
    "IFCOPENSHELL": {
      "URL": "ifcopenshell/ifcopenshell",
      "DEFAULT_PORT": 50000,
      "FULL_NAME": "IfcOpenShell",
      "NAME": "ifcopenshell",
      "LICENSE": "LGPL-3.0-only",
      "ABBREVIATION": "ifcos",
      "EXTENSION": ".ifcos",
      "NORMAL_EXTENSIONS": [
        ".cpp",
        ".py"
      ],
      "SOURCE": "https://github.com/IfcOpenShell/IfcOpenShell",
      "LEARN": "https://ifcopenshell.org/"
    },
    "CADQUERY": {
      "URL": "cadquery/cadquery",
      "DEFAULT_PORT": 49900,
      "FULL_NAME": "CadQuery",
      "NAME": "cadquery",
      "LICENSE": "LGPL-3.0-only",
      "ABBREVIATION": "cq",
      "EXTENSION": ".cq",
      "NORMAL_EXTENSIONS": [
        ".py",
        ".cq.py"
      ],
      "SOURCE": "https://github.com/CadQuery/cadquery/",
      "LEARN": "https://cadquery.readthedocs.io/en/latest/"
    },
    "FREECAD": {
      "URL": "freecad/freecad",
      "DEFAULT_PORT": 49800,
      "FULL_NAME": "FreeCAD",
      "NAME": "freecad",
      "LICENSE": "GPL-2.0",
      "ABBREVIATION": "fc",
      "EXTENSION": ".fc",
      "NORMAL_EXTENSIONS": [
        ".FCMAcro",
        ".py"
      ],
      "SOURCE": "https://github.com/FreeCAD/FreeCAD",
      "LEARN": "https://wiki.freecadweb.org/Main_Page"
    },
    "OPENSCAD": {
      "URL": "openscad/openscad",
      "DEFAULT_PORT": 49700,
      "FULL_NAME": "OpenSCAD",
      "NAME": "openscad",
      "LICENSE": "GPL-2.0",
      "ABBREVIATION": "oc",
      "EXTENSION": ".scad",
      "NORMAL_EXTENSIONS": [
        ".scad"
      ],
      "SOURCE": "https://github.com/openscad",
      "LEARN": "https://openscad.org/"
    },
    "JSCAD": {
      "URL": "jscad/jscad",
      "DEFAULT_PORT": 49600,
      "FULL_NAME": "OpenJSCAD",
      "NAME": "jscad",
      "LICENSE": "Apache-2.0",
      "ABBREVIATION": "jc",
      "EXTENSION": "jc.js",
      "NORMAL_EXTENSIONS": [
        ".js"
      ],
      "SOURCE": "https://github.com/jscad",
      "LEARN": "https://openjscad.xyz/"
    },
    "FORNJOT": {
      "URL": "hannobraun/fornjot",
      "DEFAULT_PORT": 49500,
      "FULL_NAME": "Fornjot",
      "NAME": "fornjot",
      "LICENSE": "0BSD",
      "ABBREVIATION": "fj",
      "EXTENSION": ".fj.rs",
      "NORMAL_EXTENSIONS": [
        ".rs"
      ],
      "SOURCE": "https://github.com/hannobraun/Fornjot",
      "LEARN": "https://fornjot.app/"
    },
    "TRUCK": {
      "URL": " ricosjp/truck",
      "DEFAULT_PORT": 49400,
      "FULL_NAME": "Truck",
      "NAME": "truck",
      "LICENSE": "Apache-2.0",
      "ABBREVIATION": "tk",
      "EXTENSION": "tk.rs",
      "NORMAL_EXTENSIONS": [
        ".rs"
      ],
      "SOURCE": "https://github.com/ricosjp/truck",
      "LEARN": "https://github.com/ricosjp/truck"
    },
    "ENERGYPLUS": {
      "URL": " nrel/energyplus",
      "DEFAULT_PORT": 40000,
      "FULL_NAME": "EnergyPlus",
      "NAME": "energyplus",
      "LICENSE": "EnergyPlus",
      "ABBREVIATION": "ep",
      "EXTENSION": ".ep",
      "NORMAL_EXTENSIONS": [
        ".idf",
        ".epJSON"
      ],
      "SOURCE": "https://github.com/NREL/EnergyPlus",
      "LEARN": "https://energyplus.net/"
    },
    "OPENSTUDIO": {
      "URL": " nrel/openstudio",
      "DEFAULT_PORT": 40001,
      "FULL_NAME": "OpenStudio",
      "NAME": "openstudio",
      "LICENSE": "OpenStudio",
      "ABBREVIATION": "os",
      "EXTENSION": ".osm",
      "NORMAL_EXTENSIONS": [
        ".osm"
      ],
      "SOURCE": "https://github.com/NREL/OpenStudio",
      "LEARN": "https://openstudio.net/"
    },
    "RHINO": {
      "URL": "mcneel/rhino",
      "DEFAULT_PORT": 20000,
      "FULL_NAME": "Rhinoceros 3D",
      "NAME": "rhino3D",
      "ABBREVIATION": "rhino",
      "EXTENSION": ".3dm",
      "NORMAL_EXTENSIONS": [
        ".3dm"
      ],
      "LEARN": "https://www.rhino3d.com/"
    },
    "GRASSHOPPER": {
      "URL": "mcneel/rhino/grasshopper",
      "FULL_NAME": "Grasshopper 3D",
      "DEFAULT_PORT": 20001,
      "NAME": "grasshopper",
      "ABBREVIATION": "gh",
      "EXTENSION": ".gh",
      "NORMAL_EXTENSIONS": [
        ".gh",
        ".ghx"
      ],
      "LEARN": "https://www.grasshopper3d.com/"
    },
    "REVIT": {
      "URL": "autodesk/revit",
      "DEFAULT_PORT": 19900,
      "FULL_NAME": "Revit",
      "NAME": "revit",
      "ABBREVIATION": "rvt",
      "EXTENSION": ".rfa",
      "NORMAL_EXTENSIONS": [
        ".rfa"
      ],
      "LEARN": "https://www.autodesk.com/autodesk-university/class/Revit-Family-Creation-Step-Step-Introduction-Just-Beginners-2017"
    },
    "DYNAMO": {
      "URL": "autodesk/revit/dynamo",
      "DEFAULT_PORT": 19901,
      "FULL_NAME": "Dynamo",
      "NAME": "dynamo",
      "ABBREVIATION": "dyn",
      "EXTENSION": ".dyn",
      "NORMAL_EXTENSIONS": [
        ".dyn"
      ],
      "LEARN": "https://primer2.dynamobim.org/"
    },
    "ARCHICAD": {
      "URL": "graphisoft/archicad",
      "DEFAULT_PORT": 19800,
      "FULL_NAME": "ArchiCAD",
      "NAME": "archicad",
      "ABBREVIATION": "ac",
      "EXTENSION": ".gsl",
      "NORMAL_EXTENSIONS": [
        ".gdl",
        ".gsl"
      ],
      "LEARN": "https://gdl.graphisoft.com/"
    },
    "CITYENGINE": {
      "URL": "esri/arcgis/cityengine",
      "DEFAULT_PORT": 19600,
      "FULL_NAME": "ArcGIS CityEngine",
      "NAME": "cityengine",
      "ABBREVIATION": "ce",
      "EXTENSION": ".rpk",
      "NORMAL_EXTENSIONS": [
        ".rpk",
        ".cga"
      ],
      "LEARN": "https://esri.github.io/cityengine/"
    },
    "EXCEL": {
      "URL": "microsoft/excel",
      "DEFAULT_PORT": 19500,
      "FULL_NAME": "Microsoft Excel",
      "NAME": "excel",
      "ABBREVIATION": "xl",
      "EXTENSION": ".xlsm",
      "NORMAL_EXTENSIONS": [
        ".xls",
        ".xlsm"
      ],
      "LEARN": "https://support.microsoft.com/en-us/office/use-the-name-manager-in-excel-4d8c4c2b-9f7d-44e3-a3b4-9f61bd5c64e4"
    },
    "HYPAR": {
      "URL": "hypar/hypar",
      "DEFAULT_PORT": 10000,
      "FULL_NAME": "Hypar",
      "NAME": "hypar",
      "ABBREVIATION": "hpr",
      "EXTENSION": ".hpr",
      "NORMAL_EXTENSIONS": [
        ".json"
      ],
      "LEARN": "https://docs.hypar.io/getting-started"
    }
  }
}

export const DEFAULT_GATEWAY_PORT = CONSTANTS['DEFAULT_GATEWAY_PORT']
export const DEFAULT_MANAGER_PORT = CONSTANTS['DEFAULT_MANAGER_PORT']

const PLATFORMS = CONSTANTS['PLATFORMS']

export const THREE = PLATFORMS['three']
export const RHINO = PLATFORMS['rhino']
export const GRASSHOPPER = PLATFORMS['gh']