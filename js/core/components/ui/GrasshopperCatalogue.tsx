import React, { FC, useState } from 'react';
import { Accordion, AccordionItem } from '@semio/js/components/ui/Accordion';

interface ParamProps {
    name: string;
    nickname: string;
    description: string;
    kind: string;
    expanded?: boolean;
}

const Param: FC<ParamProps> = ({ name, nickname, description, kind, expanded }) => {
    return (
        <AccordionItem
            className="size-fit flex items-center justify-center my-1 text-sm cursor-help"
            title={`Name: ${name}\nDescription: ${description}\nKind: ${kind}`}
        >
            {expanded ? <p>{name}</p> : <p>{nickname}</p>}
        </AccordionItem>
    );
};


interface ParamsProps {
    params?: ParamProps[];
    input?: boolean;
}

const Params: FC<ParamsProps> = ({ params, input }) => {
    const [expanded, setExpanded] = useState(false);

    return (
        <Accordion className="flex flex-col gap-1">
            {params.map((param, index) => (
                <Param key={index} {...param} expanded={expanded} />
            ))}
        </Accordion>
    );
}

interface GrasshopperComponentProps {
    name: string;
    nickname: string;
    description: string;
    inputs?: ParamProps[];
    outputs?: ParamProps[];
}

const GrasshopperComponent: FC<GrasshopperComponentProps> = ({ nickname, inputs, outputs }) => {
    const [expandedOutputs, setExpandedOutputs] = useState(false);
    return (
        <div className="size-fit border-2 border-dark p-1 bg-light flex flex-row items-center justify-between">
            <Params params={inputs || []} />
            <div className="rotate-270 text-center relative bg-dark text-light p-2 text-lg font-bold flex flex-col items-center justify-center gap-1">
                <p>{nickname}</p>
            </div>
            <Params params={outputs || []} input={false} />
        </div>
    );
};

interface GrasshopperCatalogueProps {
    components: GrasshopperComponentProps[];
}

const GrasshopperCatalogue: FC<GrasshopperCatalogueProps> = ({ components }) => {
    return (
        <div className="flex flex-col gap-4">
            {components.map((component, index) => (
                <GrasshopperComponent key={index} {...component} />
            ))}
        </div>
    );
};

export default GrasshopperCatalogue;
