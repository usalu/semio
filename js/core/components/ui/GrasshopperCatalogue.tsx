import React, { FC, useState } from 'react';
import { Accordion, AccordionContent, AccordionItem, AccordionTrigger } from '@semio/js/components/ui/Accordion';

interface ParamProps {
    name: string;
    nickname: string;
    description: string;
    kind: string;
    expanded?: boolean;
}

const Param: FC<ParamProps> = ({ name, nickname, description, kind, expanded }) => {
    return (
        <AccordionItem value={name} className="size-fit"
        >
            <AccordionTrigger>
                {expanded ? <p>{name}</p> : <p>{nickname}</p>}
            </AccordionTrigger>
            <AccordionContent>
                {description}
            </AccordionContent>
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
        <div className="flex flex-row items-stretch size-fit border-2 border-dark bg-light justify-between">
            <Params params={inputs || []} />
            <div className="flex items-center justify-center min-w-10 bg-dark text-light text-lg font-bold" style={{ writingMode: 'vertical-rl' }}>
                <p className="m-0 p-0 w-full text-center rotate-180">{nickname}</p>
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
