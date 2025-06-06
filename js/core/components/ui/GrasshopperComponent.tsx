import React, { FC, useCallback, useMemo, useState, useEffect } from 'react';



interface ParamProps {
    name: string;
    nickname: string;
    description: string;
    kind: string;
}


const Param: FC<ParamProps> = ({ name, nickname, description, kind }) => {
    return (
        <div>
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

const GrasshopperComponent: FC<GrasshopperComponentProps> = ({ name,nickname,description,inputs,outputs }) => {
    return (
        <div>
            <h1>{name}</h1>
            <p>{description}</p>
            <p>{nickname}</p>

        </div>
    );
};

export default GrasshopperComponent;