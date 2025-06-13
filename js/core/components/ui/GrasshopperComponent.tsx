import React, { FC, useState } from 'react';

interface ParamProps {
    name: string;
    nickname: string;
    description: string;
    kind: string;
    onMouseEnter?: () => void;
    onMouseLeave?: () => void;
    onClick?: () => void; // Added onClick property
}

const Param: FC<ParamProps> = ({ name, nickname, description, kind, onMouseEnter, onMouseLeave, onClick }) => {
    return (
        <div
            className="w-fit h-10 flex items-center justify-center my-1 border border-black bg-yellow-100 text-sm cursor-help"
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
<<<<<<< HEAD
                <div className="w-fit border border-black rounded-lg p-1 bg-gray-800 flex flex-col items-start gap-1">
=======
                <div className="w-fit border-2 border-black rounded-lg p-5 bg-gray-800 flex flex-col items-start gap-4">
>>>>>>> origin/docs-grasshopper-kinan
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
                </div>

                {/* Main Component Box */}
<<<<<<< HEAD
                <div className="w-fit border border-black rounded-lg p-1 bg-gray-800 flex flex-row items-start gap-1">
=======
                <div className="w-fit border-2 border-black rounded-lg p-5 bg-gray-800 flex flex-row items-start gap-4">
>>>>>>> origin/docs-grasshopper-kinan
                    <div className="flex flex-col gap-1 items-start">
                        {inputs?.map((input, index) => (
                            <Param
                                key={index}
                                {...input}
                                onMouseEnter={() => setHoveredParam(input.description)}
                                onMouseLeave={() => setHoveredParam(null)}
                                onClick={() => setHoveredParam(input.description)}
                            />
                        ))}
                    </div>
<<<<<<< HEAD
                    <div className="rotate-90 text-center relative bg-gray-900 text-white p-1 rounded-md text-lg font-bold flex flex-col items-center justify-center gap-1">
=======
                    <div className="rotate-90 text-center relative bg-gray-900 text-white p-2 rounded-md text-lg font-bold flex flex-col items-center justify-center gap-1">
>>>>>>> origin/docs-grasshopper-kinan
                        <p>{nickname}</p>
                    </div>
                    <div className="flex flex-col gap-1 items-start">
                        {outputs?.map((output, index) => (
                            <Param
                                key={index}
                                {...output}
                                onMouseEnter={() => setHoveredParam(output.description)}
                                onMouseLeave={() => setHoveredParam(null)}
                                onClick={() => setHoveredParam(output.description)}
                            />
                        ))}
                    </div>
                </div>

                {/* Output Names Box */}
<<<<<<< HEAD
                <div className="w-fit border border-black rounded-lg p-1 bg-gray-800 flex flex-col items-start gap-1">
=======
                <div className="w-fit border-2 border-black rounded-lg p-5 bg-gray-800 flex flex-col items-start gap-4">
>>>>>>> origin/docs-grasshopper-kinan
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
            </div>

<<<<<<< HEAD
            <div className="w-fit bg-gray-700 text-white p-1 rounded-md text-sm text-center mt-2">
=======
            <div className="w-fit bg-gray-700 text-white p-3 rounded-md text-sm text-center mt-2">
>>>>>>> origin/docs-grasshopper-kinan
                {hoveredParam || description}
            </div>
        </div>
    );
};

export default GrasshopperComponent;