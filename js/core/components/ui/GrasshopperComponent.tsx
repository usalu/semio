import React, { FC, useState } from 'react';

interface ParamProps {
    name: string;
    nickname: string;
    description: string;
    kind: string;
    onMouseEnter?: () => void;
    onMouseLeave?: () => void;
    onClick?: () => void;
    className?: string; // Added className property
}

const Param: FC<ParamProps> = ({ name, nickname, description, kind, onMouseEnter, onMouseLeave, onClick, className }) => {
    return (
        <div
            className={`w-fit h-10 flex items-center justify-center my-1 text-sm cursor-help ${className}`}
            title={`Name: ${name}\nDescription: ${description}\nKind: ${kind}`}
            onMouseEnter={onMouseEnter}
            onMouseLeave={onMouseLeave}
            onClick={onClick}
        >
            <a href={'#' + kind} target="_blank" rel="noopener noreferrer"
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

const GrasshopperComponent: FC<GrasshopperComponentProps> = ({ nickname, inputs, outputs, description }) => {
    const [hoveredParam, setHoveredParam] = useState<string | null>(null);

    return (
        <div className="flex flex-col items-center gap-1">
            <div className="flex flex-row items-start gap-4">
                {/* Input Names Box */}
                <div className="w-fit border border-black rounded-lg p-2 flex flex-col items-start gap-1">
                    <div className="flex flex-col gap-1 items-start">
                        {inputs?.map((input, index) => (
                            <React.Fragment key={index}>
                                <div
                                    className="w-fit h-10 flex items-center justify-center my-1 text-sm cursor-default p-4"
                                >
                                    {input.name}
                                </div>
                                {index < inputs.length - 1 && <div className="w-full h-px bg-gray-400"></div>}
                            </React.Fragment>
                        ))}
                    </div>
                </div>

                {/* Main Component Box */}
                <div className="w-fit border border-black rounded-lg p-2 flex flex-row items-start gap-1">
                    <div className="flex flex-col gap-1 items-start">
                        {inputs?.map((input, index) => (
                            <React.Fragment key={index}>
                                <Param
                                    {...input}
                                    className="p-4"
                                    onMouseEnter={() => setHoveredParam(input.description)}
                                    onMouseLeave={() => setHoveredParam(null)}
                                    onClick={() => setHoveredParam(input.description)}
                                />
                                {index < inputs.length - 1 && <div className="w-full h-px bg-gray-400"></div>}
                            </React.Fragment>
                        ))}
                    </div>
                    <div className="rotate-90 text-center bg-black text-white p-4 rounded-md text-lg font-bold flex items-center justify-center h-full">
                        <p className="transform -translate-x-1/2 -translate-y-1/2">{nickname}</p>
                    </div>
                    <div className="flex flex-col gap-1 items-start">
                        {outputs?.map((output, index) => (
                            <React.Fragment key={index}>
                                <Param
                                    {...output}
                                    className="p-4"
                                    onMouseEnter={() => setHoveredParam(output.description)}
                                    onMouseLeave={() => setHoveredParam(null)}
                                    onClick={() => setHoveredParam(output.description)}
                                />
                                {index < outputs.length - 1 && <div className="w-full h-px bg-gray-400"></div>}
                            </React.Fragment>
                        ))}
                    </div>
                </div>

                {/* Output Names Box */}
                <div className="w-fit border border-black rounded-lg p-2 flex flex-col items-start gap-1">
                    <div className="flex flex-col gap-1 items-start">
                        {outputs?.map((output, index) => (
                            <React.Fragment key={index}>
                                <div
                                    className="w-fit h-10 flex items-center justify-center my-1 text-sm cursor-default p-4"
                                >
                                    {output.name}
                                </div>
                                {index < outputs.length - 1 && <div className="w-full h-px bg-gray-400"></div>}
                            </React.Fragment>
                        ))}
                    </div>
                </div>
            </div>

            <div className="w-fit text-white p-2 rounded-md text-sm text-center mt-1 bg-black">
                {hoveredParam || description}
            </div>
        </div>
    );
};

export default GrasshopperComponent;