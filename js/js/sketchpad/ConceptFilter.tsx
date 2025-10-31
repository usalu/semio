// #region Header

// ConceptFilter.tsx

// 2025 Ueli Saluz

// Horizontal scrollable concept filter for filtering artifacts by concepts

// #endregion

import { FC, useMemo } from "react";
import { useSearchParams } from "react-router";
import { Toggle } from "../elements/input/Toggle";

interface ConceptFilterProps {
    allConcepts: string[];
    onConceptsChange?: (concepts: string[]) => void;
    paramName?: string;
}

export const ConceptFilter: FC<ConceptFilterProps> = ({ allConcepts, onConceptsChange, paramName = "concepts" }) => {
    const [searchParams, setSearchParams] = useSearchParams();

    const selectedConcepts = useMemo(() => {
        if (paramName === "c") {
            return searchParams.getAll("c");
        }
        const conceptsParam = searchParams.get(paramName);
        return conceptsParam ? conceptsParam.split(",").filter(Boolean) : [];
    }, [searchParams, paramName]);

    const toggleConcept = (concept: string) => {
        const newConcepts = selectedConcepts.includes(concept) ? selectedConcepts.filter((c) => c !== concept) : [...selectedConcepts, concept];

        const newParams = new URLSearchParams(searchParams);
        if (paramName === "c") {
            newParams.delete("c");
            newConcepts.forEach((c) => newParams.append("c", c));
        } else {
            if (newConcepts.length > 0) {
                newParams.set(paramName, newConcepts.join(","));
            } else {
                newParams.delete(paramName);
            }
        }
        setSearchParams(newParams);
        onConceptsChange?.(newConcepts);
    };

    if (allConcepts.length === 0) return null;

    return (
        <div className="border-b overflow-x-auto">
            <div className="flex gap-1 p-1 min-w-min">
                {allConcepts.map((concept) => (
                    <Toggle key={concept} id={`concept-${concept}`} pressed={selectedConcepts.includes(concept)} onPressedChange={() => toggleConcept(concept)} className="whitespace-nowrap">
                        {concept}
                    </Toggle>
                ))}
            </div>
        </div>
    );
};
