import React, { FC } from 'react';
import './GrasshopperComponent.css';

interface ParamProps {
    name: string;
    nickname: string;
    description: string;
    kind: string;
}

const Param: FC<ParamProps> = ({ name, nickname, description, kind }) => {
    return (
        <div className="param" title={`Name: ${name}\nDescription: ${description}\nKind: ${kind}`}>
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
        <div className="grasshopper-component">
            <div className="inputs">
                {inputs?.map((input, index) => (
                    <Param key={index} {...input} />
                ))}
            </div>
            <div className="name-vertical">
                <p>{nickname}</p>
            </div>
            <div className="outputs">
                {outputs?.map((output, index) => (
                    <Param key={index} {...output} />
                ))}
            </div>
        </div>
    );
};

export default GrasshopperComponent;