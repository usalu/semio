import React, { FC, useLayoutEffect, useRef, useState } from 'react';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@semio/js/components/ui/Tabs';

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
            className={`${extended ? 'p-1' : ''} ${extended && highlight ? 'bg-primary' : ''}`}
            onClick={(e) => {
                if (extended && setHighlight) {
                    e.stopPropagation();
                    setHighlight(name, description);
                }
            }}
        >
            <p className="whitespace-nowrap">{extended ? name : nickname}</p>
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
        <div className="flex flex-col justify-center"
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
    icon: string;
    inputs?: ParamProps[];
    outputs?: ParamProps[];
}

const GrasshopperComponent: FC<GrasshopperComponentProps> = ({ name, nickname, description, inputs, outputs }) => {
    const [highlight, setHighlight] = useState<Highlight | undefined>(undefined);
    const [extended, setExtended] = useState(false);
    const [coreWidth, setCoreWidth] = useState<number | undefined>(undefined);
    const coreRef = useRef<HTMLDivElement>(null);

    useLayoutEffect(() => {
        if (coreRef.current) {
            setCoreWidth(coreRef.current.offsetWidth);
        }
    }, [inputs, outputs, extended]);

    const setHighlightHandler = (hl: Highlight) => {
        if (highlight?.input === hl.input && highlight?.index === hl.index) {
            setHighlight(undefined);
        }
        else {
            setHighlight(hl);
        }
    }

    return (
        <div
            id="extended-component"
            className="flex flex-col"
            style={{ width: coreWidth ? `${coreWidth}px` : 'auto' }}
            onClick={() => setExtended(!extended)}
        >
            <div ref={coreRef} id="core" className="flex flex-row items-stretch size-fit border-2 border-dark bg-light justify-between">
                {inputs && <Params params={inputs || []} input={true} setHighlight={setHighlightHandler} extended={extended} highlight={highlight?.input ? highlight?.index : undefined} />}
                <div className="bg-dark text-light text-lg font-bold" style={{ writingMode: 'vertical-rl' }}>
                    <p className="w-full text-center rotate-180">{extended ? name : nickname}</p>
                </div>
                {outputs && <Params params={outputs || []} input={false} setHighlight={setHighlightHandler} extended={extended} highlight={highlight?.input ? undefined : highlight?.index} />}
            </div>
            {extended &&
                <div id="description" className="bg-gray p-1">
                    {highlight?.description || description}
                </div>
            }
        </div>
    );
};

interface GrasshopperCatalogueProps {
    components: GrasshopperComponentProps[];
}

const GrasshopperCatalogue: FC<GrasshopperCatalogueProps> = ({ components }) => {
    return (
        // <Tabs className="w-full h-full">
        //     <TabsList className="">
        //         {components.map((component, index) => (
        //             <TabsTrigger key={index} value={component.name}>{component.icon}</TabsTrigger>
        //         ))}
        //     </TabsList>
        //     {components.map((component, index) => (
        //         <TabsContent key={index} value={component.name}><GrasshopperComponent key={index} {...component} /></TabsContent>
        //     ))}
        // </Tabs>
        <div className="flex flex-row flex-wrap gap-2 p-2 ">
            {
                components.map((component, index) => (
                    <GrasshopperComponent key={index} {...component} />
                ))
            }
        </div>
    );
};

export default GrasshopperCatalogue;
