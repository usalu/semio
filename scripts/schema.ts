#!/usr/bin/env tsx
/**
 * Schema extraction and synchronization script.
 *
 * Extracts schema definitions from:
 * - semio.ts (TypeScript - source of truth)
 * - engine.py (Python)
 * - Semio.cs (C#)
 * - Semio.Grasshopper.cs (Grasshopper components)
 * - dataarchitecture.pu (Database schema)
 * - interfacearchitecture.txt (API schema)
 * - softwarearchitecture.pu (Software class diagrams)
 *
 * Generates reports: schema-ts.json, schema-py.json, schema-net.json, schema-grasshopper.json,
 * schema-database.json, schema-interface.json, schema-software.json, schema.json (summary)
 */

import { readFileSync, writeFileSync } from "fs";
import { join } from "path";

const rootDir = join(__dirname, "..");
const reportsDir = join(rootDir, "reports");
const engineeringDir = join(rootDir, "engineering");

// File paths
const semioTsPath = join(rootDir, "js", "js", "semio.ts");
const enginePyPath = join(rootDir, "py", "engine", "engine.py");
const semioCsPath = join(rootDir, "net", "Semio", "Semio.cs");
const grasshopperCsPath = join(rootDir, "net", "Semio.Grasshopper", "Semio.Grasshopper.cs");
const dataArchPath = join(engineeringDir, "dataarchitecture.pu");
const interfaceArchPath = join(engineeringDir, "interfacearchitecture.txt");
const softwareArchPath = join(engineeringDir, "softwarearchitecture.pu");

// #region Types

interface EntityField {
    name: string;
    type: string;
    optional: boolean;
    description?: string;
}

interface Entity {
    name: string;
    fields: EntityField[];
    hasId: boolean;
    hasDiff: boolean;
    hasDiffs: boolean;
}

interface GrasshopperComponent {
    name: string;
    nickname: string;
    inputs: EntityField[];
    outputs: EntityField[];
}

interface SchemaReport {
    timestamp: string;
    entities: Entity[];
    idTypes: string[];
    weakEntities: string[];
}

interface GrasshopperReport {
    timestamp: string;
    components: GrasshopperComponent[];
    params: string[];
    goos: string[];
}

interface Issue {
    severity: "error" | "warning";
    entity: string;
    field?: string;
    message: string;
    source: string;
}

interface DatabaseField {
    name: string;
    type: string;
    constraints: string[];
}

interface DatabaseEntity {
    name: string;
    fields: DatabaseField[];
}

interface DatabaseRelationship {
    from: string;
    to: string;
    cardinality: string;
    label: string;
}

interface DatabaseReport {
    timestamp: string;
    entities: DatabaseEntity[];
    relationships: DatabaseRelationship[];
}

interface InterfaceField {
    name: string;
    type: string;
    required: boolean;
    isArray: boolean;
    nested?: InterfaceEntity;
}

interface InterfaceEntity {
    name: string;
    fields: InterfaceField[];
}

interface InterfaceReport {
    timestamp: string;
    rootEntity: InterfaceEntity;
}

interface ClassField {
    name: string;
    type: string;
}

interface ClassEntity {
    name: string;
    kind: "class" | "interface" | "enum";
    fields: ClassField[];
    enumValues?: string[];
}

interface ClassRelationship {
    from: string;
    to: string;
    type: string;
    cardinality: string;
}

interface SoftwareReport {
    timestamp: string;
    entities: ClassEntity[];
    relationships: ClassRelationship[];
}

// #endregion Types

// Known main entities (non-weak, have guid)
const MAIN_ENTITIES = [
    "Attribute",
    "Location",
    "Author",
    "File",
    "Folder",
    "Benchmark",
    "Quality",
    "Interface",
    "Prop",
    "Model",
    "Port",
    "Tag",
    "Concept",
    "Type",
    "Layer",
    "Piece",
    "Group",
    "Connection",
    "Stat",
    "Design",
    "Kit",
];

// Weak entities (no guid)
const WEAK_ENTITIES = ["Coord", "Vec", "Point", "Vector", "Plane", "Range", "Side"];

// #region TypeScript Parser

function parseTypeScriptSchema(): SchemaReport {
    const content = readFileSync(semioTsPath, "utf-8");
    const entities: Entity[] = [];
    const idTypes: string[] = [];
    const weakEntities: string[] = [];

    // Extract entity ID types
    const idTypeRegex = /export type (\w+Id) = \{ guid: Guid \}/g;
    let match;
    while ((match = idTypeRegex.exec(content)) !== null) {
        idTypes.push(match[1]);
    }

    // Extract schema definitions by region markers
    for (const entityName of [...MAIN_ENTITIES, ...WEAK_ENTITIES]) {
        // Look for the Schema definition
        const schemaRegex = new RegExp(
            `export const ${entityName}Schema = z\\.object\\(\\{([\\s\\S]*?)\\}\\)`,
            "m"
        );
        const schemaMatch = content.match(schemaRegex);

        if (schemaMatch) {
            const fieldsStr = schemaMatch[1];
            const fields = parseZodFields(fieldsStr);
            const hasDiff = content.includes(`export const ${entityName}DiffSchema`);
            const hasDiffs = content.includes(`export const ${entityName}sDiffSchema`);
            const hasId = idTypes.includes(`${entityName}Id`);

            const isWeak = WEAK_ENTITIES.includes(entityName);
            if (isWeak) {
                weakEntities.push(entityName);
            }

            entities.push({ name: entityName, fields, hasId, hasDiff, hasDiffs });
        }
    }

    return {
        timestamp: new Date().toISOString(),
        entities,
        idTypes,
        weakEntities,
    };
}

function parseZodFields(fieldsStr: string): EntityField[] {
    const fields: EntityField[] = [];
    const lines = fieldsStr.split("\n");
    for (const line of lines) {
        // Match: fieldName: z.something()
        const fieldMatch = line.match(/^\s*(\w+):\s*(.+?),?\s*$/);
        if (fieldMatch) {
            const name = fieldMatch[1];
            const typeStr = fieldMatch[2];
            const optional = typeStr.includes(".optional()");
            const type = inferZodType(typeStr);
            fields.push({ name, type, optional });
        }
    }
    return fields;
}

function inferZodType(zodStr: string): string {
    if (zodStr.includes("z.string()")) return "string";
    if (zodStr.includes("z.number()")) return "number";
    if (zodStr.includes("z.boolean()")) return "boolean";
    if (zodStr.includes("z.array(")) {
        const innerMatch = zodStr.match(/z\.array\((\w+)Schema/);
        if (innerMatch) return `${innerMatch[1]}[]`;
        return "array";
    }
    const schemaMatch = zodStr.match(/(\w+)Schema/);
    if (schemaMatch) return schemaMatch[1];
    return "unknown";
}

// #endregion TypeScript Parser

// #region Python Parser

function parsePythonSchema(): SchemaReport {
    const content = readFileSync(enginePyPath, "utf-8");
    const entities: Entity[] = [];
    const idTypes: string[] = [];
    const weakEntities: string[] = [];

    // Find Id classes
    const idClassRegex = /class (\w+Id)\([^)]*\):/g;
    let match;
    while ((match = idClassRegex.exec(content)) !== null) {
        if (!match[1].endsWith("IdId")) {
            idTypes.push(match[1]);
        }
    }

    // Parse main entities - look for table classes and their field classes
    for (const entityName of MAIN_ENTITIES) {
        const fields: EntityField[] = [];

        // Look for field classes like AttributeKeyField, AttributeValueField
        const fieldClassRegex = new RegExp(
            `class ${entityName}\\w*Field\\([^)]*\\):([\\s\\S]*?)(?=\\nclass |\\n# )`,
            "gm"
        );
        while ((match = fieldClassRegex.exec(content)) !== null) {
            const classBody = match[1];
            const classFields = parsePythonSqlmodelFields(classBody);
            for (const f of classFields) {
                if (!fields.some((ef) => ef.name === f.name)) {
                    fields.push(f);
                }
            }
        }

        // Look for the Props class
        const propsRegex = new RegExp(
            `class ${entityName}Props\\([^)]*\\):([\\s\\S]*?)(?=\\nclass |\\n# )`,
            "m"
        );
        const propsMatch = content.match(propsRegex);
        if (propsMatch) {
            const propsFields = parsePythonSqlmodelFields(propsMatch[1]);
            for (const f of propsFields) {
                if (!fields.some((ef) => ef.name === f.name)) {
                    fields.push(f);
                }
            }
        }

        // Check if main class exists
        const mainClassRegex = new RegExp(`class ${entityName}\\([^)]*(?:TableEntity|table=True)[^)]*\\):`, "m");
        if (mainClassRegex.test(content)) {
            // All main entities have guid
            if (!fields.some((f) => f.name === "guid")) {
                fields.unshift({ name: "guid", type: "string", optional: false });
            }

            const hasDiff = content.includes(`class ${entityName}Diff`);
            const hasDiffs = content.includes(`class ${entityName}sDiff`);
            const hasId = idTypes.includes(`${entityName}Id`);

            entities.push({ name: entityName, fields, hasId, hasDiff, hasDiffs });
        }
    }

    // Parse weak entities
    for (const weakName of WEAK_ENTITIES) {
        const classRegex = new RegExp(`class ${weakName}\\([^)]*\\):([\\s\\S]*?)(?=\\nclass |\\n# )`, "m");
        const classMatch = content.match(classRegex);
        if (classMatch) {
            const fields = parsePythonSqlmodelFields(classMatch[1]);
            weakEntities.push(weakName);
            entities.push({ name: weakName, fields, hasId: false, hasDiff: false, hasDiffs: false });
        }
    }

    return {
        timestamp: new Date().toISOString(),
        entities,
        idTypes,
        weakEntities,
    };
}

function parsePythonSqlmodelFields(classBody: string): EntityField[] {
    const fields: EntityField[] = [];
    // Match: name: type = sqlmodel.Field(...) or name: type
    const fieldRegex = /^\s{4}(\w+):\s*([^=\n]+?)(?:\s*=.*)?$/gm;
    let match;
    while ((match = fieldRegex.exec(classBody)) !== null) {
        const name = match[1];
        if (name.startsWith("_") || name === "PLURAL" || name === "pk" || name === "Meta") continue;
        const typeStr = match[2].trim();
        const optional =
            typeStr.includes("Optional") || typeStr.includes("| None") || typeStr.includes("None |");
        const type = inferPythonType(typeStr);
        fields.push({ name, type, optional });
    }
    return fields;
}

function inferPythonType(typeStr: string): string {
    let type = typeStr
        .replace(/typing\.Optional\[([^\]]+)\]/, "$1")
        .replace(/Optional\[([^\]]+)\]/, "$1")
        .replace(/\s*\|\s*None/, "")
        .replace(/None\s*\|/, "")
        .trim();

    if (type === "str") return "string";
    if (type === "int" || type === "float") return "number";
    if (type === "bool") return "boolean";
    if (type.match(/(?:typing\.)?[Ll]ist\[/)) {
        const inner = type.match(/(?:typing\.)?[Ll]ist\[([^\]]+)\]/);
        if (inner) return `${inferPythonType(inner[1])}[]`;
    }
    return type;
}

// #endregion Python Parser

// #region C# Parser

function parseCSharpSchema(): SchemaReport {
    const content = readFileSync(semioCsPath, "utf-8");
    const entities: Entity[] = [];
    const idTypes: string[] = [];
    const weakEntities: string[] = [];

    // Find Id classes
    const idClassRegex = /public\s+class\s+(\w+Id)\s*:/g;
    let match;
    while ((match = idClassRegex.exec(content)) !== null) {
        idTypes.push(match[1]);
    }

    // Parse main entities
    for (const entityName of MAIN_ENTITIES) {
        // Match the class and its body until the closing brace at the same level
        const classRegex = new RegExp(
            `\\[Model\\([^\\]]+\\)\\]\\s*public\\s+class\\s+${entityName}\\s*:[^{]*\\{`,
            "m"
        );
        const classStart = content.match(classRegex);

        if (classStart && classStart.index !== undefined) {
            const startIdx = classStart.index + classStart[0].length;
            const body = extractClassBody(content, startIdx);
            const fields = parseCSharpFields(body);

            const hasDiff =
                content.includes(`public class ${entityName}Diff`) ||
                content.includes(`public class ${entityName}Diff :`);
            const hasDiffs =
                content.includes(`public class ${entityName}sDiff`) ||
                content.includes(`public class ${entityName}sDiff :`);
            const hasId = idTypes.includes(`${entityName}Id`);

            entities.push({ name: entityName, fields, hasId, hasDiff, hasDiffs });
        }
    }

    // Parse weak entities
    for (const weakName of WEAK_ENTITIES) {
        const classRegex = new RegExp(`public\\s+class\\s+${weakName}\\s*:[^{]*\\{`, "m");
        const classStart = content.match(classRegex);

        if (classStart && classStart.index !== undefined) {
            const startIdx = classStart.index + classStart[0].length;
            const body = extractClassBody(content, startIdx);
            const fields = parseCSharpFields(body);
            weakEntities.push(weakName);
            entities.push({ name: weakName, fields, hasId: false, hasDiff: false, hasDiffs: false });
        }
    }

    return {
        timestamp: new Date().toISOString(),
        entities,
        idTypes,
        weakEntities,
    };
}

function extractClassBody(content: string, startIdx: number): string {
    let depth = 1;
    let endIdx = startIdx;
    while (depth > 0 && endIdx < content.length) {
        if (content[endIdx] === "{") depth++;
        if (content[endIdx] === "}") depth--;
        endIdx++;
    }
    return content.substring(startIdx, endIdx - 1);
}

function parseCSharpFields(body: string): EntityField[] {
    const fields: EntityField[] = [];
    // Match: public Type? Name { get; set; }
    const propRegex = /public\s+([\w<>]+)\??\s+(\w+)\s*\{\s*get;\s*set;\s*\}/g;
    let match;
    while ((match = propRegex.exec(body)) !== null) {
        let typeStr = match[1];
        const name = match[2];
        const fullMatch = match[0];
        const optional = fullMatch.includes(typeStr + "?") || body.includes(`${typeStr}? ${name}`);
        typeStr = typeStr.replace(/\?$/, "");
        const type = inferCSharpType(typeStr);
        fields.push({ name, type, optional });
    }
    return fields;
}

function inferCSharpType(typeStr: string): string {
    if (typeStr === "string") return "string";
    if (typeStr === "int" || typeStr === "float" || typeStr === "double") return "number";
    if (typeStr === "bool") return "boolean";
    if (typeStr.startsWith("List<")) {
        const inner = typeStr.match(/List<([^>]+)>/);
        if (inner) return `${inferCSharpType(inner[1])}[]`;
    }
    return typeStr;
}

// #endregion C# Parser

// #region Grasshopper Parser

function parseGrasshopperSchema(): GrasshopperReport {
    const content = readFileSync(grasshopperCsPath, "utf-8");
    const components: GrasshopperComponent[] = [];
    const params: string[] = [];
    const goos: string[] = [];

    // Extract Param classes
    const paramRegex = /public\s+class\s+(\w+Param)\s*:/g;
    let match;
    while ((match = paramRegex.exec(content)) !== null) {
        params.push(match[1]);
    }

    // Extract Goo classes
    const gooRegex = /public\s+class\s+(\w+Goo)\s*:/g;
    while ((match = gooRegex.exec(content)) !== null) {
        goos.push(match[1]);
    }

    // Extract Component classes
    const componentRegex =
        /public\s+class\s+(\w+)Component\s*:[^{]+ModelComponent[^{]*\{[\s\S]*?ModelName\s*=>\s*"(\w+)"[\s\S]*?ModelNickname\s*=>\s*"(\w+)"/g;
    while ((match = componentRegex.exec(content)) !== null) {
        const className = match[1] + "Component";
        const modelName = match[2];
        const nickname = match[3];

        // Extract inputs and outputs from the component
        const inputs = extractGrasshopperParams(content, className, "Input");
        const outputs = extractGrasshopperParams(content, className, "Output");

        components.push({
            name: modelName,
            nickname,
            inputs,
            outputs,
        });
    }

    return {
        timestamp: new Date().toISOString(),
        components,
        params,
        goos,
    };
}

function extractGrasshopperParams(
    content: string,
    className: string,
    paramType: "Input" | "Output"
): EntityField[] {
    const fields: EntityField[] = [];

    // Find the class definition
    const classRegex = new RegExp(`class\\s+${className}[^{]*\\{([\\s\\S]*?)\\n\\}\\s*\\n`, "m");
    const classMatch = content.match(classRegex);
    if (!classMatch) return fields;

    const classBody = classMatch[1];

    // Find RegisterModel*Params method
    const methodRegex = new RegExp(
        `RegisterModel${paramType}Params[^{]*\\{([\\s\\S]*?)\\n    \\}`,
        "m"
    );
    const methodMatch = classBody.match(methodRegex);
    if (!methodMatch) return fields;

    const methodBody = methodMatch[1];

    // Extract AddTextParameter
    const textParamRegex = /AddTextParameter\s*\(\s*"([^"]+)",\s*"([^"]+)",\s*"([^"]+)"/g;
    let match;
    while ((match = textParamRegex.exec(methodBody)) !== null) {
        const name = match[1];
        const nickname = match[2];
        const description = match[3];
        const optional = nickname.includes("?");
        fields.push({ name, type: "string", optional, description });
    }

    // Extract AddBooleanParameter
    const boolParamRegex = /AddBooleanParameter\s*\(\s*"([^"]+)",\s*"([^"]+)",\s*"([^"]+)"/g;
    while ((match = boolParamRegex.exec(methodBody)) !== null) {
        const name = match[1];
        const nickname = match[2];
        const description = match[3];
        const optional = nickname.includes("?");
        fields.push({ name, type: "boolean", optional, description });
    }

    // Extract AddParameter with typed params
    const typedParamRegex =
        /AddParameter\s*\(\s*new\s+(\w+)Param\(\)[^,]*,\s*"([^"]+)",\s*"([^"]+)",\s*"([^"]+)"/g;
    while ((match = typedParamRegex.exec(methodBody)) !== null) {
        const paramType = match[1];
        const name = match[2];
        const nickname = match[3];
        const description = match[4];
        const optional = nickname.includes("?") || nickname.includes("*");
        fields.push({ name, type: paramType, optional, description });
    }

    return fields;
}

// #endregion Grasshopper Parser

// #region Database Architecture Parser

function parseDatabaseArchitecture(): DatabaseReport {
    const content = readFileSync(dataArchPath, "utf-8");
    const entities: DatabaseEntity[] = [];
    const relationships: DatabaseRelationship[] = [];

    // Parse entities
    const entityRegex = /entity\s+(\w+)\s*\{([^}]+)\}/g;
    let match;
    while ((match = entityRegex.exec(content)) !== null) {
        const entityName = match[1];
        const fieldsStr = match[2];
        const fields = parseDatabaseFields(fieldsStr);
        entities.push({ name: entityName, fields });
    }

    // Parse relationships
    const relRegex = /(\w+)\s+([\|\}o][\|\-][o\|\-\{][\|\-]o[\|\-]?\{?)\s+(\w+)\s*:\s*"([^"]+)"/g;
    while ((match = relRegex.exec(content)) !== null) {
        const from = match[1];
        const cardinality = match[2];
        const to = match[3];
        const label = match[4];
        relationships.push({ from, to, cardinality, label });
    }

    return {
        timestamp: new Date().toISOString(),
        entities,
        relationships,
    };
}

function parseDatabaseFields(fieldsStr: string): DatabaseField[] {
    const fields: DatabaseField[] = [];
    const lines = fieldsStr.split("\n");

    for (const line of lines) {
        // Match: *name : type <<constraints>> or name : type
        const fieldMatch = line.match(/^\s*(\*?)(\w+)\s*:\s*([^\s<]+)(?:\s*<<([^>]+)>>)?/);
        if (fieldMatch) {
            const isRequired = fieldMatch[1] === "*";
            const name = fieldMatch[2];
            const type = fieldMatch[3];
            const constraintStr = fieldMatch[4] || "";

            const constraints: string[] = [];
            if (isRequired) constraints.push("NOT NULL");
            if (constraintStr) {
                constraints.push(...constraintStr.split(",").map((c) => c.trim()));
            }

            fields.push({ name, type, constraints });
        }
    }

    return fields;
}

// #endregion Database Architecture Parser

// #region Interface Architecture Parser

function parseInterfaceArchitecture(): InterfaceReport {
    const content = readFileSync(interfaceArchPath, "utf-8");
    const rootEntity = parseInterfaceEntity(content, 0);

    return {
        timestamp: new Date().toISOString(),
        rootEntity,
    };
}

function parseInterfaceEntity(content: string, startIndex: number): InterfaceEntity {
    const lines = content.split("\n");
    const fields: InterfaceField[] = [];
    let entityName = "Kit";

    for (let i = startIndex; i < lines.length; i++) {
        const line = lines[i];
        if (!line.trim() || line.trim().startsWith("//")) continue;

        // Match field definitions like: name : !Type or name : ?Type[ or name : *Type[
        const fieldMatch = line.match(/^\s{4}(\w+)\s*:\s*(!|\?|\*|\+)(\w+)(\[)?/);
        if (fieldMatch) {
            const name = fieldMatch[1];
            const requiredMarker = fieldMatch[2];
            const typeName = fieldMatch[3];
            const isArray = fieldMatch[4] !== undefined;
            const required = requiredMarker === "!" || requiredMarker === "+";

            fields.push({
                name,
                type: typeName,
                required,
                isArray,
            });
        }

        // Extract nested entity name from first line
        if (i === startIndex) {
            const nameMatch = line.match(/^(\w+)\s*:/);
            if (nameMatch) {
                entityName = nameMatch[1];
            }
        }
    }

    return { name: entityName, fields };
}

// #endregion Interface Architecture Parser

// #region Software Architecture Parser

function parseSoftwareArchitecture(): SoftwareReport {
    const content = readFileSync(softwareArchPath, "utf-8");
    const entities: ClassEntity[] = [];
    const relationships: ClassRelationship[] = [];

    // Parse classes
    const classRegex = /(?:class|interface)\s+(\w+)\s*\{([^}]+)\}/g;
    let match;
    while ((match = classRegex.exec(content)) !== null) {
        const name = match[1];
        const kind = content.substring(match.index - 10, match.index).includes("interface")
            ? "interface"
            : "class";
        const fieldsStr = match[2];
        const fields = parseSoftwareFields(fieldsStr);
        entities.push({ name, kind, fields });
    }

    // Parse enums
    const enumRegex = /class\s+(\w+)\s*\{[^}]*<<enumeration>>[^}]*\}/g;
    while ((match = enumRegex.exec(content)) !== null) {
        const name = match[1];
        const enumValues = match[0].match(/^\s+(\w+)\s*$/gm)?.map((v) => v.trim()) || [];
        entities.push({ name, kind: "enum", fields: [], enumValues });
    }

    // Parse relationships
    const relRegex = /(\w+)\s+([\*o\|\-\.\>]+)\s+"([^"]+)"\s+(\w+)/g;
    while ((match = relRegex.exec(content)) !== null) {
        const from = match[1];
        const cardinalityAndType = match[2];
        const label = match[3];
        const to = match[4];

        let type = "association";
        if (cardinalityAndType.includes("*--")) type = "composition";
        if (cardinalityAndType.includes("o--")) type = "aggregation";
        if (cardinalityAndType.includes("-|>")) type = "inheritance";
        if (cardinalityAndType.includes("..>")) type = "dependency";

        relationships.push({ from, to, type, cardinality: label });
    }

    return {
        timestamp: new Date().toISOString(),
        entities,
        relationships,
    };
}

function parseSoftwareFields(fieldsStr: string): ClassField[] {
    const fields: ClassField[] = [];
    const lines = fieldsStr.split("\n");

    for (const line of lines) {
        // Match: name: type or name?: type
        const fieldMatch = line.match(/^\s*(\w+)\??:\s*([^\s]+)/);
        if (fieldMatch && !line.includes("<<") && !line.includes("context")) {
            const name = fieldMatch[1];
            const type = fieldMatch[2];
            fields.push({ name, type });
        }
    }

    return fields;
}

// #endregion Software Architecture Parser

// #region Comparison

// Field name mappings: TypeScript field name -> equivalent names in other languages
const FIELD_MAPPINGS: Record<string, Record<string, string[]>> = {
    Attribute: { key: ["name", "key"] }, // TS 'key' = PY 'name'
    Quality: {
        canScale: ["scalable", "can_scale", "Scalable"],
        defaultSiUnit: ["si", "default_si_unit", "SI"],
        defaultImperialUnit: ["imperial", "default_imperial_unit", "Imperial"],
        isMinExcluded: ["min_excluded", "minExcluded", "MinExcluded"],
        isMaxExcluded: ["max_excluded", "maxExcluded", "MaxExcluded"],
        defaultValue: ["default", "default_value", "Default"],
    },
    Layer: {
        path: ["name", "path", "Path", "Name"], // Layer path might be called name in some implementations
        isHidden: ["is_hidden", "hidden", "IsHidden"],
        isLocked: ["is_locked", "locked", "IsLocked"],
    },
    Port: {
        mandatory: ["is_mandatory", "mandatory", "Mandatory"],
        name: ["id_", "name", "Name"], // TS 'name' = PY 'id_'
    },
    Piece: {
        name: ["id_", "name", "Name"], // TS 'name' = PY 'id_'
        design: ["designPiece", "design_piece", "design", "Design"],
    },
    Prop: {
        quality: ["key", "quality_key", "qualityKey", "quality", "Quality"],
    },
    Stat: {
        quality: ["key", "quality_key", "qualityKey", "quality", "Quality"],
    },
    Model: {
        name: ["id_", "name", "Name"],
    },
    Tag: {
        name: ["id_", "name", "Name"],
    },
    Concept: {
        name: ["id_", "name", "Name"],
    },
    Type: {
        virtual: ["is_virtual", "virtual", "Virtual"],
    },
};

// Fields to skip in comparison (timestamps, internal fields)
const SKIP_FIELDS = ["createdAt", "createdBy", "updatedAt", "updatedBy", "pk"];

// Fields that are relationships in Python (stored differently via SQLModel Relationships)
const RELATIONSHIP_FIELDS: Record<string, string[]> = {
    // All entities with attributes have them as relationships in Python
    Location: ["attributes"],
    Folder: ["attributes"],
    Benchmark: ["attributes"],
    Quality: ["benchmarks", "attributes"],
    Interface: ["compatibleInterfaces", "attributes"],
    Prop: ["attributes"],
    Model: ["tags", "attributes"],
    Port: ["props", "attributes"],
    Tag: ["attributes"],
    Concept: ["attributes"],
    Type: ["models", "ports", "props", "authors", "concepts", "attributes"],
    Layer: ["attributes"],
    Piece: ["props", "attributes"],
    Group: ["pieces", "attributes"],
    Connection: ["attributes"],
    Stat: [],
    Design: ["pieces", "connections", "layers", "groups", "stats", "props", "authors", "concepts", "attributes"],
    Kit: ["types", "designs", "tags", "concepts", "interfaces", "qualities", "files", "folders", "authors", "attributes"],
};

function compareSchemas(
    ts: SchemaReport,
    py: SchemaReport,
    cs: SchemaReport,
    gh: GrasshopperReport
): Issue[] {
    const issues: Issue[] = [];

    // TypeScript is source of truth
    for (const tsEntity of ts.entities) {
        // Skip weak entities for comparison
        if (ts.weakEntities.includes(tsEntity.name)) continue;

        // Check Python
        const pyEntity = py.entities.find((e) => e.name === tsEntity.name);
        if (!pyEntity) {
            issues.push({
                severity: "error",
                entity: tsEntity.name,
                message: `Missing entity in Python`,
                source: "py",
            });
        } else {
            // Compare fields
            for (const tsField of tsEntity.fields) {
                // Skip timestamp and internal fields
                if (SKIP_FIELDS.includes(tsField.name)) continue;

                // Skip relationship fields (they're defined via SQLAlchemy relationships)
                const relFields = RELATIONSHIP_FIELDS[tsEntity.name] || [];
                if (relFields.includes(tsField.name)) continue;

                const pyFieldName = toSnakeCase(tsField.name);
                const mappings = FIELD_MAPPINGS[tsEntity.name]?.[tsField.name] || [tsField.name, pyFieldName];

                const pyField = pyEntity.fields.find(
                    (f) => mappings.includes(f.name) || f.name === pyFieldName
                );
                if (!pyField) {
                    issues.push({
                        severity: "error",
                        entity: tsEntity.name,
                        field: tsField.name,
                        message: `Missing field in Python (expected: ${pyFieldName})`,
                        source: "py",
                    });
                }
            }
        }

        // Check C#
        const csEntity = cs.entities.find((e) => e.name === tsEntity.name);
        if (!csEntity) {
            issues.push({
                severity: "error",
                entity: tsEntity.name,
                message: `Missing entity in C#`,
                source: "net",
            });
        } else {
            // Compare fields
            for (const tsField of tsEntity.fields) {
                // Skip timestamp and internal fields
                if (SKIP_FIELDS.includes(tsField.name)) continue;

                const csFieldName = toPascalCase(tsField.name);
                const mappings = FIELD_MAPPINGS[tsEntity.name]?.[tsField.name] || [tsField.name, csFieldName];

                const csField = csEntity.fields.find(
                    (f) => mappings.some(m => m.toLowerCase() === f.name.toLowerCase()) ||
                        f.name.toLowerCase() === tsField.name.toLowerCase()
                );
                if (!csField) {
                    issues.push({
                        severity: "error",
                        entity: tsEntity.name,
                        field: tsField.name,
                        message: `Missing field in C# (expected: ${csFieldName})`,
                        source: "net",
                    });
                }
            }
        }

        // Check Grasshopper components
        const ghComponent = gh.components.find(
            (c) => c.name === tsEntity.name || c.name === toPascalCase(tsEntity.name)
        );
        if (!ghComponent) {
            issues.push({
                severity: "warning",
                entity: tsEntity.name,
                message: `Missing Grasshopper component`,
                source: "grasshopper",
            });
        }
    }

    // Check for Id types
    for (const tsId of ts.idTypes) {
        const entityName = tsId.replace("Id", "");
        if (!py.idTypes.includes(tsId)) {
            issues.push({
                severity: "warning",
                entity: entityName,
                message: `Missing ${tsId} in Python`,
                source: "py",
            });
        }
        if (!cs.idTypes.includes(tsId)) {
            issues.push({
                severity: "warning",
                entity: entityName,
                message: `Missing ${tsId} in C#`,
                source: "net",
            });
        }
    }

    // Check Grasshopper Param/Goo coverage
    for (const entityName of MAIN_ENTITIES) {
        if (!gh.params.includes(`${entityName}Param`)) {
            issues.push({
                severity: "warning",
                entity: entityName,
                message: `Missing ${entityName}Param in Grasshopper`,
                source: "grasshopper",
            });
        }
        if (!gh.goos.includes(`${entityName}Goo`)) {
            issues.push({
                severity: "warning",
                entity: entityName,
                message: `Missing ${entityName}Goo in Grasshopper`,
                source: "grasshopper",
            });
        }
    }

    return issues;
}

function toSnakeCase(str: string): string {
    return str.replace(/([A-Z])/g, "_$1").toLowerCase().replace(/^_/, "");
}

function toPascalCase(str: string): string {
    return str.charAt(0).toUpperCase() + str.slice(1);
}

// #endregion Comparison

// #region Main

console.log("📊 Extracting schemas...\n");

const tsReport = parseTypeScriptSchema();
console.log(`TypeScript: ${tsReport.entities.length} entities, ${tsReport.idTypes.length} ID types`);

const pyReport = parsePythonSchema();
console.log(`Python: ${pyReport.entities.length} entities, ${pyReport.idTypes.length} ID types`);

const csReport = parseCSharpSchema();
console.log(`C#: ${csReport.entities.length} entities, ${csReport.idTypes.length} ID types`);

const ghReport = parseGrasshopperSchema();
console.log(
    `Grasshopper: ${ghReport.components.length} components, ${ghReport.params.length} params, ${ghReport.goos.length} goos`
);

const dbReport = parseDatabaseArchitecture();
console.log(
    `Database Architecture: ${dbReport.entities.length} entities, ${dbReport.relationships.length} relationships`
);

const ifaceReport = parseInterfaceArchitecture();
console.log(`Interface Architecture: ${ifaceReport.rootEntity.fields.length} top-level fields`);

const swReport = parseSoftwareArchitecture();
console.log(
    `Software Architecture: ${swReport.entities.length} entities, ${swReport.relationships.length} relationships`
);

// Write individual reports
writeFileSync(join(reportsDir, "schema-ts.json"), JSON.stringify(tsReport, null, 2));
writeFileSync(join(reportsDir, "schema-py.json"), JSON.stringify(pyReport, null, 2));
writeFileSync(join(reportsDir, "schema-net.json"), JSON.stringify(csReport, null, 2));
writeFileSync(join(reportsDir, "schema-grasshopper.json"), JSON.stringify(ghReport, null, 2));
writeFileSync(join(reportsDir, "schema-database.json"), JSON.stringify(dbReport, null, 2));
writeFileSync(join(reportsDir, "schema-interface.json"), JSON.stringify(ifaceReport, null, 2));
writeFileSync(join(reportsDir, "schema-software.json"), JSON.stringify(swReport, null, 2));

console.log("\n📝 Reports written to reports/schema-*.json");

// Compare and generate issues
const issues = compareSchemas(tsReport, pyReport, csReport, ghReport);
const errors = issues.filter((i) => i.severity === "error");
const warnings = issues.filter((i) => i.severity === "warning");

const summaryReport = {
    timestamp: new Date().toISOString(),
    summary: {
        typescript: { entities: tsReport.entities.length, idTypes: tsReport.idTypes.length },
        python: { entities: pyReport.entities.length, idTypes: pyReport.idTypes.length },
        csharp: { entities: csReport.entities.length, idTypes: csReport.idTypes.length },
        grasshopper: {
            components: ghReport.components.length,
            params: ghReport.params.length,
            goos: ghReport.goos.length,
        },
        database: {
            entities: dbReport.entities.length,
            relationships: dbReport.relationships.length,
        },
        interface: {
            topLevelFields: ifaceReport.rootEntity.fields.length,
        },
        software: {
            entities: swReport.entities.length,
            relationships: swReport.relationships.length,
        },
        errors: errors.length,
        warnings: warnings.length,
    },
    errors,
    warnings,
    status: errors.length > 0 ? "error" : warnings.length > 0 ? "warning" : "success",
};

writeFileSync(join(reportsDir, "schema.json"), JSON.stringify(summaryReport, null, 2));

console.log(`\n${errors.length} errors, ${warnings.length} warnings`);

if (errors.length > 0) {
    console.log("\n❌ Errors:");
    for (const err of errors.slice(0, 20)) {
        console.log(`  - [${err.source}] ${err.entity}${err.field ? "." + err.field : ""}: ${err.message}`);
    }
    if (errors.length > 20) {
        console.log(`  ... and ${errors.length - 20} more errors`);
    }
}

if (warnings.length > 0) {
    console.log("\n⚠️  Warnings:");
    for (const warn of warnings.slice(0, 10)) {
        console.log(`  - [${warn.source}] ${warn.entity}${warn.field ? "." + warn.field : ""}: ${warn.message}`);
    }
    if (warnings.length > 10) {
        console.log(`  ... and ${warnings.length - 10} more warnings`);
    }
}

process.exit(errors.length > 0 ? 1 : 0);

// #endregion Main
