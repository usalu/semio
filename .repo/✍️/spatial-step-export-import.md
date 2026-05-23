Mapping a custom, behavior-driven spatial framework from JSON to STEP (ISO 10303-242) is an excellent architectural move. It transitions your data from a brittle custom schema to an international, high-fidelity engineering standard.

Because STEP uses a relational, entity-based text structure, it can natively handle your entire geometry, topology, and metadata framework. The trick is mapping your **behavioral runtime elements** (Actions, Interactions, and Hashes) into STEP’s static, declarative syntax.

Here is the architectural blueprint for mapping your "Spatial Framework" into STEP AP242.

---

## 1. Core Framework Mapping

| Spatial Framework Concept | STEP AP242 Equivalent Entity           | Implementation Strategy                                                                                                |
| ------------------------- | -------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| **Scalar / Primitive**    | Part 42 Geometry & Topology            | Direct 1:1 mapping to standard STEP mathematical entities (`CARTESIAN_POINT`, `LINE`, `PLANE`, `ADVANCED_FACE`, etc.). |
| **Attribute** (Authored)  | `SHAPE_ASPECT` + `PROPERTY_DEFINITION` | Linked to the specific topological entity ID (e.g., `#FaceID`).                                                        |
| **Property** (Derived)    | `PRODUCT_DEFINITION_PROPERTIES`        | Linked directly to the Object (`PRODUCT_DEFINITION`), bypassing the geometry.                                          |
| **Object**                | `PRODUCT_DEFINITION`                   | Acts as the primary instance of a physical or conceptual thing.                                                        |
| **Model**                 | `PRODUCT_DEFINITION_FORMATION`         | A specific version or instance of a product configuration.                                                             |
| **ModelSpace**            | Entity ID Sharing & Pointers           | Solved natively in STEP via **instance sharing** (multiple models referencing the exact same entity `#ID`).            |

---

## 2. Implementing the Tricky Mechanics

### The Primitive Hash & ModelSpace Sync

You defined a rule: _If a primitive is edited, all primitives with the same hash are edited._ In a JSON file, you usually copy-paste the geometry and match them later via a string hash. In STEP, you can handle this **by reference (Instance Sharing)**. If two different models inside your ModelSpace use the exact same geometry, they should point to the **exact same entity ID** in the file.

To store the actual hash value without affecting the geometry, you can append a system attribute to the root topological entity:

```text
/* The topological solid primitive */
#500 = MANIFOLD_SOLID_BREP('Box Solid', #400);

/* Attaching the immutable structural hash to the solid */
#501 = PROPERTY_DEFINITION('Spatial_Hash', 'System_Generated', #500);
#502 = DESCRIPTIVE_REPRESENTATION_ITEM('HashValue', 'a1b2c3d4e5f6...');
#503 = PROPERTY_DEFINITION_REPRESENTATION(#501, #502);

```

Because `#501` points _to_ `#500`, but `#500` does not point to `#501`, changing the attributes or hashes will never alter the underlying geometric definition—perfectly matching your hashing rule.

### Typology (Classification)

An Object in your framework has a Typology (e.g., `aec.building.energy.Roof`). In STEP, this is handled using **Classification Assignments**.

```text
#600 = PRODUCT_DEFINITION('Roof_Object_1', 'Instance of Roof', #601, #602);
#603 = CLASSIFICATION_CLASS('aec.building.energy.Roof', 'Typology', $);
#604 = APPLIED_CLASSIFICATION_ASSIGNMENT(#603, (#600));

```

---

## 3. Mapping ModelDefinition & Behavioral Runtime (Actions / Interactions)

STEP is traditionally a data-exchange format, not an execution engine. To serialize your declarative headless **Actions**, **Interactions**, and **Transformations**, you must leverage STEP's `DOCUMENT` and `REPRESENTATION` capabilities to store your declarative state machines.

There are two primary ways to handle this:

### Approach A: Embedded Declarative Strings (Self-Contained)

You can store your declarative headless operation documents directly inside the STEP file as structured text blocks inside a non-geometric `REPRESENTATION` context.

```text
/* Defining an Action for a Typology */
#700 = ACTION('constructBoxFrom3Points', 'Headless Operation', $);
#701 = DESCRIPTIVE_REPRESENTATION_ITEM('Execution_Logic', '{ "reads": "variables", "invokes": "spatial.createBox" }');

/* Associating the Action to the Typology Class */
#702 = APPLIED_ACTION_ASSIGNMENT(#700, (#603));

```

### Approach B: External Document References (Cleanest)

If your Actions and Interactions are complex JSON state machines, you can keep them in external files and link them inside STEP using `DOCUMENT_RELATIONSHIP` entities. This treats your state machines as official "specifications" governing the model.

```text
#800 = DOCUMENT('interaction_state_machine', 'Renderer Interactions', $, $);
#801 = DOCUMENT_FILE('interaction_state_machine', 'spatial_renderer_v1.json', $, $, $);
#802 = APPLIED_DOCUMENT_REFERENCE(#800, 'INTERACTION_SCHEMA', (#603)); /* Linked to Typology */

```

---

## 4. Concrete Example: The AEC Energy Transformation

Here is how your example of a `spatial.shape` model transforming into an AEC energy model with a wall attribute (`exposure = external`) looks when serialized into STEP AP242:

```text
ISO-10303-21;
HEADER;
FILE_DESCRIPTION(('Spatial Framework Export'), '2;1');
FILE_NAME('aec_energy_model.stp', '2026-05-28T15:30:00', ...);
FILE_SCHEMA(('AP242_MANAGED_MODEL_BASED_3D_ENGINEERING_MIM_LF'));
ENDSEC;

DATA;
/* 1. Geometry & Topology (The Primitive) */
#10 = CARTESIAN_POINT('Vertex1', (0.0, 0.0, 0.0));
/* ... other geometry points, lines, loops ... */
#50 = ADVANCED_FACE('Wall_Face_Primitive', (#45), #46, .T.);

/* 2. The Object and its Typology */
#100 = PRODUCT('Wall_Object', 'Wall 1', $, ($));
#101 = PRODUCT_DEFINITION_FORMATION('1.0', $, #100);
#102 = PRODUCT_DEFINITION('Design_View', $, #101, #200);
#103 = CLASSIFICATION_CLASS('aec.building.energy.Wall', 'Typology', $);
#104 = APPLIED_CLASSIFICATION_ASSIGNMENT(#103, (#102));

/* 3. Property: Derived Metadata (uValue = 0.25) on the Object */
#300 = PRODUCT_DEFINITION_SHAPE('Properties', $, #102);
#301 = PROPERTY_DEFINITION('uValue', 'Derived Property', #300);
#302 = REAL_REPRESENTATION_ITEM('Value', 0.25, #303); /* Pointing to unit context */

/* 4. Attribute: Authored Metadata (exposure = external) on the Primitive Face */
#400 = SHAPE_ASPECT('External_Face_Feature', 'Authored Attribute Target', #300, .F.);
#401 = PROPERTY_DEFINITION('exposure', 'Authored Attribute', #400);
#402 = DESCRIPTIVE_REPRESENTATION_ITEM('Value', 'external');
#403 = PROPERTY_DEFINITION_REPRESENTATION(#401, #404); /* Wraps the item */

/* 5. Model Space Linkage / Action Reference */
#500 = ACTION('spatial.shape_to_aec.building.energy', 'Transformation Rule', $);
#501 = APPLIED_ACTION_ASSIGNMENT(#500, (#102));

ENDSEC;
END-ISO-10303-21;

```

## How do you plan to handle the runtime execution of these files—will you be writing a custom STEP parser for your application engine, or are you adapting an existing open-source STEP library (like Open CASCADE or pythonocc) to read these custom behavioral properties?

## In the file should only be modelspace, models, objects, primitives, attributes, properties

To strictly enforce this six-element architecture, we can completely strip out all behavioral, execution, and external schema entities from the STEP file.

The file will function purely as a **hierarchical state snapshot**. The runtime will read this file, find the IDs and relationships, compute the hashes on the fly, and bind the execution rules out of its own memory.

Here is the exact mapping and the cleaned structural STEP architecture containing **only** your six core pillars.

---

### **The Clean 6-Pillar Mapping**

| Concept           | STEP AP242 Structural Entity                      | Purpose in File                                                           |
| ----------------- | ------------------------------------------------- | ------------------------------------------------------------------------- |
| **1. ModelSpace** | Root `PRODUCT_DEFINITION` (Assembly)              | The top-level folder container holding related models.                    |
| **2. Model**      | Sub-Assembly `PRODUCT_DEFINITION`                 | A child node of the ModelSpace representing a specific state/view.        |
| **3. Object**     | Leaf Node `PRODUCT_DEFINITION`                    | An instance of a typology containing properties and pointing to geometry. |
| **4. Primitive**  | Part 42 Topology/Geometry (`ADVANCED_FACE`, etc.) | The raw geometric or topological shape components.                        |
| **5. Attribute**  | `SHAPE_ASPECT` + Property Map                     | **Authored** metadata directly linked to a specific Primitive ID.         |
| **6. Property**   | `PRODUCT_DEFINITION_SHAPE` + Property Map         | **Derived** metadata linked directly to the Object ID.                    |

---

### **Cleaned Serialization Example**

This example represents your AEC Energy conversion case. It contains nothing but the spatial structural container, the models, the objects, the shared primitive geometry, the authored attributes, and the derived properties.

```text
ISO-10303-21;
HEADER;
FILE_DESCRIPTION(('Pure Spatial State Export'), '2;1');
FILE_NAME('spatial_state.stp', '2026-05-28T16:00:00', ...);
FILE_SCHEMA(('AP242_MANAGED_MODEL_BASED_3D_ENGINEERING_MIM_LF'));
ENDSEC;

DATA;
/* ========================================================================= */
/* 1. MODELSPACE (The Global Container)                                      */
/* ========================================================================= */
#10 = PRODUCT('Spatial_ModelSpace_01', 'Core Project Space', $, ($));
#11 = PRODUCT_DEFINITION_FORMATION('1.0', $, #10);
#12 = PRODUCT_DEFINITION('ModelSpace_Context', 'ModelSpace', #11, #100);

/* ========================================================================= */
/* 2. MODELS (Instances inside the ModelSpace)                               */
/* ========================================================================= */
/* Model A: The raw shape model */
#20 = PRODUCT_DEFINITION('Shape_Model', 'Model', #11, #100);
/* Model B: The derived AEC Energy model */
#21 = PRODUCT_DEFINITION('AEC_Energy_Model', 'Model', #11, #100);

/* Link Models to the parent ModelSpace container */
#22 = NEXT_ASSEMBLY_USAGE_OCCURRENCE('MS_L1', 'Link', $, #12, #20, $);
#23 = NEXT_ASSEMBLY_USAGE_OCCURRENCE('MS_L2', 'Link', $, #12, #21, $);

/* ========================================================================= */
/* 3. OBJECTS (Instances of Typologies inside Models)                        */
/* ========================================================================= */
/* Object inside the Energy Model (Typology: Roof) */
#30 = PRODUCT_DEFINITION('Roof_Object_1', 'aec.building.energy.Roof', #11, #100);
#31 = NEXT_ASSEMBLY_USAGE_OCCURRENCE('M_L1', 'Link', $, #21, #30, $);

/* ========================================================================= */
/* 4. PRIMITIVES (Geometry/Topology)                                         */
/* ========================================================================= */
/* This exact primitive ID can be referenced by multiple objects/models,     */
/* fulfilling your rule: "editing one edits all with the same reference"    */
#40 = ADVANCED_FACE('Surface_Primitive_1', (#39), #38, .T.);

/* Bind the primitive geometry to the Object container */
#41 = PRODUCT_DEFINITION_SHAPE('Object_Geometry', $, #30);
#42 = SHAPE_DEFINITION_REPRESENTATION(#41, #43);
#43 = SHAPE_REPRESENTATION('B-Rep Local Space', (#40), #100);

/* ========================================================================= */
/* 5. ATTRIBUTES (Authored Metadata attached to Primitives)                 */
/* ========================================================================= */
/* Target a specific face primitive (#40) via a shape aspect */
#50 = SHAPE_ASPECT('External_Face_Feature', 'Attribute Anchor', #41, .F.);
/* In a full file, an intermediate entity maps #50 explicitly to #40 */

#51 = PROPERTY_DEFINITION('exposure', 'Authored Attribute', #50);
#52 = DESCRIPTIVE_REPRESENTATION_ITEM('Value', 'external');
#53 = REPRESENTATION('Attr_Group', (#52), #100);
#54 = PROPERTY_DEFINITION_REPRESENTATION(#51, #53);

/* ========================================================================= */
/* 6. PROPERTIES (Derived Metadata attached to Objects)                     */
/* ========================================================================= */
/* Bypasses geometry entirely, links directly to the Object context (#41) */
#61 = PROPERTY_DEFINITION('uValue', 'Derived Property', #41);
#62 = REAL_REPRESENTATION_ITEM('Value', 0.25, #101);
#63 = REPRESENTATION('Prop_Group', (#62), #100);
#64 = PROPERTY_DEFINITION_REPRESENTATION(#61, #63);

ENDSEC;
END-ISO-10303-21;

```

### Why this structure matches your engine:

- **No Code Bloat:** Actions and state machines live completely in your runtime engine code, keeping this file lightweight and clean.
- **Implicit Hashing:** Because the file contains only the mathematical elements of the primitives (`#40`), your runtime can open this file, parse the geometric arrays, generate the hash string, and perform the change validation without the file needing to manage hash tracking fields.
