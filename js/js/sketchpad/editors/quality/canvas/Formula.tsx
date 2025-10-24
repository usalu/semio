// #region Header

// Formula.tsx

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

import { FC, useEffect, useRef } from "react";
import { Quality } from "../../../../semio";
import { useQuality } from "../../../kits/store";
import { parseFormula, formulaToLatex as convertToLatex } from "../functions";

declare global {
  interface Window {
    MathJax?: any;
  }
}

const Formula: FC = () => {
  const quality = useQuality(undefined, undefined, true) as Quality | undefined;
  const mathRef = useRef<HTMLDivElement>(null);

  const formulaToLatex = (formula?: string): string => {
    if (!formula) return "\\text{No formula defined}";

    try {
      // Parse the s-expression formula
      const ast = parseFormula(formula);
      // Convert to LaTeX
      return convertToLatex(ast);
    } catch (error) {
      console.error("[ORIGIN] Error parsing formula:", error);
      // Fallback to displaying raw formula
      return `\\text{${formula}}`;
    }
  };

  useEffect(() => {
    const loadMathJax = () => {
      if (window.MathJax) {
        console.log("[ORIGIN] MathJax already loaded");
        if (mathRef.current) {
          window.MathJax.typesetPromise([mathRef.current]).catch((err: any) => console.error("[ORIGIN] MathJax typeset error:", err));
        }
        return;
      }
      console.log("[ORIGIN] Loading MathJax...");
      const script = document.createElement("script");
      script.src = "https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js";
      script.async = true;
      script.onload = () => {
        console.log("[ORIGIN] MathJax loaded successfully");
        if (window.MathJax && mathRef.current) {
          window.MathJax.typesetPromise([mathRef.current]).catch((err: any) => console.error("[ORIGIN] MathJax typeset error:", err));
        }
      };
      script.onerror = () => {
        console.error("[ORIGIN] Failed to load MathJax script");
      };
      document.head.appendChild(script);
    };
    loadMathJax();
  }, []);

  useEffect(() => {
    if (window.MathJax && mathRef.current) {
      console.log("[ORIGIN] Rendering formula:", quality?.formula);
      window.MathJax.typesetPromise([mathRef.current]).catch((err: any) => console.error("[ORIGIN] MathJax typeset error:", err));
    }
  }, [quality?.formula]);

  return (
    <div className="h-full w-full border-b border-foreground bg-base flex items-center justify-center overflow-auto">
      <div ref={mathRef} className="text-foreground p-4" style={{ fontSize: "1.5rem" }}>
        {(() => {
          const latex = formulaToLatex(quality?.formula);
          console.log("[ORIGIN] Generated LaTeX:", latex);
          return `\\[${latex}\\]`;
        })()}
      </div>
    </div>
  );
};

export default Formula;
