// #region Header

// functions.ts

// 2025 Ueli Saluz

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion

/**
 * Function definition for quality formula builder
 * Each function defines how to calculate values and how to render in LaTeX
 */
export interface FormulaFunction {
  name: string;
  category: "numeric" | "branching" | "data" | "text" | "comparison";
  arity: number | "variadic"; // number of operands or "variadic" for any number
  icon?: string;
  description: string;
  /** Calculate the result based on operand values */
  calculate: (...operands: any[]) => any;
  /** Render the function in LaTeX notation */
  toLatex: (...operands: string[]) => string;
}

/**
 * Registry of all available formula functions
 */
export const formulaFunctions: Record<string, FormulaFunction> = {
  // Numeric Functions
  Add: {
    name: "Add",
    category: "numeric",
    arity: "variadic",
    icon: "plus",
    description: "Add two or more numbers",
    calculate: (...operands: number[]) => operands.reduce((sum, val) => sum + val, 0),
    toLatex: (...operands: string[]) => operands.join(" + "),
  },
  Subtract: {
    name: "Subtract",
    category: "numeric",
    arity: 2,
    icon: "minus",
    description: "Subtract second number from first",
    calculate: (a: number, b: number) => a - b,
    toLatex: (a: string, b: string) => `${a} - ${b}`,
  },
  Multiply: {
    name: "Multiply",
    category: "numeric",
    arity: "variadic",
    icon: "times",
    description: "Multiply two or more numbers",
    calculate: (...operands: number[]) => operands.reduce((product, val) => product * val, 1),
    toLatex: (...operands: string[]) => operands.join(" \\times "),
  },
  Divide: {
    name: "Divide",
    category: "numeric",
    arity: 2,
    icon: "divide",
    description: "Divide first number by second",
    calculate: (a: number, b: number) => (b !== 0 ? a / b : NaN),
    toLatex: (a: string, b: string) => `\\frac{${a}}{${b}}`,
  },
  Power: {
    name: "Power",
    category: "numeric",
    arity: 2,
    icon: "superscript",
    description: "Raise first number to the power of second",
    calculate: (a: number, b: number) => Math.pow(a, b),
    toLatex: (a: string, b: string) => `{${a}}^{${b}}`,
  },
  Sqrt: {
    name: "Sqrt",
    category: "numeric",
    arity: 1,
    icon: "square-root",
    description: "Calculate square root",
    calculate: (a: number) => Math.sqrt(a),
    toLatex: (a: string) => `\\sqrt{${a}}`,
  },

  // Comparison Functions
  Smaller: {
    name: "Smaller",
    category: "comparison",
    arity: 2,
    icon: "less-than",
    description: "Check if first value is smaller than second",
    calculate: (a: any, b: any) => a < b,
    toLatex: (a: string, b: string) => `${a} < ${b}`,
  },
  Greater: {
    name: "Greater",
    category: "comparison",
    arity: 2,
    icon: "greater-than",
    description: "Check if first value is greater than second",
    calculate: (a: any, b: any) => a > b,
    toLatex: (a: string, b: string) => `${a} > ${b}`,
  },
  Equal: {
    name: "Equal",
    category: "comparison",
    arity: 2,
    icon: "equals",
    description: "Check if two values are equal",
    calculate: (a: any, b: any) => a === b,
    toLatex: (a: string, b: string) => `${a} = ${b}`,
  },

  // Branching Functions
  If: {
    name: "If",
    category: "branching",
    arity: 3,
    icon: "question",
    description: "If condition is true, return first value, else return second",
    calculate: (condition: boolean, thenValue: any, elseValue: any) => (condition ? thenValue : elseValue),
    toLatex: (condition: string, thenValue: string, elseValue: string) => `\\text{if } ${condition} \\text{ then } ${thenValue} \\text{ else } ${elseValue}`,
  },
  Switch: {
    name: "Switch",
    category: "branching",
    arity: "variadic",
    icon: "switch",
    description: "Match value against cases and return corresponding result",
    calculate: (value: any, ...cases: any[]) => {
      // Cases should be pairs of [match, result]
      for (let i = 0; i < cases.length - 1; i += 2) {
        if (value === cases[i]) return cases[i + 1];
      }
      // Default case is the last element if odd number of cases
      return cases.length % 2 === 1 ? cases[cases.length - 1] : undefined;
    },
    toLatex: (value: string, ...cases: string[]) => {
      const casesLatex = [];
      for (let i = 0; i < cases.length - 1; i += 2) {
        casesLatex.push(`${cases[i]} \\rightarrow ${cases[i + 1]}`);
      }
      if (cases.length % 2 === 1) {
        casesLatex.push(`\\text{default} \\rightarrow ${cases[cases.length - 1]}`);
      }
      return `\\text{switch}(${value}) \\{ ${casesLatex.join(", ")} \\}`;
    },
  },

  // Text Functions
  StartsWith: {
    name: "StartsWith",
    category: "text",
    arity: 2,
    icon: "text",
    description: "Check if string starts with prefix",
    calculate: (str: string, prefix: string) => str.startsWith(prefix),
    toLatex: (str: string, prefix: string) => `\\text{StartsWith}(${str}, ${prefix})`,
  },
  Name: {
    name: "Name",
    category: "text",
    arity: 1,
    icon: "tag",
    description: "Get the name of an entity",
    calculate: (entity: any) => entity?.name || "",
    toLatex: (entity: string) => `\\text{Name}(${entity})`,
  },

  // Data Structure Functions
  List: {
    name: "List",
    category: "data",
    arity: "variadic",
    icon: "list",
    description: "Create a list of values",
    calculate: (...values: any[]) => values,
    toLatex: (...values: string[]) => `[${values.join(", ")}]`,
  },
  Dictionary: {
    name: "Dictionary",
    category: "data",
    arity: "variadic",
    icon: "book",
    description: "Create a dictionary from key-value pairs",
    calculate: (...pairs: any[]) => {
      const dict: Record<string, any> = {};
      for (const pair of pairs) {
        if (pair && typeof pair === "object" && "key" in pair && "value" in pair) {
          dict[pair.key] = pair.value;
        }
      }
      return dict;
    },
    toLatex: (...pairs: string[]) => `\\{${pairs.join(", ")}\\}`,
  },
  KeyValuePair: {
    name: "KeyValuePair",
    category: "data",
    arity: 2,
    icon: "key",
    description: "Create a key-value pair for a dictionary",
    calculate: (key: any, value: any) => ({ key, value }),
    toLatex: (key: string, value: string) => `${key}: ${value}`,
  },
  Key: {
    name: "Key",
    category: "data",
    arity: 1,
    icon: "key",
    description: "Extract key from a key-value pair",
    calculate: (pair: any) => pair?.key,
    toLatex: (pair: string) => `\\text{Key}(${pair})`,
  },
  Value: {
    name: "Value",
    category: "data",
    arity: 1,
    icon: "value",
    description: "Extract value from a key-value pair",
    calculate: (pair: any) => pair?.value,
    toLatex: (pair: string) => `\\text{Value}(${pair})`,
  },
  InList: {
    name: "InList",
    category: "data",
    arity: 2,
    icon: "list-check",
    description: "Check if value is in list",
    calculate: (value: any, list: any[]) => Array.isArray(list) && list.includes(value),
    toLatex: (value: string, list: string) => `${value} \\in ${list}`,
  },
  HasKey: {
    name: "HasKey",
    category: "data",
    arity: 2,
    icon: "key-check",
    description: "Check if dictionary has key",
    calculate: (key: any, dict: any) => typeof dict === "object" && dict !== null && key in dict,
    toLatex: (key: string, dict: string) => `${key} \\in \\text{keys}(${dict})`,
  },
};

/**
 * Parse an s-expression formula string into a tree structure
 */
export function parseFormula(formula: string): any {
  const tokens = tokenizeFormula(formula);
  const [ast] = parseTokens(tokens, 0);
  return ast;
}

function tokenizeFormula(formula: string): string[] {
  const tokens: string[] = [];
  let current = "";
  let inString = false;
  let inUnit = false;

  for (let i = 0; i < formula.length; i++) {
    const char = formula[i];

    if (char === "'" && !inUnit) {
      if (inString) {
        tokens.push(current + char);
        current = "";
        inString = false;
      } else {
        if (current.trim()) tokens.push(current.trim());
        current = char;
        inString = true;
      }
    } else if (inString) {
      current += char;
    } else if (char === "(") {
      if (current.trim()) tokens.push(current.trim());
      tokens.push("(");
      current = "";
    } else if (char === ")") {
      if (current.trim()) tokens.push(current.trim());
      tokens.push(")");
      current = "";
    } else if (char === " " || char === "\t" || char === "\n") {
      if (current.trim()) tokens.push(current.trim());
      current = "";
    } else {
      current += char;
    }
  }

  if (current.trim()) tokens.push(current.trim());
  return tokens;
}

function parseTokens(tokens: string[], start: number): [any, number] {
  if (start >= tokens.length) return [null, start];

  const token = tokens[start];

  // Open paren - parse list
  if (token === "(") {
    const list: any[] = [];
    let i = start + 1;
    while (i < tokens.length && tokens[i] !== ")") {
      const [item, newI] = parseTokens(tokens, i);
      list.push(item);
      i = newI;
    }
    return [list, i + 1]; // Skip closing paren
  }

  // Literal value
  return [token, start + 1];
}

/**
 * Convert s-expression AST to LaTeX
 */
export function formulaToLatex(ast: any): string {
  if (typeof ast === "string") {
    // Handle strings, units, variables, and quality keys
    if (ast.startsWith("'") && ast.endsWith("'")) {
      // String literal or unit
      const content = ast.slice(1, -1);
      // Check if it's a unit (contains space and ends with unit symbol)
      if (/\d+\s*[a-zA-Z²³°]+/.test(content)) {
        return `\\text{${content}}`;
      }
      return `\\text{"${content}"}`;
    } else if (ast.startsWith("$")) {
      // Variable
      return `\\textit{${ast.substring(1)}}`;
    } else if (ast.includes(".")) {
      // Quality key
      return `\\mathit{${ast}}`;
    } else if (!isNaN(Number(ast))) {
      // Number
      return ast;
    } else {
      // Other literal
      return `\\text{${ast}}`;
    }
  }

  if (Array.isArray(ast) && ast.length > 0) {
    const functionName = ast[0];
    const operands = ast.slice(1);

    // Check if it's a known function
    const fn = formulaFunctions[functionName];
    if (fn) {
      const operandLatex = operands.map((op) => formulaToLatex(op));
      return fn.toLatex(...operandLatex);
    }

    // Unknown function - render as function call
    const operandLatex = operands.map((op) => formulaToLatex(op));
    return `\\text{${functionName}}(${operandLatex.join(", ")})`;
  }

  return "\\text{?}";
}
