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
        <div className="param">
            <h2>{name}</h2>
            <p>{description}</p>
            <p>{nickname}</p>
            <p>Kind: {kind}</p>
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

const GrasshopperComponent: FC<GrasshopperComponentProps> = ({ name, nickname, description, inputs, outputs }) => {
    return (
        <div className="grasshopper-component">
            <div className="inputs">
                {inputs?.map((input, index) => (
                    <Param key={index} {...input} />
                ))}
            </div>
            <div className="name-vertical">
                <h1>{name}</h1>
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