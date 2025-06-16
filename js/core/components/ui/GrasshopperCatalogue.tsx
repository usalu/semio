import React, { FC, useState } from 'react';
import { Accordion, AccordionContent, AccordionItem, AccordionTrigger } from '@semio/js/components/ui/Accordion';

interface ParamProps {
    name: string;
    nickname: string;
    description: string;
    kind: string;
    extended?: boolean;
    setDescription?: (description: string) => void;
}

const Param: FC<ParamProps> = ({ name, nickname, description, kind, extended, setDescription }) => {
    return (
        <div
            className="size-fit"
            onClick={(e) => {
                if (extended && setDescription) {
                    e.stopPropagation();
                    setDescription(description);
                }
            }}
        >
            {extended ? <p>{name}</p> : <p>{nickname}</p>}
        </div>
    );
};


interface ParamsProps {
    params?: ParamProps[];
    input?: boolean;
    setDescription?: (description: string) => void;
    extended?: boolean;
}

const Params: FC<ParamsProps> = ({ params, input, setDescription, extended }) => {
    return (
        <div className="flex flex-col gap-1"
        >
            {params.map((param, index) => (
                <Param key={index} {...param} extended={extended} setDescription={setDescription} />
            ))}
        </div >
    );
}

interface GrasshopperComponentProps {
    name: string;
    nickname: string;
    inputs?: ParamProps[];
    outputs?: ParamProps[];
}

const GrasshopperComponent: FC<GrasshopperComponentProps> = ({ name, nickname, inputs, outputs }) => {
    const [description, setDescription] = useState('A description of the component can go here…');
    const [extended, setExtended] = useState(false);
    return (
        <div id="extended-component" className=""
            onDoubleClick={
                () => setExtended(!extended)
            }>
            <div id="component" className="flex flex-row items-stretch size-fit border-2 border-dark bg-light justify-between relative">
                <Params params={inputs || []} setDescription={setDescription} extended={extended} />
                <div className="flex items-center justify-center min-w-10 bg-dark text-light text-lg font-bold" style={{ writingMode: 'vertical-rl' }}>
                    <p className="m-0 p-0 w-full text-center rotate-180">{extended ? name : nickname}</p>
                </div>
                <Params params={outputs || []} input={false} setDescription={setDescription} extended={extended} />
                {extended &&
                    <div id="description" className="bg-gray w-full absolute left-0 top-full mt-1 p-2">
                        {description}
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
