import * as grpc from '@grpc/grpc-js';

import {Representation} from 'semio/model/v1/model'
import {RepresentationConversionRequest} from 'semio/extension/converter/v1/converter'
import {IConverterService,converterServiceDefinition} from 'semio/extension/converter/v1/converter.grpc-server'

// const {Representation } = require('semio/model/v1/model')
// const {RepresentationConversionRequest } = require('semio/extension/converter/v1/converter')
// const {IConverterService, convertRepresentation } = require('semio/extension/converter/v1/converter.grpc-server')

const host = '[::]:5000';

const threeConverterService: IConverterService = {

    convertRepresentation(call: grpc.ServerUnaryCall<RepresentationConversionRequest, Representation>, callback: grpc.sendUnaryData<Representation>): void {
      console.log("Converting representation...")
      callback(null,{name:"HelloRep", lod:BigInt(435)});
    }
}


function getServer(): grpc.Server {
    const server = new grpc.Server();
    server.addService(converterServiceDefinition, threeConverterService);
    return server;
}

// function convertRepresentation(call : grpc.ServerUnaryCall<RepresentationConversionRequest, Representation>,callback: grpc.sendUnaryData<Representation>){
//     console.log("Converting representation...")
//     callback(null,{name:"HelloRep", lod:BigInt(435)})
// }

// function getServer(): grpc.Server {
//     const server = new grpc.Server();
//     server.addService(converterServiceDefinition, {
//         "convertRepresentation": convertRepresentation
//     });
//     return server;
// }


if (require.main === module) {
    const server = getServer();
    server.bindAsync(
        host,
        grpc.ServerCredentials.createInsecure(),
        (err: Error | null, port: number) => {
            if (err) {
                console.error(`Server error: ${err.message}`);
            } else {
                console.log(`Server bound on port: ${port}`);
                server.start();
            }
        }
    );
}
