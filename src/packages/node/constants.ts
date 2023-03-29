const CONSTANTS = {
  "GENERAL_FILEEXTENSIONS": [
    "txt",
    "json",
    "xml"
  ],
  "SERVICES": {
    "GATEWAY": {
      "NAME": "gateway",
      "PORT": 2000
    },
    "GATEWAYRESTPROXY": {
      "NAME": "gatewayrestproxy",
      "PORT": 2999
    },
    "ASSEMBLER": {
      "NAME": "assembler",
      "PORT": 2001
    },
    "MANAGER": {
      "NAME": "manager",
      "PORT": 2002
    }
  },
  "PLATFORMS": {
    "SEMIO": {
      "URL": "usalu/semio",
      "PORT": 65000,
      "FULL_NAME": "semio",
      "NAME": "semio",
      "LICENSE": "AGPL-3.0-or-later",
      "ABBREVIATION": "semio",
      "FILEEXTENSION": ".semio",
      "NORMALFILEEXTENSIONS": [
        ".semio",
        ".semio.json"
      ],
      "SOURCE": "https://github.com/usalu/semio",
      "LEARN": "https://github.com/usalu/semio"
    },
    "THREE": {
      "URL": "mrdoob/three",
      "PORT": 64900,
      "FULL_NAME": "Three.js",
      "NAME": "three",
      "LICENSE": "MIT",
      "ABBREVIATION": "three",
      "FILEEXTENSION": ".three.json",
      "NORMALFILEEXTENSIONS": [
        ".json"
      ],
      "SOURCE": "https://github.com/mrdoob/three.js/",
      "LEARN": "https://threejs.org/"
    },
    "SVERCHOK": {
      "URL": "nortikin/sverchok",
      "PORT": 64800,
      "FULL_NAME": "Sverchok",
      "NAME": "sverchok",
      "LICENSE": "GPL-3.0-only",
      "ABBREVIATION": "sk",
      "FILEEXTENSION": ".sk.json",
      "NORMALFILEEXTENSIONS": [
        ".json",
        ".sk.json"
      ],
      "SOURCE": "https://github.com/nortikin/sverchok",
      "LEARN": "https://nortikin.github.io/sverchok/"
    },
    "IFCOPENSHELL": {
      "URL": "ifcopenshell/ifcopenshell",
      "PORT": 50000,
      "FULL_NAME": "IfcOpenShell",
      "NAME": "ifcopenshell",
      "LICENSE": "LGPL-3.0-only",
      "ABBREVIATION": "ifcos",
      "FILEEXTENSION": ".ifcos",
      "NORMALFILEEXTENSIONS": [
        ".cpp",
        ".py"
      ],
      "SOURCE": "https://github.com/IfcOpenShell/IfcOpenShell",
      "LEARN": "https://ifcopenshell.org/"
    },
    "CADQUERY": {
      "URL": "cadquery/cadquery",
      "PORT": 49900,
      "FULL_NAME": "CadQuery",
      "NAME": "cadquery",
      "LICENSE": "LGPL-3.0-only",
      "ABBREVIATION": "cq",
      "FILEEXTENSION": ".cq",
      "NORMALFILEEXTENSIONS": [
        ".py",
        ".cq.py"
      ],
      "SOURCE": "https://github.com/CadQuery/cadquery/",
      "LEARN": "https://cadquery.readthedocs.io/en/latest/"
    },
    "FREECAD": {
      "URL": "freecad/freecad",
      "PORT": 49800,
      "FULL_NAME": "FreeCAD",
      "NAME": "freecad",
      "LICENSE": "GPL-2.0",
      "ABBREVIATION": "fc",
      "FILEEXTENSION": ".fc",
      "NORMALFILEEXTENSIONS": [
        ".FCMAcro",
        ".py"
      ],
      "SOURCE": "https://github.com/FreeCAD/FreeCAD",
      "LEARN": "https://wiki.freecadweb.org/Main_Page"
    },
    "OPENSCAD": {
      "URL": "openscad/openscad",
      "PORT": 49700,
      "FULL_NAME": "OpenSCAD",
      "NAME": "openscad",
      "LICENSE": "GPL-2.0",
      "ABBREVIATION": "oc",
      "FILEEXTENSION": ".scad",
      "NORMALFILEEXTENSIONS": [
        ".scad"
      ],
      "SOURCE": "https://github.com/openscad",
      "LEARN": "https://openscad.org/"
    },
    "JSCAD": {
      "URL": "jscad/jscad",
      "PORT": 49600,
      "FULL_NAME": "OpenJSCAD",
      "NAME": "jscad",
      "LICENSE": "Apache-2.0",
      "ABBREVIATION": "jc",
      "FILEEXTENSION": "jc.js",
      "NORMALFILEEXTENSIONS": [
        ".js"
      ],
      "SOURCE": "https://github.com/jscad",
      "LEARN": "https://openjscad.xyz/"
    },
    "FORNJOT": {
      "URL": "hannobraun/fornjot",
      "PORT": 49500,
      "FULL_NAME": "Fornjot",
      "NAME": "fornjot",
      "LICENSE": "0BSD",
      "ABBREVIATION": "fj",
      "FILEEXTENSION": ".fj.rs",
      "NORMALFILEEXTENSIONS": [
        ".rs"
      ],
      "SOURCE": "https://github.com/hannobraun/Fornjot",
      "LEARN": "https://fornjot.app/"
    },
    "TRUCK": {
      "URL": " ricosjp/truck",
      "PORT": 49400,
      "FULL_NAME": "Truck",
      "NAME": "truck",
      "LICENSE": "Apache-2.0",
      "ABBREVIATION": "tk",
      "FILEEXTENSION": "tk.rs",
      "NORMALFILEEXTENSIONS": [
        ".rs"
      ],
      "SOURCE": "https://github.com/ricosjp/truck",
      "LEARN": "https://github.com/ricosjp/truck"
    },
    "ENERGYPLUS": {
      "URL": " nrel/energyplus",
      "PORT": 40000,
      "FULL_NAME": "EnergyPlus",
      "NAME": "energyplus",
      "LICENSE": "EnergyPlus",
      "ABBREVIATION": "ep",
      "FILEEXTENSION": ".ep",
      "NORMALFILEEXTENSIONS": [
        ".idf",
        ".epJSON"
      ],
      "SOURCE": "https://github.com/NREL/EnergyPlus",
      "LEARN": "https://energyplus.net/"
    },
    "OPENSTUDIO": {
      "URL": " nrel/openstudio",
      "PORT": 40001,
      "FULL_NAME": "OpenStudio",
      "NAME": "openstudio",
      "LICENSE": "OpenStudio",
      "ABBREVIATION": "os",
      "FILEEXTENSION": ".osm",
      "NORMALFILEEXTENSIONS": [
        ".osm"
      ],
      "SOURCE": "https://github.com/NREL/OpenStudio",
      "LEARN": "https://openstudio.net/"
    },
    "RHINO": {
      "URL": "mcneel/rhino",
      "PORT": 20000,
      "FULL_NAME": "Rhinoceros 3D",
      "NAME": "rhino",
      "ABBREVIATION": "rhino",
      "FILEEXTENSION": ".3dm",
      "NORMALFILEEXTENSIONS": [
        ".3dm"
      ],
      "LEARN": "https://www.rhino3d.com/"
    },
    "GRASSHOPPER": {
      "URL": "mcneel/rhino/grasshopper",
      "FULL_NAME": "Grasshopper 3D",
      "PORT": 20001,
      "NAME": "grasshopper",
      "ABBREVIATION": "gh",
      "FILEEXTENSION": ".gh",
      "NORMALFILEEXTENSIONS": [
        ".gh",
        ".ghx"
      ],
      "LEARN": "https://www.grasshopper3d.com/"
    },
    "REVIT": {
      "URL": "autodesk/revit",
      "PORT": 19900,
      "FULL_NAME": "Revit",
      "NAME": "revit",
      "ABBREVIATION": "rvt",
      "FILEEXTENSION": ".rfa",
      "NORMALFILEEXTENSIONS": [
        ".rfa"
      ],
      "LEARN": "https://www.autodesk.com/autodesk-university/class/Revit-Family-Creation-Step-Step-Introduction-Just-Beginners-2017"
    },
    "DYNAMO": {
      "URL": "autodesk/revit/dynamo",
      "PORT": 19901,
      "FULL_NAME": "Dynamo",
      "NAME": "dynamo",
      "ABBREVIATION": "dyn",
      "FILEEXTENSION": ".dyn",
      "NORMALFILEEXTENSIONS": [
        ".dyn"
      ],
      "LEARN": "https://primer2.dynamobim.org/"
    },
    "ARCHICAD": {
      "URL": "graphisoft/archicad",
      "PORT": 19800,
      "FULL_NAME": "ArchiCAD",
      "NAME": "archicad",
      "ABBREVIATION": "ac",
      "FILEEXTENSION": ".gsl",
      "NORMALFILEEXTENSIONS": [
        ".gdl",
        ".gsl"
      ],
      "LEARN": "https://gdl.graphisoft.com/"
    },
    "CITYENGINE": {
      "URL": "esri/arcgis/cityengine",
      "PORT": 19600,
      "FULL_NAME": "ArcGIS CityEngine",
      "NAME": "cityengine",
      "ABBREVIATION": "ce",
      "FILEEXTENSION": ".rpk",
      "NORMALFILEEXTENSIONS": [
        ".rpk",
        ".cga"
      ],
      "LEARN": "https://esri.github.io/cityengine/"
    },
    "EXCEL": {
      "URL": "microsoft/excel",
      "PORT": 19500,
      "FULL_NAME": "Microsoft Excel",
      "NAME": "excel",
      "ABBREVIATION": "xl",
      "FILEEXTENSION": ".xlsm",
      "NORMALFILEEXTENSIONS": [
        ".xls",
        ".xlsm"
      ],
      "LEARN": "https://support.microsoft.com/en-us/office/use-the-name-manager-in-excel-4d8c4c2b-9f7d-44e3-a3b4-9f61bd5c64e4"
    },
    "HYPAR": {
      "URL": "hypar/hypar",
      "PORT": 10000,
      "FULL_NAME": "Hypar",
      "NAME": "hypar",
      "ABBREVIATION": "hpr",
      "FILEEXTENSION": ".hpr",
      "NORMALFILEEXTENSIONS": [
        ".json"
      ],
      "LEARN": "https://docs.hypar.io/getting-started"
    }
  }
}

export const GATEWAY_PORT = CONSTANTS['GATEWAY_PORT']
export const MANAGER_PORT = CONSTANTS['MANAGER_PORT']

const PLATFORMS = CONSTANTS['PLATFORMS']

export const THREE = PLATFORMS['three']
export const RHINO = PLATFORMS['rhino']
export const GRASSHOPPER = PLATFORMS['gh']