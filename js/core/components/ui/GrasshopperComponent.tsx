import React, { FC } from 'react';

interface ParamProps {
    name: string;
    nickname: string;
    description: string;
    kind: string;
}

const Param: FC<ParamProps> = ({ name, nickname, description, kind }) => {
    return (
        <div
            className="w-fit h-10 flex items-center justify-center my-1 border border-black bg-yellow-100 text-sm cursor-help"
            title={`Name: ${name}\nDescription: ${description}\nKind: ${kind}`}
        >
            <p>{nickname}</p>
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
    return (
        <div className="w-fit border-2 border-black rounded-lg p-5 bg-gray-800 flex flex-row items-center justify-between">
            <div className="flex flex-col gap-1 items-start">
                {inputs?.map((input, index) => (
                    <Param key={index} {...input} />
                ))}
            </div>
            <div className="rotate-90 text-center relative bg-gray-900 text-white p-2 rounded-md text-lg font-bold flex flex-col items-center justify-center gap-1">
                <p>{nickname}</p>
            </div>
            <div className="flex flex-col gap-1 items-start">
                {outputs?.map((output, index) => (
                    <Param key={index} {...output} />
                ))}
            </div>
        </div>
    );
};

export default GrasshopperComponent;