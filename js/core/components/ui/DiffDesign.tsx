import { FC, ReactNode, useState, useEffect, useReducer } from 'react';
import Sketchpad from "@semio/js/components/ui/Sketchpad";
import { Input } from "@semio/js/components/ui/Input";
import { Design } from '@semio/js';

interface DiffDesignProps {
    initialDesign: Design;
    initialDiff: Design;
}

const DiffDesign: FC<DiffDesignProps> = ({ initialDesign, initialDiff }) => {
    const [design, setDesign] = useState<Design>(initialDesign);
    const [diff, setDiff] = useState<Design>(initialDiff);

    useEffect(() => {
        setDesign(initialDesign);
        setDiff(initialDiff);
    }, [initialDesign, initialDiff]);


    return (
        <>
            <Input value={JSON.stringify(design)} onChange={(e) => setDesign(JSON.parse(e.target.value))} />
            <Input value={JSON.stringify(diff)} onChange={(e) => setDiff(JSON.parse(e.target.value))} />
            <Sketchpad userId="user-test" />
        </>
    )

};

export default DiffDesign;
