import React, { FC, useState } from 'react';
import { Accordion, AccordionContent, AccordionItem, AccordionTrigger } from '@semio/js/components/ui/Accordion';

interface ParamProps {
    name: string;
    nickname: string;
    description: string;
    kind: string;
    extended?: boolean;
    highlight?: boolean;
    setHighlight?: (name: string, description: string) => void;
}

const Param: FC<ParamProps> = ({ name, nickname, description, kind, extended, highlight, setHighlight }) => {
    return (
        <div
            className={`size-fit ${extended && highlight ? 'bg-primary' : ''}`}
            onClick={(e) => {
                if (extended && setHighlight) {
                    e.stopPropagation();
                    setHighlight(name, description);
                }
            }}
        >
            {extended ? <p>{name}</p> : <p>{nickname}</p>}
        </div>
    );
};


interface ParamsProps {
    params: ParamProps[];
    input: boolean;
    highlight: number | undefined;
    setHighlight: (highlight: Highlight) => void;
    extended?: boolean;
}

const Params: FC<ParamsProps> = ({ params, input, highlight, setHighlight, extended }) => {
    const setHighlightHandler = (name: string, description: string) => {
        setHighlight({ input, index: params.findIndex(param => param.name === name), description })
    };
    return (
        <div className="flex flex-col"
        >
            {params.map((param, index) => (
                <Param key={index} {...param} extended={extended} setHighlight={setHighlightHandler} highlight={highlight === index} />
            ))}
        </div >
    );
}

interface Highlight {
    input: boolean;
    index: number;
    description: string;
}

interface GrasshopperComponentProps {
    name: string;
    nickname: string;
    description: string;
    inputs?: ParamProps[];
    outputs?: ParamProps[];
}

const GrasshopperComponent: FC<GrasshopperComponentProps> = ({ name, nickname, description, inputs, outputs }) => {
    const [highlight, setHighlight] = useState<Highlight | undefined>(undefined);
    const [extended, setExtended] = useState(false);
    return (
        <div id="extended-component" className=""
            onClick={
                () => setExtended(!extended)
            }>
            <div id="component" className="flex flex-row items-stretch size-fit border-2 border-dark bg-light justify-between relative">
                {inputs && <Params params={inputs || []} input={true} setHighlight={setHighlight} extended={extended} highlight={highlight?.input ? highlight?.index : undefined} />}
                <div className="flex items-center justify-center bg-dark text-light text-lg font-bold" style={{ writingMode: 'vertical-rl' }}>
                    <p className="w-full text-center rotate-180">{extended ? name : nickname}</p>
                </div>
                {outputs && <Params params={outputs || []} input={false} setHighlight={setHighlight} extended={extended} highlight={highlight?.input ? undefined : highlight?.index} />}
                {extended &&
                    <div id="description" className="bg-gray w-full absolute left-0 top-full mt-1 p-1">
                        {highlight?.description}
                    </div>
                }
            </div>
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
