// #region Header

// DiffDesign.tsx

// 2025 Ueli Saluz

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion
import { Design } from '@semio/js';
import { Input } from "@semio/js/components/ui/Input";
import Sketchpad from "@semio/js/components/ui/Sketchpad";
import { FC, useEffect, useState } from 'react';

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
