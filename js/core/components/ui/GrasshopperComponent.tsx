import React, { FC, useState } from 'react';

interface ParamProps {
    name: string;
    nickname: string;
    description: string;
    kind: string;
    onMouseEnter?: () => void;
    onMouseLeave?: () => void;
}

const Param: FC<ParamProps> = ({ name, nickname, description, kind, onMouseEnter, onMouseLeave }) => {
    return (
        <div
            className="w-fit h-10 flex items-center justify-center my-1 border border-black bg-yellow-100 text-sm cursor-help"
            title={`Name: ${name}\nDescription: ${description}\nKind: ${kind}`}
            onMouseEnter={onMouseEnter}
            onMouseLeave={onMouseLeave}
        >
            <a href={'#'+ kind} target="_blank" rel="noopener noreferrer"
                className="text-blue-600 hover:text-blue-800">
                {nickname}
                </a>
        </div>
    );
};

interface GrasshopperComponentProps {
    name: string;
    nickname: string;
    description: string;
    inputs?: ParamProps[];
    outputs?: ParamProps[];
}

const GrasshopperComponent: FC<GrasshopperComponentProps> = ({ nickname, inputs, outputs }) => {
    const [hoveredParam, setHoveredParam] = useState<string | null>(null);

    return (
        <div className="flex flex-col items-center gap-4">
            <div className="flex flex-row items-start gap-4">
                <div className="flex flex-col gap-1 items-start">
                    {inputs?.map((input, index) => (
                        <div
                            key={index}
                            className="w-fit h-10 flex items-center justify-center my-1 border border-black bg-yellow-100 text-sm cursor-default"
                        >
                            {input.name}
                        </div>
                    ))}
                </div>
                <div className="w-fit border-2 border-black rounded-lg p-5 bg-gray-800 flex flex-row items-start gap-4">
                    <div className="flex flex-col gap-1 items-start">
                        {inputs?.map((input, index) => (
                            <Param
                                key={index}
                                {...input}
                                onMouseEnter={() => setHoveredParam(input.description)}
                                onMouseLeave={() => setHoveredParam(null)}
                            />
                        ))}
                    </div>
                    <div className="rotate-90 text-center relative bg-gray-900 text-white p-2 rounded-md text-lg font-bold flex flex-col items-center justify-center gap-1">
                        <p>{nickname}</p>
                    </div>
                    <div className="flex flex-col gap-1 items-start">
                        {outputs?.map((output, index) => (
                            <Param
                                key={index}
                                {...output}
                                onMouseEnter={() => setHoveredParam(output.description)}
                                onMouseLeave={() => setHoveredParam(null)}
                            />
                        ))}
                    </div>
                </div>
                <div className="flex flex-col gap-1 items-start">
                    {outputs?.map((output, index) => (
                        <div
                            key={index}
                            className="w-fit h-10 flex items-center justify-center my-1 border border-black bg-yellow-100 text-sm cursor-default"
                        >
                            {output.name}
                        </div>
                    ))}
                </div>
            </div>
            {hoveredParam && (
                <div className="w-fit bg-gray-700 text-white p-3 rounded-md text-sm text-center mt-2">
                    {hoveredParam}
                </div>
            )}
        </div>
    );
};

export default GrasshopperComponent;