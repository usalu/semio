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
            className={`w-fit h-10 flex items-center justify-center my-1 border border-black bg-yellow-100 text-sm cursor-help ${className}`}
            title={`Name: ${name}\nDescription: ${description}\nKind: ${kind}`}
            onMouseEnter={onMouseEnter}
            onMouseLeave={onMouseLeave}
            onClick={onClick}
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

const GrasshopperComponent: FC<GrasshopperComponentProps> = ({ nickname, inputs, outputs, description }) => {
    const [hoveredParam, setHoveredParam] = useState<string | null>(null);

    return (
        <div className="flex flex-col items-center gap-4">
            <div className="flex flex-row items-start gap-4">
                {/* Input Names Box */}
                <div className="w-fit border border-black rounded-lg p-3 bg-gray-800 flex flex-col items-start gap-3">
                    <div className="flex flex-col gap-3 items-start">
                        {inputs?.map((input, index) => (
                            <div
                                key={index}
                                className="w-fit h-10 flex items-center justify-center my-1 border border-black bg-yellow-100 text-sm cursor-default p-4"
                            >
                                {input.name}
                            </div>
                        ))}
                    </div>
                </div>

                {/* Main Component Box */}
                <div className="w-fit border border-black rounded-lg p-3 bg-gray-800 flex flex-row items-start gap-3">
                    <div className="flex flex-col gap-3 items-start">
                        {inputs?.map((input, index) => (
                            <Param
                                key={index}
                                {...input}
                                className="p-4" // Added padding for Param components
                                onMouseEnter={() => setHoveredParam(input.description)}
                                onMouseLeave={() => setHoveredParam(null)}
                                onClick={() => setHoveredParam(input.description)}
                            />
                        ))}
                    </div>
                    <div className="rotate-90 text-center relative bg-gray-900 text-white p-4 rounded-md text-lg font-bold flex flex-col items-center justify-center gap-3">
                        <p>{nickname}</p>
                    </div>
                    <div className="flex flex-col gap-3 items-start">
                        {outputs?.map((output, index) => (
                            <Param
                                key={index}
                                {...output}
                                className="p-4" // Added padding for Param components
                                onMouseEnter={() => setHoveredParam(output.description)}
                                onMouseLeave={() => setHoveredParam(null)}
                                onClick={() => setHoveredParam(output.description)}
                            />
                        ))}
                    </div>
                </div>

                {/* Output Names Box */}
                <div className="w-fit border border-black rounded-lg p-3 bg-gray-800 flex flex-col items-start gap-3">
                    <div className="flex flex-col gap-3 items-start">
                        {outputs?.map((output, index) => (
                            <div
                                key={index}
                                className="w-fit h-10 flex items-center justify-center my-1 border border-black bg-yellow-100 text-sm cursor-default p-4"
                            >
                                {output.name}
                            </div>
                        ))}
                    </div>
                </div>
            </div>

            <div className="w-fit bg-gray-700 text-white p-3 rounded-md text-sm text-center mt-2">
                {hoveredParam || description}
            </div>
        </div>
    );
};

export default GrasshopperComponent;