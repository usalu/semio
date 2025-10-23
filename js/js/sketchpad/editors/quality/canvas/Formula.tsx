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

declare global {
  interface Window {
    MathJax?: any;
  }
}

const Formula: FC = () => {
  const quality = useQuality() as Quality | undefined;
  const mathRef = useRef<HTMLDivElement>(null);

  const formulaToLatex = (formula?: string): string => {
    if (!formula) return "\\text{No formula defined}";
    return `\\text{${formula}}`;
  };

  useEffect(() => {
    const loadMathJax = () => {
      if (window.MathJax) {
        if (mathRef.current) {
          window.MathJax.typesetPromise([mathRef.current]).catch((err: any) => console.error("[ORIGIN] MathJax typeset error:", err));
        }
        return;
      }
      const script = document.createElement("script");
      script.src = "https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js";
      script.async = true;
      script.onload = () => {
        if (window.MathJax && mathRef.current) {
          window.MathJax.typesetPromise([mathRef.current]).catch((err: any) => console.error("[ORIGIN] MathJax typeset error:", err));
        }
      };
      document.head.appendChild(script);
    };
    loadMathJax();
  }, []);

  useEffect(() => {
    if (window.MathJax && mathRef.current) {
      window.MathJax.typesetPromise([mathRef.current]).catch((err: any) => console.error("[ORIGIN] MathJax typeset error:", err));
    }
  }, [quality?.formula]);

  return (
    <div className="h-full w-full border-b border-foreground bg-base flex items-center justify-center overflow-auto">
      <div ref={mathRef} className="text-foreground p-4" style={{ fontSize: "1.5rem" }}>
        {`\\[${formulaToLatex(quality?.formula)}\\]`}
      </div>
    </div>
  );
};

export default Formula;
