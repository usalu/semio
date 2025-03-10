import * as React from 'react'
import Diagram from '@semio/core/components/ui/Diagram'

const IndexPage = () => {
    return (
        <main>
            <div className='h-100vh w-100vw'>
                <>Hallo</>
                <Diagram />
            </div>
        </main>
    )
}

export const Head = () => <title>Home Page</title>

export default IndexPage