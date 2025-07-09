// #region Header

// GrasshopperCatalogue.tsx

// 2025 Ueli Saluz
// 2025 Kinan Sarakbi

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

import { Tabs, TabsContent, TabsList, TabsTrigger } from '@semio/js/components/ui/Tabs'
import { FC, useLayoutEffect, useRef, useState } from 'react'
// import { default as Components } from '../../../../assets/grasshopper/components.json'

interface ParamProps {
  name: string
  nickname: string
  description: string
  kind: string
  extended?: boolean
  highlight?: boolean
  setHighlight?: (name: string, description: string) => void
}

const Param: FC<ParamProps> = ({ name, nickname, description, kind, extended, highlight, setHighlight }) => {
  return (
    <div
      className={`${extended ? 'p-1' : ''} ${extended && highlight ? 'bg-primary' : ''}`}
      onClick={(e) => {
        if (extended && setHighlight) {
          e.stopPropagation()
          setHighlight(name, description)
        }
      }}
    >
      <p className="whitespace-nowrap">{extended ? name : nickname}</p>
    </div>
  )
}

interface ParamsProps {
  params: ParamProps[]
  input: boolean
  highlight: number | undefined
  setHighlight: (highlight: Highlight) => void
  extended?: boolean
}

const Params: FC<ParamsProps> = ({ params, input, highlight, setHighlight, extended }) => {
  const setHighlightHandler = (name: string, description: string) => {
    setHighlight({ input, index: params.findIndex((param) => param.name === name), description })
  }
  return (
    <div className="flex flex-col justify-center">
      {params.map((param, index) => (
        <Param
          key={index}
          {...param}
          extended={extended}
          setHighlight={setHighlightHandler}
          highlight={highlight === index}
        />
      ))}
    </div>
  )
}

interface Highlight {
  input: boolean
  index: number
  description: string
}

interface GrasshopperComponentProps {
  name: string
  nickname: string
  description: string
  icon: string
  inputs?: ParamProps[]
  outputs?: ParamProps[]
}

const GrasshopperComponent: FC<GrasshopperComponentProps> = ({ name, nickname, description, inputs, outputs }) => {
  const [highlight, setHighlight] = useState<Highlight | undefined>(undefined)
  const [extended, setExtended] = useState(false)
  const [coreWidth, setCoreWidth] = useState<number | undefined>(undefined)
  const coreRef = useRef<HTMLDivElement>(null)

  useLayoutEffect(() => {
    if (coreRef.current) {
      setCoreWidth(coreRef.current.offsetWidth)
    }
  }, [inputs, outputs, extended])

  const setHighlightHandler = (hl: Highlight) => {
    if (highlight?.input === hl.input && highlight?.index === hl.index) {
      setHighlight(undefined)
    } else {
      setHighlight(hl)
    }
  }

  return (
    <div
      id="extended-component"
      className="flex flex-col"
      style={{ width: coreWidth ? `${coreWidth}px` : 'auto' }}
      onClick={() => setExtended(!extended)}
    >
      <div
        ref={coreRef}
        id="core"
        className="flex flex-row items-stretch size-fit border-2 border-dark bg-light justify-between"
      >
        {inputs && (
          <Params
            params={inputs || []}
            input={true}
            setHighlight={setHighlightHandler}
            extended={extended}
            highlight={highlight?.input ? highlight?.index : undefined}
          />
        )}
        <div className="bg-dark text-light font-bold" style={{ writingMode: 'vertical-rl' }}>
          <p className="w-full text-center rotate-180">{extended ? name : nickname}</p>
        </div>
        {outputs && (
          <Params
            params={outputs || []}
            input={false}
            setHighlight={setHighlightHandler}
            extended={extended}
            highlight={highlight?.input ? undefined : highlight?.index}
          />
        )}
      </div>
      {extended && (
        <div id="description" className="bg-gray p-1 text-sm">
          {highlight?.description || description}
        </div>
      )}
    </div>
  )
}

interface GrasshopperCatalogueProps {
  [group: string]: {
    [exposure: number]: GrasshopperComponentProps[];
  };
};

const GrasshopperCatalogue: FC<GrasshopperCatalogueProps> = ({ props }) => {
  // Sort group names alphabetically
  const groups = Object.keys(props).sort()
  return (
    <Tabs className="w-full h-full" defaultValue={groups[0]}>
      <TabsList>
        {groups.map((group) => (
          <TabsTrigger key={group} value={group}>
            {group}
          </TabsTrigger>
        ))}
      </TabsList>
      {groups.map((group) => {
        const exposures = Object.keys(props[group]).sort((a, b) => Number(a) - Number(b))
        return (
          <TabsContent key={group} value={group}>
            <Tabs className="w-full h-full" defaultValue={exposures[0]}>
              <TabsList>
                {exposures.map((exposure) => (
                  <TabsTrigger key={exposure} value={exposure}>
                    {exposure}
                  </TabsTrigger>
                ))}
              </TabsList>
              {exposures.map((exposure) => (
                <TabsContent key={exposure} value={exposure}>
                  <div className="flex flex-row flex-wrap gap-2 p-2">
                    {(props[group][Number(exposure)] as GrasshopperComponentProps[]).map(
                      (component: GrasshopperComponentProps, idx: number) => (
                        <GrasshopperComponent key={idx} {...component} />
                      )
                    )}
                  </div>
                </TabsContent>
              ))}
            </Tabs>
          </TabsContent>
        )
      })}
    </Tabs>
  )
}

export default GrasshopperCatalogue
