import React, { FC, useCallback, useMemo, useState, useEffect } from 'react';


interface GrasshopperComponentProps {
    name: string;
    description: string;
}

const GrasshopperComponent: FC<GrasshopperComponentProps> = ({ name, description }) => {
    return (
        <div>
            <h1>{name}</h1>
            <p>{description}</p>
        </div>
    );
};

export default GrasshopperComponent;